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

result_uint16_t fan_sensors_count(uint32_t dv_ind)
{
    if (init.status != RSMI_STATUS_SUCCESS)
    {
        result_uint16_t error = {init.status, 0};
        return error;
    }
    int64_t speed;
    uint16_t count = 0;
    rsmi_status_t ret = rsmi_dev_fan_rpms_get(dv_ind, count, &speed);
    while (ret == 0)
    {
        count++;
        ret = rsmi_dev_fan_rpms_get(dv_ind, count, &speed);
    }

    result_uint16_t res = {init.status, count};
    return res;
}

result_int64_t fan_speed(uint32_t dv_ind, uint32_t sensor_ind)
{
    if (init.status != RSMI_STATUS_SUCCESS)
    {
        result_int64_t error = {init.status, 0};
        return error;
    }

    int64_t speed;
    rsmi_status_t ret = rsmi_dev_fan_speed_get(dv_ind, sensor_ind, &speed);

    result_int64_t res = {ret, speed};
    return res;
}

result_uint64_t max_fan_speed(uint32_t dv_ind, uint32_t sensor_ind)
{
    if (init.status != RSMI_STATUS_SUCCESS)
    {
        result_uint64_t error = {init.status, 0};
        return error;
    }

    uint64_t speed;
    rsmi_status_t ret = rsmi_dev_fan_speed_max_get(dv_ind, sensor_ind, &speed);

    result_uint64_t res = {ret, speed};
    return res;
}