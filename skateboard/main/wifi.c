#include "wifi.h"
#include "esp_log.h"
#include "esp_wifi.h"
#include "freertos/idf_additions.h"

extern const char* TAG;

#define WIFI_SSID CONFIG_SKATEBOARD_WIFI_SSID
#define WIFI_PASSWORD CONFIG_SKATEBOARD_WIFI_PASSWORD
#define WIFI_MAXIMUM_RETRIES CONFIG_SKATEBOARD_WIFI_MAXIMUM_RETRIES

enum {
    Ev_Connected = 1 << 0,
    Ev_Failed = 1 << 1,
};

static void event_handler(
    void* arg, esp_event_base_t base, int32_t id, void* data)
{
    AppWifi* app = arg;

    if (base == WIFI_EVENT && id == WIFI_EVENT_STA_START) {
        esp_wifi_connect();
        return;
    }

    if (base == WIFI_EVENT && id == WIFI_EVENT_STA_DISCONNECTED) {
        esp_wifi_connect();

        if (app->wifi_retries < WIFI_MAXIMUM_RETRIES) {
            esp_wifi_connect();
            app->wifi_retries += 1;
            ESP_LOGE(TAG,
                "WIFI connection failed. Retrying (%d/%d)",
                app->wifi_retries + 1,
                WIFI_MAXIMUM_RETRIES);
            return;
        }

        xEventGroupSetBits(app->wifi_event_group, Ev_Failed);
        return;
    }

    if (base == IP_EVENT && id == IP_EVENT_STA_GOT_IP) {
        ip_event_got_ip_t* ip_event = (ip_event_got_ip_t*)data;
        app->wifi_retries = 0;
        ESP_LOGI(TAG,
            "WIFI connected with IP " IPSTR,
            IP2STR(&ip_event->ip_info.ip));
        xEventGroupSetBits(app->wifi_event_group, Ev_Connected);
        return;
    }
}

void app_wifi_init(AppWifi* wifi)
{
    wifi->wifi_event_group = xEventGroupCreate();

    ESP_ERROR_CHECK(esp_netif_init());
    ESP_ERROR_CHECK(esp_event_loop_create_default());
    esp_netif_create_default_wifi_sta();

    wifi_init_config_t init_config = WIFI_INIT_CONFIG_DEFAULT();
    ESP_ERROR_CHECK(esp_wifi_init(&init_config));

    ESP_ERROR_CHECK(esp_event_handler_instance_register(
        WIFI_EVENT, ESP_EVENT_ANY_ID, &event_handler, wifi, NULL));

    ESP_ERROR_CHECK(esp_event_handler_instance_register(
        IP_EVENT, IP_EVENT_STA_GOT_IP, &event_handler, wifi, NULL));

    wifi_config_t config = {
        .sta = {
            .ssid = WIFI_SSID,
            .password = WIFI_PASSWORD,
            .threshold.authmode = WIFI_AUTH_WPA2_PSK,
        },
    };
    ESP_ERROR_CHECK(esp_wifi_set_mode(WIFI_MODE_STA));
    ESP_ERROR_CHECK(esp_wifi_set_config(WIFI_IF_STA, &config));
    ESP_ERROR_CHECK(esp_wifi_start());

    ESP_LOGI(TAG, "WIFI started");

    EventBits_t event_bits = xEventGroupWaitBits(wifi->wifi_event_group,
        Ev_Connected | Ev_Failed,
        pdFALSE,
        pdFALSE,
        portMAX_DELAY);

    if (event_bits & Ev_Connected) {
        ESP_LOGI(TAG, "WIFI connected to %s / '%s'", WIFI_SSID, WIFI_PASSWORD);
    } else if (event_bits & Ev_Failed) {
        ESP_LOGE(TAG,
            "WIFI failed connecting to %s / '%s'",
            WIFI_SSID,
            WIFI_PASSWORD);
    } else {
        ESP_LOGE(TAG, "Unexpected event");
    }
}
