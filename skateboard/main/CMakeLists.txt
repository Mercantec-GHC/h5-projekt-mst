idf_component_register(
    SRCS skateboard.c wifi.c
    PRIV_REQUIRES nvs_flash esp_wifi
    INCLUDE_DIRS ".")

