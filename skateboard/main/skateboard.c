#include "app_mqtt.h"
#include "app_wifi.h"
#include "esp_event.h"
#include "esp_log.h"
#include "esp_netif.h"
#include "freertos/idf_additions.h"
#include "nvs_flash.h"
#include <stdbool.h>
#include <stdio.h>

#define NEW_DRIVER

#ifndef NEW_DRIVER
#include "app_mpu.h"
#else
#include "new_mpu6050.h"
#endif

const char* TAG = "skateboard";

typedef struct App {
    AppWifi wifi;
    AppMqtt mqtt;
#ifndef NEW_DRIVER
    AppMpu mpu;
#else
    Mpu6050 mpu;
#endif
} App;

#define msg_buffer_capacity 1024
char msg_buffer[msg_buffer_capacity];

static void configure_cb(const char* topic,
    size_t topic_size,
    const void* data,
    size_t data_size,
    void* arg)
{
    App* app = arg;
    (void)app;

    ESP_LOGI(TAG, "Received configure event (%.*s)", (int)topic_size, topic);
    ESP_LOGI(TAG, "Data: %.*s", (int)data_size, (const char*)data);
}

static void publish_message(App* app)
{
    float3 accel = { 0 };
#ifndef NEW_DRIVER
    app_mpu_read_acceleration(&app->mpu, &accel);
#else
    ESP_ERROR_CHECK(new_mpu6050_get_acceleration(&app->mpu, &accel));
#endif

    float3 rotation = { 0 };
#ifndef NEW_DRIVER
    app_mpu_read_rotation(&app->mpu, &rotation);
#else
    ESP_ERROR_CHECK(new_mpu6050_get_rotation(&app->mpu, &rotation));
#endif

    float temp = 0;
#ifndef NEW_DRIVER
    app_mpu_read_temperature(&app->mpu, &temp);
#else
    ESP_ERROR_CHECK(new_mpu6050_read_temperature(&app->mpu, &temp));
#endif

    int msg_size = snprintf(msg_buffer,
        msg_buffer_capacity - 1,
        "{ \"acceleration\": [% 9.4f, % 9.4f, % 9.4f], "
        "\"rotation\": [% 9.4f, % 9.4f, % 9.4f], "
        "\"temperature\": % 5.2f }",
        accel.x,
        accel.y,
        accel.z,
        rotation.x,
        rotation.y,
        rotation.z,
        temp);

#if false
    app_mqtt_publish(&app->mqtt, "/skateboard/update", msg_buffer, msg_size);
#else
    ESP_LOGI(TAG, "%.*s", (int)msg_size, msg_buffer);
#endif
}

void app_main(void)
{
    ESP_LOGI(TAG, "Initializing");
    ESP_LOGI(TAG, "IDF version: %s", esp_get_idf_version());

    ESP_ERROR_CHECK(nvs_flash_init());
    ESP_ERROR_CHECK(esp_netif_init());
    ESP_ERROR_CHECK(esp_event_loop_create_default());

    App app = { 
        .wifi = {
            .event_group = 0,
            .wifi_retries = 0,
        }, 
    };

    // app_wifi_init(&app.wifi);
    // app_mqtt_init(&app.mqtt);

    ESP_LOGI(TAG, "=== Initializing MPU6050 ===");
#ifndef NEW_DRIVER
    app_mpu_init(&app.mpu);
#else
    ESP_ERROR_CHECK(new_mpu6050_init(&app.mpu));
#endif
    ESP_LOGI(TAG, "=== MPU6050 initialized ===");

    // app_mqtt_subscribe(&app.mqtt, "/skateboard/configure", configure_cb,
    // &app);

    ESP_LOGI(TAG, "Initialized");
    ESP_LOGI(TAG, "Free memory: %" PRIu32 " bytes", esp_get_free_heap_size());

    while (true) {
        publish_message(&app);

        vTaskDelay(pdMS_TO_TICKS(1000));
    }
}
