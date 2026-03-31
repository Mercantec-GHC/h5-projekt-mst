#include "app_mqtt.h"
#include "app_wifi.h"
#include "esp_event.h"
#include "esp_log.h"
#include "esp_netif.h"
#include "esp_timer.h"
#include "freertos/idf_additions.h"
#include "nvs_flash.h"
#include <stdbool.h>
#include <stdio.h>

#include "new_mpu6050.h"

const char* TAG = "skateboard";

typedef struct App {
    AppWifi wifi;
    AppMqtt mqtt;
    Mpu6050 mpu;

    float3 rotation_acc;
    float3 accel_acc;
    float time_acc;
    float last_temp;
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

    int msg_size = snprintf(msg_buffer,
        msg_buffer_capacity - 1,
        "{ \"acceleration\": [% 9.4f, % 9.4f, % 9.4f], "
        "\"rotation\": [% 9.4f, % 9.4f, % 9.4f], "
        "\"temperature\": % 5.2f }",
        app->accel_acc.x,
        app->accel_acc.y,
        app->accel_acc.z,
        app->rotation_acc.x,
        app->rotation_acc.y,
        app->rotation_acc.z,
        app->last_temp);

#if false
    app_mqtt_publish(&app->mqtt, "/skateboard/update", msg_buffer, msg_size);
#else
    ESP_LOGI(TAG, "%.*s", (int)msg_size, msg_buffer);
#endif
}

static void mpu_timer_cb(void* arg)
{
    App* app = arg;
    (void)app;

    float3 accel = { 0 };
    ESP_ERROR_CHECK(new_mpu6050_get_acceleration(&app->mpu, &accel));

    float3 rotation = { 0 };
    ESP_ERROR_CHECK(new_mpu6050_get_rotation(&app->mpu, &rotation));

    float temp = 0;
    ESP_ERROR_CHECK(new_mpu6050_read_temperature(&app->mpu, &temp));

    app->accel_acc.x = accel.x;
    app->accel_acc.y = accel.y;
    app->accel_acc.z = accel.z;

    app->rotation_acc.x = rotation.x;
    app->rotation_acc.y = rotation.y;
    app->rotation_acc.z = rotation.z;

    app->last_temp = temp;
}

void app_main(void)
{
    ESP_LOGI(TAG, "Initializing");
    ESP_LOGI(TAG, "IDF version: %s", esp_get_idf_version());

    ESP_ERROR_CHECK(nvs_flash_init());
    ESP_ERROR_CHECK(esp_netif_init());
    ESP_ERROR_CHECK(esp_event_loop_create_default());

    App app = { };

    esp_timer_handle_t mpu_timer;
    esp_timer_create_args_t mpu_timer_config = {
        .callback = mpu_timer_cb,
        .arg = &app,
    };
    ESP_ERROR_CHECK(esp_timer_create(&mpu_timer_config, &mpu_timer));

    // app_wifi_init(&app.wifi);
    // app_mqtt_init(&app.mqtt);

    ESP_ERROR_CHECK(new_mpu6050_init(&app.mpu));

    ESP_LOGI(TAG, "Calibrating MPU6050");
    ESP_ERROR_CHECK(
        new_mpu6050_calibrate(&app.mpu, &(float3) { 0.0f, 0.0f, -1.0f }));
    ESP_LOGI(TAG, "MPU6050 calibrated");

    ESP_ERROR_CHECK(esp_timer_start_periodic(mpu_timer, 40000));
    vTaskDelay(pdMS_TO_TICKS(100));

    // app_mqtt_subscribe(&app.mqtt, "/skateboard/configure", configure_cb,
    // &app);

    ESP_LOGI(TAG, "Initialized");
    ESP_LOGI(TAG, "Free memory: %" PRIu32 " bytes", esp_get_free_heap_size());

    while (true) {
        publish_message(&app);

        vTaskDelay(pdMS_TO_TICKS(1000));
    }
}
