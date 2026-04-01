#include "app_mqtt.h"
#include "app_wifi.h"
#include "esp_event.h"
#include "esp_log.h"
#include "esp_netif.h"
#include "esp_timer.h"
#include "freertos/idf_additions.h"
#include "mpu6050.h"
#include "nvs_flash.h"
#include <math.h>
#include <stdbool.h>
#include <stdio.h>

const char* TAG = "skateboard";

typedef struct {
    float3 rotation;
    float3 uncertainty;
} KalmanState;

static void kalman_update_axis(
    float* angle, float* uncertainty, float rotation_rate, float rotation)
{
    const float time_delta = 0.04f;
    const float std_div0 = 4.0f;
    const float std_div1 = 3.0f;

    *angle += time_delta * rotation_rate;
    *uncertainty += time_delta * time_delta * std_div0 * std_div0;
    float gain = *uncertainty * 1 / (*uncertainty + std_div1 * std_div1);
    *angle += gain * (rotation - *angle);
    *uncertainty *= (1 - gain);
}

void kalman_update(KalmanState* state, float3 rotation_rate, float3 rotation)
{
    kalman_update_axis(
        &state->rotation.x, &state->uncertainty.x, rotation_rate.x, rotation.x);
    kalman_update_axis(
        &state->rotation.y, &state->uncertainty.y, rotation_rate.y, rotation.y);
    kalman_update_axis(
        &state->rotation.z, &state->uncertainty.z, rotation_rate.z, rotation.z);
}

typedef struct App {
    AppWifi wifi;
    AppMqtt mqtt;
    Mpu6050 mpu;

    float last_temp;

    float3 gyro_rotation;
    float3 accel_rotation;
    KalmanState kalman;
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
        "{  gyro: [% 9.4f, % 9.4f, % 9.4f], accel: [% 9.4f, % 9.4f, % 9.4f], "
        "kalman: [% 9.4f, % 9.4f, % 9.4f] }",
        app->gyro_rotation.x,
        app->gyro_rotation.y,
        app->gyro_rotation.z,
        app->accel_rotation.x,
        app->accel_rotation.y,
        app->accel_rotation.z,
        app->kalman.rotation.x,
        app->kalman.rotation.y,
        app->kalman.rotation.z);

#if false
    app_mqtt_publish(&app->mqtt, "/skateboard/update", msg_buffer, msg_size);
#else
    ESP_LOGI(TAG, "%.*s", (int)msg_size, msg_buffer);
#endif
}

static float calculate_accel_axis_angle(
    float axis, float tangent0, float tangent1)
{
    return atan(axis / sqrtf(tangent0 * tangent0 + tangent1 * tangent1))
        * (1.0f / (M_PI / 180.0f));
}

static void mpu_timer_cb(void* arg)
{
    App* app = arg;
    (void)app;

    float3 accel = { 0 };
    ESP_ERROR_CHECK(mpu6050_get_acceleration(&app->mpu, &accel));

    float3 rotation = { 0 };
    ESP_ERROR_CHECK(mpu6050_get_rotation(&app->mpu, &rotation));

    float temp = 0;
    ESP_ERROR_CHECK(mpu6050_read_temperature(&app->mpu, &temp));

    app->gyro_rotation.x += rotation.x * 0.04;
    app->gyro_rotation.y += rotation.y * 0.04;
    app->gyro_rotation.z += rotation.z * 0.04;

    app->last_temp = temp;

    app->accel_rotation.x
        = calculate_accel_axis_angle(accel.x, accel.y, accel.z);
    app->accel_rotation.y
        = calculate_accel_axis_angle(accel.y, accel.x, accel.z);
    app->accel_rotation.z = 0.0f;
    // = calculate_accel_axis_angle(accel.z, accel.x, accel.y);

    kalman_update(&app->kalman, rotation, app->accel_rotation);
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

    ESP_ERROR_CHECK(mpu6050_init(&app.mpu));

    ESP_LOGI(TAG, "Calibrating MPU6050");
    ESP_ERROR_CHECK(
        mpu6050_calibrate(&app.mpu, &(float3) { 0.0f, 0.0f, -1.0f }));
    ESP_LOGI(TAG, "MPU6050 calibrated");

    ESP_ERROR_CHECK(esp_timer_start_periodic(mpu_timer, 40000));
    vTaskDelay(pdMS_TO_TICKS(100));

    // app_mqtt_subscribe(&app.mqtt, "/skateboard/configure", configure_cb,
    // &app);

    ESP_LOGI(TAG, "Initialized");
    ESP_LOGI(TAG, "Free memory: %" PRIu32 " bytes", esp_get_free_heap_size());

    while (true) {
        publish_message(&app);

        vTaskDelay(pdMS_TO_TICKS(200));
    }
}
