#include "mpu6050.h"
#include "driver/i2c_master.h"
#include "driver/i2c_types.h"
#include "esp_err.h"
#include "esp_log.h"
#include "esp_timer.h"
#include "freertos/idf_additions.h"
#include "freertos/projdefs.h"
#include <assert.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

static const char* TAG = "mpu6050";

#define CHECK(EXPR)                                                            \
    do {                                                                       \
        esp_err_t status = (EXPR);                                             \
        if (status != ESP_OK) {                                                \
            return status;                                                     \
        }                                                                      \
    } while (0)

#define DEFAULT_TIMEOUT 1000

typedef enum : uint8_t {
    REG_SMPLRT_DIV = 0x19,
    REG_CONFIG = 0x1a,
    REG_GYRO_CONFIG = 0x1b,
    REG_ACCEL_CONFIG = 0x1c,
    REG_FIFO_EN = 0x23,
    REG_ACCEL_XOUT_H = 0x3b,
    REG_ACCEL_XOUT_L = 0x3c,
    REG_ACCEL_YOUT_H = 0x3d,
    REG_ACCEL_YOUT_L = 0x3e,
    REG_ACCEL_ZOUT_H = 0x3f,
    REG_ACCEL_ZOUT_L = 0x40,
    REG_TEMP_OUT_H = 0x41,
    REG_TEMP_OUT_L = 0x42,
    REG_GYRO_XOUT_H = 0x43,
    REG_GYRO_XOUT_L = 0x44,
    REG_GYRO_YOUT_H = 0x45,
    REG_GYRO_YOUT_L = 0x46,
    REG_GYRO_ZOUT_H = 0x47,
    REG_GYRO_ZOUT_L = 0x48,
    REG_SIGNAL_PATH_RESET = 0x68,
    REG_USER_CTRL = 0x6a,
    REG_PWR_MGMT_1 = 0x6b,
    REG_PWR_MGMT_2 = 0x6c,
    REG_FIFO_COUNTH = 0x72,
    REG_FIFO_COUNTL = 0x73,
    REG_FIFO_R_W = 0x74,
} Reg;

typedef enum : uint8_t {
    BIT_CONFIG_DLPF_CFG = 0,
    BIT_GYRO_CONFIG_FS_SEL = 3,
    BIT_ACCEL_CONFIG_AFS_SEL = 3,
    BIT_PWR_MGMT_1_CLKSEL = 0,
    BIT_PWR_MGMT_1_SLEEP = 6,
} Bit;

typedef enum : uint8_t {
    MASK_CONFIG_DLPF_CFG = 0x7,
    MASK_GYRO_CONFIG_FS_SEL = 0x3,
    MASK_ACCEL_CONFIG_AFS_SEL = 0x3,
    MASK_PWR_MGMT_1_CLKSEL = 0x7,
} Mask;

static esp_err_t read_regs(
    Mpu6050* dev, void* out_data, size_t data_size, Reg reg)
{
    return i2c_master_transmit_receive(
        dev->i2c_dev, &reg, 1, out_data, data_size, DEFAULT_TIMEOUT);
}

static esp_err_t read_reg(Mpu6050* dev, uint8_t* out_value, Reg reg)
{
    return i2c_master_transmit_receive(
        dev->i2c_dev, &reg, 1, out_value, 1, DEFAULT_TIMEOUT);
}

static esp_err_t write_reg(Mpu6050* dev, Reg reg, uint8_t value)
{
    uint8_t buffer[] = { reg, value };
    return i2c_master_transmit(dev->i2c_dev, buffer, 2, DEFAULT_TIMEOUT);
}

static esp_err_t read_bits(
    Mpu6050* dev, uint8_t* out_value, Reg reg, Bit offset, Mask mask)
{
    uint8_t buffer;
    CHECK(read_reg(dev, &buffer, reg));
    *out_value = buffer >> offset & mask;
    return ESP_OK;
}

static esp_err_t write_bits(
    Mpu6050* dev, Reg reg, Bit offset, Mask mask, uint8_t value)
{
    uint8_t buffer;
    CHECK(read_reg(dev, &buffer, reg));
    buffer &= ~mask << offset;
    buffer |= value & mask << offset;
    CHECK(write_reg(dev, reg, buffer));
    return ESP_OK;
}

esp_err_t mpu6050_init(Mpu6050* dev)
{
    *dev = (Mpu6050) { 0 };

    i2c_master_bus_config_t bus_config = {
        .clk_source = I2C_CLK_SRC_DEFAULT,
        .i2c_port = I2C_NUM_0,
        .sda_io_num = 11,
        .scl_io_num = 12,
        .glitch_ignore_cnt = 7,
        .flags.enable_internal_pullup = true,
    };
    CHECK(i2c_new_master_bus(&bus_config, &dev->i2c_bus));
    i2c_device_config_t dev_config = {
        .dev_addr_length = I2C_ADDR_BIT_LEN_7,
        .device_address = 0x68,
        .scl_speed_hz = 400000,
    };
    CHECK(i2c_master_bus_add_device(dev->i2c_bus, &dev_config, &dev->i2c_dev));

    const int max_attempts = 5;
    esp_err_t probe_res;
    for (int i = 0; i < max_attempts; ++i) {
        probe_res = i2c_master_probe(dev->i2c_bus, 0x68, DEFAULT_TIMEOUT);
        if (probe_res == ESP_OK)
            break;
        ESP_LOGW(
            TAG, "Device not found. Retrying (%d/%d)", i + 1, max_attempts);
        vTaskDelay(pdMS_TO_TICKS(1000));
    }
    if (probe_res != ESP_OK) {
        ESP_LOGE(TAG, "Device not found.");
        return ESP_ERR_NOT_FOUND;
    }
    ESP_LOGI(TAG, "Found MPU6050 device");

    dev->gyro_range = MPU6050_GYRO_RANGE_250;
    dev->accel_range = MPU6050_ACCEL_RANGE_2;

    vTaskDelay(pdMS_TO_TICKS(250));

    CHECK(mpu6050_set_clock_source(dev, MPU6050_CLKSEL_PLL_GYRO_X_REF));

    CHECK(mpu6050_set_sleep_enabled(dev, false));

    CHECK(mpu6050_set_sample_rate_div(dev, 3));

    return ESP_OK;
}

esp_err_t mpu6050_get_rotation(Mpu6050* dev, float3* rotation)
{
    static const float resolution[] = {
        [MPU6050_GYRO_RANGE_250] = 1.0f / 131.0f,
        [MPU6050_GYRO_RANGE_500] = 1.0f / 65.5f,
        [MPU6050_GYRO_RANGE_1000] = 1.0f / 32.8f,
        [MPU6050_GYRO_RANGE_2000] = 1.0f / 16.4f,
    };

    uint8_t buffer[6];
    CHECK(read_regs(dev, buffer, 6, REG_GYRO_XOUT_H));

    rotation->x
        = (int16_t)(buffer[0] << 8 | buffer[1]) * resolution[dev->gyro_range]
        - dev->rotation_bias.x;
    rotation->y
        = (int16_t)(buffer[2] << 8 | buffer[3]) * resolution[dev->gyro_range]
        - dev->rotation_bias.y;
    rotation->z
        = (int16_t)(buffer[4] << 8 | buffer[5]) * resolution[dev->gyro_range]
        - dev->rotation_bias.z;

    return ESP_OK;
}

esp_err_t mpu6050_get_acceleration(Mpu6050* dev, float3* accel)
{
    static const float resolution[] = {
        [MPU6050_ACCEL_RANGE_2] = 1.0f / 16384.0f,
        [MPU6050_ACCEL_RANGE_4] = 1.0f / 1892.0f,
        [MPU6050_ACCEL_RANGE_8] = 1.0f / 4096.0f,
        [MPU6050_ACCEL_RANGE_16] = 1.0f / 2048.0f,
    };

    uint8_t buffer[6];
    CHECK(read_regs(dev, buffer, 6, REG_ACCEL_XOUT_H));

    accel->x
        = (int16_t)(buffer[0] << 8 | buffer[1]) * resolution[dev->accel_range]
        - dev->accel_bias.x;
    accel->y
        = (int16_t)(buffer[2] << 8 | buffer[3]) * resolution[dev->accel_range]
        - dev->accel_bias.y;
    accel->z
        = (int16_t)(buffer[4] << 8 | buffer[5]) * resolution[dev->accel_range]
        - dev->accel_bias.z;

    return ESP_OK;
}

esp_err_t mpu6050_read_temperature(Mpu6050* dev, float* temperature)
{

    uint8_t buffer[2];
    CHECK(read_regs(dev, buffer, 2, REG_TEMP_OUT_H));

    *temperature = (int16_t)(buffer[0] << 8 | buffer[1]) / 340.0f + 36.53f;

    return ESP_OK;
}

esp_err_t mpu6050_deinit(Mpu6050* dev)
{
    CHECK(i2c_master_bus_rm_device(dev->i2c_dev));
    CHECK(i2c_del_master_bus(dev->i2c_bus));
    return ESP_OK;
}

esp_err_t mpu6050_set_clock_source(Mpu6050* dev, Mpu6050_ClockSource source)
{
    return write_bits(dev,
        REG_PWR_MGMT_1,
        BIT_PWR_MGMT_1_CLKSEL,
        MASK_PWR_MGMT_1_CLKSEL,
        source);
}

esp_err_t mpu6050_set_gyro_range(Mpu6050* dev, Mpu6050_GyroRange range)
{
    CHECK(write_bits(dev,
        REG_GYRO_CONFIG,
        BIT_GYRO_CONFIG_FS_SEL,
        MASK_GYRO_CONFIG_FS_SEL,
        range));
    dev->gyro_range = range;
    return ESP_OK;
}

esp_err_t mpu6050_set_accel_range(Mpu6050* dev, Mpu6050_AccelRange range)
{
    CHECK(write_bits(dev,
        REG_ACCEL_CONFIG,
        BIT_ACCEL_CONFIG_AFS_SEL,
        MASK_ACCEL_CONFIG_AFS_SEL,
        range));
    dev->accel_range = range;
    return ESP_OK;
}

esp_err_t mpu6050_set_sleep_enabled(Mpu6050* dev, bool enabled)
{
    return write_bits(dev, REG_PWR_MGMT_1, BIT_PWR_MGMT_1_SLEEP, 1, enabled);
}

esp_err_t mpu6050_set_sample_rate_div(Mpu6050* dev, uint8_t div)
{
    return write_reg(dev, REG_SMPLRT_DIV, div);
}

esp_err_t mpu6050_set_dlpf(Mpu6050* dev, Mpu6050_DLPF selector)
{
    return write_bits(
        dev, REG_CONFIG, BIT_CONFIG_DLPF_CFG, MASK_CONFIG_DLPF_CFG, selector);
}

typedef struct {
    Mpu6050* dev;
    int count;
    esp_err_t status;
    float3 accel_acc;
    float3 rotation_acc;
    EventGroupHandle_t event_group;
} Calibrator;

#define CALIBRATION_COUNT_TOTAL 25

static void calibrate_measure_cb(void* arg)
{
    Calibrator* calib = arg;

    if (calib->status != ESP_OK || calib->count >= CALIBRATION_COUNT_TOTAL)
        goto loop_break;

    float3 accel;
    calib->status = mpu6050_get_acceleration(calib->dev, &accel);

    if (calib->status != ESP_OK)
        goto loop_break;

    float3 rotation;
    calib->status = mpu6050_get_rotation(calib->dev, &rotation);

    if (calib->status != ESP_OK)
        goto loop_break;

    calib->accel_acc.x += accel.x;
    calib->accel_acc.y += accel.y;
    calib->accel_acc.z += accel.z;

    calib->rotation_acc.x += rotation.x;
    calib->rotation_acc.y += rotation.y;
    calib->rotation_acc.z += rotation.z;

    calib->count += 1;
    return;

loop_break:
    xEventGroupSetBits(calib->event_group, 1);
}

esp_err_t mpu6050_calibrate(Mpu6050* dev, const float3* initial_rotation)
{

    Calibrator calib = {
        .dev = dev,
        .event_group = xEventGroupCreate(),
    };

    esp_timer_handle_t timer;
    esp_timer_create_args_t timer_config = {
        .callback = calibrate_measure_cb,
        .arg = &calib,
    };
    CHECK(esp_timer_create(&timer_config, &timer));

    CHECK(esp_timer_start_periodic(timer, 40000));

    xEventGroupWaitBits(calib.event_group,
        1,
        pdFALSE,
        pdFALSE,
        pdMS_TO_TICKS(CALIBRATION_COUNT_TOTAL * 40 * 2));

    CHECK(calib.status);

    CHECK(esp_timer_stop(timer));
    CHECK(esp_timer_delete(timer));

    dev->accel_bias.x = calib.accel_acc.x / (float)CALIBRATION_COUNT_TOTAL
        - initial_rotation->x;
    dev->accel_bias.y = calib.accel_acc.y / (float)CALIBRATION_COUNT_TOTAL
        - initial_rotation->y;
    dev->accel_bias.z = calib.accel_acc.z / (float)CALIBRATION_COUNT_TOTAL
        - initial_rotation->z;

    dev->rotation_bias.x
        = calib.rotation_acc.x / (float)CALIBRATION_COUNT_TOTAL;
    dev->rotation_bias.y
        = calib.rotation_acc.y / (float)CALIBRATION_COUNT_TOTAL;
    dev->rotation_bias.z
        = calib.rotation_acc.z / (float)CALIBRATION_COUNT_TOTAL;

    return ESP_OK;
}
