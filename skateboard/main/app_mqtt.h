#pragma once

#include "mqtt_client.h"
#include <stddef.h>

typedef void (*AppMqttSubCb)(const char* topic,
    size_t topic_size,
    const void* data,
    size_t data_size,
    void* arg);

typedef struct {
    const char* topic;
    AppMqttSubCb callback;
    void* arg;
} AppMqttSub;

#define app_mqtt_subs_max_count 4

typedef struct AppMqtt {
    EventGroupHandle_t event_group;
    esp_mqtt_client_handle_t client;
    AppMqttSub subs[app_mqtt_subs_max_count];
    size_t subs_counts;
} AppMqtt;

void app_mqtt_init(AppMqtt* mqtt);
void app_mqtt_subscribe(
    AppMqtt* mqtt, const char* topic, AppMqttSubCb callback, void* arg);
void app_mqtt_publish(
    AppMqtt* mqtt, const char* topic, const void* data, size_t size);
