idf_component_register(
    SRCS skateboard.c app_wifi.c app_mpu.c
    PRIV_REQUIRES nvs_flash esp_wifi
    INCLUDE_DIRS ".")

