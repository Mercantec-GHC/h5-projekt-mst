#pragma once

#include "driver/i2c_types.h"
#include "esp_err.h"
#include <stdbool.h>
#include <stdint.h>

typedef enum : uint8_t {
    // Internal 8MHz oscillator
    MPU6050_CLKSEL_INTERNAL_8MHZ_OSC = 0,
    // PLL with X axis gyroscope reference
    MPU6050_CLKSEL_PLL_GYRO_X_REF = 1,
    // PLL with Y axis gyroscope reference
    MPU6050_CLKSEL_PLL_GYRO_Y_REF = 2,
    // PLL with Z axis gyroscope reference
    MPU6050_CLKSEL_PLL_GYRO_Z_REF = 3,
    // PLL with external 32.768kHz reference
    MPU6050_CLKSEL_PLL_EXTERNAL_32_768_HZ_REF = 4,
    // PLL with external 19.2MHz reference
    MPU6050_CLKSEL_PLL_EXTERNAL_19_200K_HZ_REF = 5,
    // Stops the clock and keeps the timing generator in reset
    MPU6050_CLKSEL_PLL_STOP_RESET = 7,

} Mpu6050_ClockSource;

typedef enum : uint8_t {
    // ± 250 °/s
    MPU6050_GYRO_RANGE_250 = 0,
    // ± 500 °/s
    MPU6050_GYRO_RANGE_500 = 1,
    // ± 1000 °/s
    MPU6050_GYRO_RANGE_1000 = 2,
    // ± 2000 °/s
    MPU6050_GYRO_RANGE_2000 = 3,
} Mpu6050_GyroRange;

typedef enum : uint8_t {
    // ± 2g
    MPU6050_ACCEL_RANGE_2 = 0,
    // ± 4g
    MPU6050_ACCEL_RANGE_4 = 1,
    // ± 8g
    MPU6050_ACCEL_RANGE_8 = 2,
    // ± 16g
    MPU6050_ACCEL_RANGE_16 = 3,
} Mpu6050_AccelRange;

typedef struct {
    float x;
    float y;
    float z;
} float3;

typedef struct {
    i2c_master_bus_handle_t i2c_bus;
    i2c_master_dev_handle_t i2c_dev;
    Mpu6050_GyroRange gyro_range;
    Mpu6050_AccelRange accel_range;
    float3 rotation_bias;
    float3 accel_bias;
} Mpu6050;

esp_err_t new_mpu6050_init(Mpu6050* dev);
esp_err_t new_mpu6050_deinit(Mpu6050* dev);

esp_err_t new_mpu6050_get_rotation(Mpu6050* dev, float3* rotation);
esp_err_t new_mpu6050_get_acceleration(Mpu6050* dev, float3* accel);
esp_err_t new_mpu6050_read_temperature(Mpu6050* dev, float* temperature);

esp_err_t new_mpu6050_set_clock_source(
    Mpu6050* dev, Mpu6050_ClockSource source);
esp_err_t new_mpu6050_set_gyro_range(Mpu6050* dev, Mpu6050_GyroRange range);
esp_err_t new_mpu6050_set_accel_range(Mpu6050* dev, Mpu6050_AccelRange range);
esp_err_t new_mpu6050_set_sleep_enabled(Mpu6050* dev, bool enabled);

#define NEW_MPU6050_SAMPLE_RATE_TO_DIV_1KHZ(SAMPLE_RATE)                       \
    ((uint8_t)(1000.0f / (float)(SAMPLE_RATE) - 1.0f))
#define NEW_MPU6050_SAMPLE_RATE_TO_DIV_8KHZ(SAMPLE_RATE)                       \
    ((uint8_t)(8000.0f / (float)(SAMPLE_RATE) - 1.0f))

// Sample rate = Gyro output rate / (1 + Sample rate divier)
// Optionally use
// - `NEW_MPU6050_SAMPLE_RATE_TO_DIV_1KHZ()`, if DLPF is enabled
// - `NEW_MPU6050_SAMPLE_RATE_TO_DIV_8KHZ()`, if DLPF is disabled
esp_err_t new_mpu6050_set_sample_rate_div(Mpu6050* dev, uint8_t div);

typedef enum : uint8_t {
    // Accelerometer: bandwidth = 260Hz, delay = 0.0ms
    // Gyroscope:     bandwidth = 256Hz, delay = 0.98ms
    // Fs:            8kHz
    MPU6050_DLPF_0 = 0,
    // Accelerometer: bandwidth = 184Hz, delay = 2.0ms
    // Gyroscope:     bandwidth = 188Hz, delay = 1.9ms
    // Fs:            1kHz
    MPU6050_DLPF_1 = 1,
    // Accelerometer: bandwidth =  94Hz, delay = 3.0ms
    // Gyroscope:     bandwidth =  98Hz, delay = 2.8ms
    // Fs:            1kHz
    MPU6050_DLPF_2 = 2,
    // Accelerometer: bandwidth = 44Hz, delay = 4.9ms
    // Gyroscope:     bandwidth = 32Hz, delay = 4.8ms
    // Fs:            1kHz
    MPU6050_DLPF_3 = 3,
    // Accelerometer: bandwidth = 21Hz, delay = 8.5ms
    // Gyroscope:     bandwidth = 20Hz, delay = 8.3ms
    // Fs:            1kHz
    MPU6050_DLPF_4 = 4,
    // Accelerometer: bandwidth = 10Hz, delay = 13.8ms
    // Gyroscope:     bandwidth = 10Hz, delay = 13.4ms
    // Fs:            1kHz
    MPU6050_DLPF_5 = 5,
    // Accelerometer: bandwidth = 5Hz, delay = 19.0ms
    // Gyroscope:     bandwidth = 5Hz, delay = 18.6ms
    // Fs:            1kHz
    MPU6050_DLPF_6 = 6,
} Mpu6050_DLPF;

// Digital low pass filter (DLPF)
// Note: Sampling rate is dependent on whether DLPF is enabled.
esp_err_t new_mpu6050_set_dlpf(Mpu6050* dev, Mpu6050_DLPF selector);

esp_err_t new_mpu6050_calibrate(Mpu6050* dev, const float3* initial_rotation);
