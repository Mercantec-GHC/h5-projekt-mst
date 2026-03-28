#include "app_mpu.h"
#include "app_wifi.h"
#include "esp_log.h"
#include "freertos/idf_additions.h"
#include "i2cdev.h"
#include "mpu6050.h"
#include "nvs_flash.h"
#include <stdbool.h>

const char* TAG = "skateboard";

typedef struct App {
    AppWifi wifi;
    AppMpu mpu;
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
    app_mpu_init(&app.mpu);

    ESP_LOGI(TAG, "Initialized");
    ESP_LOGI(TAG, "Free memory: %" PRIu32 " bytes", esp_get_free_heap_size());

    while (true) {
        float accel_x;
        float accel_y;
        float accel_z;
        app_mpu_read_acceleration(&app.mpu, &accel_x, &accel_y, &accel_z);

        float rotation_x;
        float rotation_y;
        float rotation_z;
        app_mpu_read_rotation(&app.mpu, &rotation_x, &rotation_y, &rotation_z);

        float temp;
        app_mpu_read_temperature(&app.mpu, &temp);

        ESP_LOGI(TAG,
            "Acceleration: x=% 7.2f y=% 7.2f z=% 7.2f "
            "Rotation: x=% 7.1f y=% 7.1f z=% 7.1f "
            "Temperature: % 2.1f",
            accel_x,
            accel_y,
            accel_z,
            rotation_x,
            rotation_y,
            rotation_z,
            temp);

        vTaskDelay(pdMS_TO_TICKS(200));
    }
}
