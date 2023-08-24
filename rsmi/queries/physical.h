#include <stdint.h>
#include <stdlib.h>
#include <rocm_smi/rocm_smi.h>

#ifndef STRUCTS_H
#define STRUCTS_H
#include "../structs.h"
#endif

#ifndef INIT_H
#define INIT_H
#include "./init.h"
#endif

result_u16 fan_sensors_count(u32 dv_ind)
{
    if (init.status != RSMI_STATUS_SUCCESS)
    {
        result_u16 error = {init.status, 0};
        return error;
    }
    i64 speed;
    u16 count = 0;
    rsmi_status_t ret = rsmi_dev_fan_rpms_get(dv_ind, count, &speed);
    while (ret == 0)
    {
        count++;
        ret = rsmi_dev_fan_rpms_get(dv_ind, count, &speed);
    }

    result_u16 res = {init.status, count};
    return res;
}

result_i64 fan_speed(u32 dv_ind, u32 sensor_ind)
{
    if (init.status != RSMI_STATUS_SUCCESS)
    {
        result_i64 error = {init.status, 0};
        return error;
    }

    i64 speed;
    rsmi_status_t ret = rsmi_dev_fan_speed_get(dv_ind, sensor_ind, &speed);

    result_i64 res = {ret, speed};
    return res;
}

result_u64 max_fan_speed(u32 dv_ind, u32 sensor_ind)
{
    if (init.status != RSMI_STATUS_SUCCESS)
    {
        result_u64 error = {init.status, 0};
        return error;
    }

    u64 speed;
    rsmi_status_t ret = rsmi_dev_fan_speed_max_get(dv_ind, sensor_ind, &speed);

    result_u64 res = {ret, speed};
    return res;
}