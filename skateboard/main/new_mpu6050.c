#include "new_mpu6050.h"
#include "driver/i2c_master.h"
#include "driver/i2c_types.h"
#include "esp_err.h"
#include "esp_log.h"
#include "freertos/idf_additions.h"
#include <assert.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

static const char* TAG = "new_mpu6050";

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

esp_err_t new_mpu6050_init(Mpu6050* dev)
{
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

    CHECK(new_mpu6050_set_clock_source(dev, MPU6050_CLKSEL_PLL_GYRO_X_REF));

    CHECK(new_mpu6050_set_sleep_enabled(dev, false));

    CHECK(new_mpu6050_set_sample_rate_div(dev, 7));

    return ESP_OK;
}

esp_err_t new_mpu6050_get_rotation(Mpu6050* dev, float3* rotation)
{
    static const float resolution[] = {
        [MPU6050_GYRO_RANGE_250] = 250.0f / 32768.0f,
        [MPU6050_GYRO_RANGE_500] = 500.0f / 32768.0f,
        [MPU6050_GYRO_RANGE_1000] = 1000.0f / 32768.0f,
        [MPU6050_GYRO_RANGE_2000] = 2000.0f / 32768.0f,
    };

    uint8_t buffer[6];
    CHECK(read_regs(dev, buffer, 6, REG_GYRO_XOUT_H));

    rotation->x
        = (int16_t)(buffer[0] << 8 | buffer[1]) * resolution[dev->gyro_range];
    rotation->y
        = (int16_t)(buffer[2] << 8 | buffer[3]) * resolution[dev->gyro_range];
    rotation->z
        = (int16_t)(buffer[4] << 8 | buffer[5]) * resolution[dev->gyro_range];

    return ESP_OK;
}

esp_err_t new_mpu6050_get_acceleration(Mpu6050* dev, float3* accel)
{
    static const float resolution[] = {
        [MPU6050_ACCEL_RANGE_2] = 2.0f / 32768.0f,
        [MPU6050_ACCEL_RANGE_4] = 4.0f / 32768.0f,
        [MPU6050_ACCEL_RANGE_8] = 8.0f / 32768.0f,
        [MPU6050_ACCEL_RANGE_16] = 16.0f / 32768.0f,
    };

    uint8_t buffer[6];
    CHECK(read_regs(dev, buffer, 6, REG_ACCEL_XOUT_H));

    accel->x
        = (int16_t)(buffer[0] << 8 | buffer[1]) * resolution[dev->accel_range];
    accel->y
        = (int16_t)(buffer[2] << 8 | buffer[3]) * resolution[dev->accel_range];
    accel->z
        = (int16_t)(buffer[4] << 8 | buffer[5]) * resolution[dev->accel_range];

    return ESP_OK;
}

esp_err_t new_mpu6050_read_temperature(Mpu6050* dev, float* temperature)
{

    uint8_t buffer[2];
    CHECK(read_regs(dev, buffer, 2, REG_TEMP_OUT_H));

    *temperature = (int16_t)(buffer[0] << 8 | buffer[1]) / 340.0f + 36.53f;

    return ESP_OK;
}

esp_err_t new_mpu6050_deinit(Mpu6050* dev)
{
    CHECK(i2c_master_bus_rm_device(dev->i2c_dev));
    CHECK(i2c_del_master_bus(dev->i2c_bus));
    return ESP_OK;
}

esp_err_t new_mpu6050_set_clock_source(Mpu6050* dev, Mpu6050_ClockSource source)
{
    return write_bits(dev,
        REG_PWR_MGMT_1,
        BIT_PWR_MGMT_1_CLKSEL,
        MASK_PWR_MGMT_1_CLKSEL,
        source);
}

esp_err_t new_mpu6050_set_gyro_range(Mpu6050* dev, Mpu6050_GyroRange range)
{
    CHECK(write_bits(dev,
        REG_GYRO_CONFIG,
        BIT_GYRO_CONFIG_FS_SEL,
        MASK_GYRO_CONFIG_FS_SEL,
        range));
    dev->gyro_range = range;
    return ESP_OK;
}

esp_err_t new_mpu6050_set_accel_range(Mpu6050* dev, Mpu6050_AccelRange range)
{
    CHECK(write_bits(dev,
        REG_ACCEL_CONFIG,
        BIT_ACCEL_CONFIG_AFS_SEL,
        MASK_ACCEL_CONFIG_AFS_SEL,
        range));
    dev->accel_range = range;
    return ESP_OK;
}

esp_err_t new_mpu6050_set_sleep_enabled(Mpu6050* dev, bool enabled)
{
    return write_bits(dev, REG_PWR_MGMT_1, BIT_PWR_MGMT_1_SLEEP, 1, enabled);
}

esp_err_t new_mpu6050_set_sample_rate_div(Mpu6050* dev, uint8_t div)
{
    return write_reg(dev, REG_SMPLRT_DIV, div);
}

esp_err_t new_mpu6050_set_dlpf(Mpu6050* dev, Mpu6050_DLPF selector)
{
    return write_bits(
        dev, REG_CONFIG, BIT_CONFIG_DLPF_CFG, MASK_CONFIG_DLPF_CFG, selector);
}
