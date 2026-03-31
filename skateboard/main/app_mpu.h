#pragma once

#include "mpu6050.h"

typedef struct float3 {
    float x;
    float y;
    float z;
} float3;

typedef struct AppMpu {
    mpu6050_dev_t dev;
} AppMpu;

void app_mpu_init(AppMpu* mpu);
void app_mpu_read_acceleration(AppMpu* mpu, float3* out_accel);
void app_mpu_read_rotation(AppMpu* mpu, float3* out_rotation);
void app_mpu_read_temperature(AppMpu* mpu, float* degree_celsius);
