idf_component_register(
    SRCS skateboard.c app_wifi.c app_mqtt.c new_mpu6050.c
    PRIV_REQUIRES nvs_flash esp_netif esp_wifi mqtt esp_driver_i2c esp_timer
    INCLUDE_DIRS ".")

