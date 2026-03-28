#include "app_mpu.h"
#include "esp_log.h"
#include "mpu6050.h"

extern const char* TAG;

#ifdef CONFIG_MPU6050_I2C_ADDRESS_LOW
#define MPU6050_I2C_ADDRESS MPU6050_I2C_ADDRESS_LOW
#else
#define MPU6050_I2C_ADDRESS MPU6050_I2C_ADDRESS_HIGH
#endif

#define MPU6050_SDA_GPIO CONFIG_MPU6050_SDA_GPIO
#define MPU6050_SCL_GPIO CONFIG_MPU6050_SCL_GPIO

void app_mpu_init(AppMpu* mpu)
{
    ESP_ERROR_CHECK(i2cdev_init());

    ESP_ERROR_CHECK(mpu6050_init_desc(
        &mpu->dev, MPU6050_I2C_ADDRESS, 0, MPU6050_SDA_GPIO, MPU6050_SCL_GPIO));
    ESP_LOGI(TAG,
        "Initializing MPU6050 device. Address: 0x%02x, SDA %d, SCL: %d",
        MPU6050_I2C_ADDRESS,
        MPU6050_SDA_GPIO,
        MPU6050_SCL_GPIO);

    int retries = 0;
    while (true) {
        esp_err_t res = i2c_dev_probe(&mpu->dev.i2c_dev, I2C_DEV_WRITE);
        if (res == ESP_OK) {
            ESP_LOGI(TAG, "Found MPU6050 device");
            break;
        }
        if (retries >= 5) {
            ESP_ERROR_CHECK(res);
        }
        ESP_LOGE(TAG, "MPU6050 not found. Retrying (%d/%d)", retries + 1, 5);
        retries += 1;
        vTaskDelay(pdMS_TO_TICKS(1000));
    }

    ESP_ERROR_CHECK(mpu6050_init(&mpu->dev));

    ESP_LOGI(TAG, "MPU6050 acceleration range: %d", mpu->dev.ranges.accel);
    ESP_LOGI(TAG, "MPU6050 gyroscope range:    %d", mpu->dev.ranges.gyro);
}

void app_mpu_read_acceleration(AppMpu* mpu, float* x, float* y, float* z)
{
    mpu6050_acceleration_t acceleration = { 0 };
    ESP_ERROR_CHECK(mpu6050_get_acceleration(&mpu->dev, &acceleration));
    *x = acceleration.x;
    *y = acceleration.y;
    *z = acceleration.z;
}

void app_mpu_read_rotation(AppMpu* mpu, float* x, float* y, float* z)
{
    mpu6050_rotation_t rotation = { 0 };

    ESP_ERROR_CHECK(mpu6050_get_rotation(&mpu->dev, &rotation));
    *x = rotation.x;
    *y = rotation.y;
    *z = rotation.z;
}

void app_mpu_read_temperature(AppMpu* mpu, float* degree_celsius)
{
    ESP_ERROR_CHECK(mpu6050_get_temperature(&mpu->dev, degree_celsius));
}
