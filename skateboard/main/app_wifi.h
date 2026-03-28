#pragma once

#include "freertos/idf_additions.h"

typedef struct AppWifi {
    EventGroupHandle_t wifi_event_group;
    int wifi_retries;
} AppWifi;

void app_wifi_init(AppWifi* wifi);
