idf_component_register(
    SRCS "main.c"
    PRIV_REQUIRES spi_flash mpu6050 esp_netif nvs_flash esp_wifi
    INCLUDE_DIRS "")

