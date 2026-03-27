#include "esp_log.h"
#include "esp_wifi.h"
#include "freertos/idf_additions.h"
#include "i2cdev.h"
#include "mpu6050.h"
#include "nvs_flash.h"
#include "wifi.h"
#include <stdbool.h>

const char* TAG = "skateboard";

#ifdef CONFIG_MPU6050_I2C_ADDRESS_LOW
#define MPU6050_I2C_ADDRESS MPU6050_I2C_ADDRESS_LOW
#else
#define MPU6050_I2C_ADDRESS MPU6050_I2C_ADDRESS_HIGH
#endif

#define MPU6050_SDA_GPIO CONFIG_MPU6050_SDA_GPIO
#define MPU6050_SCL_GPIO CONFIG_MPU6050_SCL_GPIO

typedef struct App {
    AppWifi wifi;
} App;

void app_main(void)
{
    ESP_LOGI(TAG, "Initializing");
    ESP_LOGI(TAG, "IDF version: %s", esp_get_idf_version());

    esp_err_t status = nvs_flash_init();
    if (status == ESP_ERR_NVS_NO_FREE_PAGES
        || status == ESP_ERR_NVS_NEW_VERSION_FOUND) {
        ESP_ERROR_CHECK(nvs_flash_erase());
        status = nvs_flash_init();
    }
    ESP_ERROR_CHECK(status);

    App app = { 
        .wifi = {
            .wifi_event_group = 0,
            .wifi_retries = 0,
        }, 
    };

    app_wifi_init(&app.wifi);

    ESP_ERROR_CHECK(i2cdev_init());

    mpu6050_dev_t mpu = { 0 };

    ESP_ERROR_CHECK(mpu6050_init_desc(
        &mpu, MPU6050_I2C_ADDRESS, 0, MPU6050_SDA_GPIO, MPU6050_SCL_GPIO));
    ESP_LOGI(TAG,
        "Initializing MPU6050 device. Address: 0x%02x, SDA %d, SCL: %d",
        MPU6050_I2C_ADDRESS,
        MPU6050_SDA_GPIO,
        MPU6050_SCL_GPIO);

    while (true) {
        esp_err_t res = i2c_dev_probe(&mpu.i2c_dev, I2C_DEV_WRITE);
        if (res == ESP_OK) {
            ESP_LOGI(TAG, "Found MPU6050 device");
            break;
        }
        ESP_LOGE(TAG, "MPU6050 not found");
        vTaskDelay(pdMS_TO_TICKS(1000));
    }

    ESP_ERROR_CHECK(mpu6050_init(&mpu));

    ESP_LOGI(TAG, "Acceleration range: %d", mpu.ranges.accel);
    ESP_LOGI(TAG, "Gyroscope range:  %d", mpu.ranges.gyro);

    ESP_LOGI(TAG, "Initialized");
    ESP_LOGI(TAG, "Free memory: %" PRIu32 " bytes", esp_get_free_heap_size());

    while (true) {
        float temp;
        mpu6050_acceleration_t accel = { 0 };
        mpu6050_rotation_t rotation = { 0 };

        ESP_ERROR_CHECK(mpu6050_get_temperature(&mpu, &temp));
        ESP_ERROR_CHECK(mpu6050_get_motion(&mpu, &accel, &rotation));

        ESP_LOGI(TAG,
            "******************************************************************"
            "****");
        ESP_LOGI(TAG,
            "Acceleration: x=%.4f   y=%.4f   z=%.4f",
            accel.x,
            accel.y,
            accel.z);
        ESP_LOGI(TAG,
            "Rotation:     x=%.4f   y=%.4f   z=%.4f",
            rotation.x,
            rotation.y,
            rotation.z);
        ESP_LOGI(TAG, "Temperature:  %.1f", temp);

        vTaskDelay(pdMS_TO_TICKS(100));
    }
}
