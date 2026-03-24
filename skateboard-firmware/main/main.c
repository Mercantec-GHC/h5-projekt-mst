#include "esp_chip_info.h"
#include "esp_flash.h"
#include "esp_system.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "mpu6050.h"
#include "sdkconfig.h"
#include <inttypes.h>
#include <stdio.h>

static mpu6050_handle_t mpu6050_dev = NULL;

static mpu6050_acce_value_t acce;
static mpu6050_gyro_value_t gyro;

static void mpu6050_init()
{
    mpu6050_dev = mpu6050_create(BSP_I2C_NUM, MPU6050_I2C_ADDRESS);
    mpu6050_config(mpu6050_dev, ACCE_FS_4G, GYRO_FS_500DPS);
    mpu6050_wake_up(mpu6050_dev);
}

static void mpu6050_read(void* pvParameters)
{
    mpu6050_get_acce(mpu6050_dev, &acce);
    mpu6050_get_gyro(mpu6050_dev, &gyro);
    mpu6050_complimentory_filter(
        mpu6050_dev, &acce, &gyro, &complimentary_angle);
}

void app_main(void)
{
    mpu6050_init();

    printf("Hello world!\n");

    for (size_t i = 0; i < 10000) {
        printf("acce_x:%.2f, acce_y:%.2f, acce_z:%.2f\n",
            acce.acce_x,
            acce.acce_y,
            acce.acce_z);
        printf("gyro_x:%.2f, gyro_y:%.2f, gyro_z:%.2f",
            gyro.gyro_x,
            gyro.gyro_y,
            gyro.gyro_z);
    }

    /* Print chip information */
    esp_chip_info_t chip_info;
    uint32_t flash_size;
    esp_chip_info(&chip_info);
    printf("This is %s chip with %d CPU core(s), %s%s%s%s, ",
        CONFIG_IDF_TARGET,
        chip_info.cores,
        (chip_info.features & CHIP_FEATURE_WIFI_BGN) ? "WiFi/" : "",
        (chip_info.features & CHIP_FEATURE_BT) ? "BT" : "",
        (chip_info.features & CHIP_FEATURE_BLE) ? "BLE" : "",
        (chip_info.features & CHIP_FEATURE_IEEE802154)
            ? ", 802.15.4 (Zigbee/Thread)"
            : "");

    unsigned major_rev = chip_info.revision / 100;
    unsigned minor_rev = chip_info.revision % 100;
    printf("silicon revision v%d.%d, ", major_rev, minor_rev);
    if (esp_flash_get_size(NULL, &flash_size) != ESP_OK) {
        printf("Get flash size failed");
        return;
    }

    printf("%" PRIu32 "MB %s flash\n",
        flash_size / (uint32_t)(1024 * 1024),
        (chip_info.features & CHIP_FEATURE_EMB_FLASH) ? "embedded"
                                                      : "external");

    printf("Minimum free heap size: %" PRIu32 " bytes\n",
        esp_get_minimum_free_heap_size());

    for (int i = 10; i >= 0; i--) {
        printf("Restarting in %d seconds...\n", i);
        vTaskDelay(1000 / portTICK_PERIOD_MS);
    }
    printf("Restarting now.\n");
    fflush(stdout);
    esp_restart();
}
