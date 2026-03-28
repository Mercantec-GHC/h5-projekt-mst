#include "app_mqtt.h"
#include "esp_event_base.h"
#include "esp_log.h"
#include "freertos/projdefs.h"
#include "mqtt_client.h"
#include "sdkconfig.h"
#include <string.h>

extern const char* TAG;

enum {
    Ev_Connected = 1 << 0,
    Ev_Failed = 1 << 1,
};

static void event_cb(void* arg, esp_event_base_t base, int32_t id, void* data)
{
    AppMqtt* mqtt = arg;
    esp_mqtt_event_handle_t event = data;
    esp_mqtt_client_handle_t client = event->client;

    (void)client;

    if (id == MQTT_EVENT_CONNECTED) {
        xEventGroupSetBits(mqtt->event_group, Ev_Connected);
        return;
    }

    if (id == MQTT_EVENT_ERROR) {
        ESP_LOGE(TAG, "MQTT error occured");

        if (event->error_handle->error_type == MQTT_ERROR_TYPE_TCP_TRANSPORT) {

            if (event->error_handle->esp_tls_last_esp_err) {
                ESP_LOGE(TAG,
                    "Error reported in esp-tls: 0x%x",
                    event->error_handle->esp_tls_last_esp_err);
            } else if (event->error_handle->esp_tls_stack_err) {
                ESP_LOGE(TAG,
                    "Error reported in tls stack: 0x%x",
                    event->error_handle->esp_tls_stack_err);
            } else if (event->error_handle->esp_transport_sock_errno) {
                ESP_LOGE(TAG,
                    "Error captured as transport's socket errno: 0x%x",
                    event->error_handle->esp_transport_sock_errno);
            }

            ESP_LOGI(TAG,
                "Last errno string (%s)",
                strerror(event->error_handle->esp_transport_sock_errno));
        }

        xEventGroupSetBits(mqtt->event_group, Ev_Failed);
        return;
    }

    if (id == MQTT_EVENT_DATA) {
        for (size_t i = 0; i < mqtt->subs_counts; ++i) {
            size_t sub_topic_len = strlen(mqtt->subs[i].topic);

            if (sub_topic_len == event->topic_len
                && strncmp(mqtt->subs[i].topic, event->topic, event->topic_len)
                    == 0) {

                mqtt->subs[i].callback(event->topic,
                    event->topic_len,
                    event->data,
                    event->data_len,
                    mqtt->subs[i].arg);
                break;
            }
        }
        return;
    }
}

void app_mqtt_init(AppMqtt* mqtt)
{
    ESP_LOGI(TAG, "Initializing MQTT");

    *mqtt = (AppMqtt) {
        .event_group = xEventGroupCreate(),
        .subs_counts = 0,
    };

    esp_mqtt_client_config_t mqtt_config = {
        .broker.address.uri = CONFIG_MQTT_URL,
        .credentials.username = CONFIG_MQTT_USERNAME,
        .credentials.authentication.password = CONFIG_MQTT_PASSWORD,
    };

    mqtt->client = esp_mqtt_client_init(&mqtt_config);
    esp_mqtt_client_register_event(
        mqtt->client, ESP_EVENT_ANY_ID, event_cb, mqtt);
    esp_mqtt_client_start(mqtt->client);

    ESP_LOGI(TAG, "MQTT started");

    EventBits_t event_bits = xEventGroupWaitBits(mqtt->event_group,
        Ev_Connected | Ev_Failed,
        pdFALSE,
        pdFALSE,
        pdMS_TO_TICKS(1000 * 60 * 2));

    if (event_bits & Ev_Connected) {
        ESP_LOGI(TAG, "Connected to MQTT broker at %s", CONFIG_MQTT_URL);
    } else {
        ESP_LOGE(
            TAG, "Could not connect to MQTT broker at %s", CONFIG_MQTT_URL);
    }
}

void app_mqtt_subscribe(
    AppMqtt* mqtt, const char* topic, AppMqttSubCb callback, void* arg)
{
    if (mqtt->subs_counts + 1 >= app_mqtt_subs_max_count) {
        ESP_LOGE(TAG,
            "Max MQTT subscriptions exceeded (%d)",
            app_mqtt_subs_max_count);

        return;
    }

    int status = esp_mqtt_client_subscribe_single(mqtt->client, topic, 0);
    if (status < 0) {
        ESP_LOGE(TAG, "Could not subscribe to MQTT topic '%s'", topic);
        return;
    }

    mqtt->subs[mqtt->subs_counts++] = (AppMqttSub) { topic, callback, arg };
}

void app_mqtt_publish(
    AppMqtt* mqtt, const char* topic, const void* data, size_t size)
{
    int status
        = esp_mqtt_client_publish(mqtt->client, topic, data, (int)size, 1, 0);
    if (status < 0) {
        ESP_LOGE(TAG, "Could not publish to MQTT topic '%s'", topic);
        return;
    }
}
