idf_component_register(
    SRCS skateboard.c app_wifi.c app_mpu.c app_mqtt.c
    PRIV_REQUIRES nvs_flash esp_netif esp_wifi mqtt
    INCLUDE_DIRS ".")

