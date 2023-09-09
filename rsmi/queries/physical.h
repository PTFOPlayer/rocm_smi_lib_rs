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

result_fans fans(uint32_t dv_ind)
{
    result_fans res = {0, 0, NULL, NULL, NULL};
    if (init.status != RSMI_STATUS_SUCCESS)
    {
        res.status = init.status;
        return res;
    }

    int64_t *fan_rpm = (int64_t *)malloc(sizeof(int64_t));
    int64_t *fan_speed = (int64_t *)malloc(sizeof(int64_t));
    uint64_t *fan_cap = (uint64_t *)malloc(sizeof(uint64_t));

    uint16_t count = 0;

    rsmi_status_t ret_rpm = rsmi_dev_fan_rpms_get(dv_ind, count, &fan_rpm[count]);
    rsmi_status_t ret_speed = rsmi_dev_fan_speed_get(dv_ind, count, &fan_speed[count]);
    rsmi_status_t ret_cap = rsmi_dev_fan_speed_max_get(dv_ind, count, &fan_cap[count]);

    count++;

    if (ret_rpm != RSMI_STATUS_SUCCESS)
    {
        res.status = ret_rpm;
        return res;
    }
    if (ret_speed != RSMI_STATUS_SUCCESS)
    {
        res.status = ret_speed;
        return res;
    }
    if (ret_cap != RSMI_STATUS_SUCCESS)
    {
        res.status = ret_cap;
        return res;
    }

    while (ret_rpm == ret_speed == ret_cap == 0)
    {
        ret_rpm = rsmi_dev_fan_rpms_get(dv_ind, count, &fan_rpm[count]);
        ret_speed = rsmi_dev_fan_speed_get(dv_ind, count, &fan_speed[count]);
        ret_cap = rsmi_dev_fan_speed_max_get(dv_ind, count, &fan_cap[count]);
        if (ret_rpm != ret_speed != ret_cap != RSMI_STATUS_SUCCESS)
            break;

        count++;
        fan_rpm = (int64_t *)realloc(fan_rpm, count * sizeof(int64_t));
        fan_speed = (int64_t *)realloc(fan_speed, count * sizeof(int64_t));
        fan_cap = (uint64_t *)realloc(fan_cap, count * sizeof(uint64_t));
    }

    res.status = RSMI_STATUS_SUCCESS;
    res.sensors = count;
    res.fan_rpm_per_sensor = fan_rpm;
    res.fan_speed_per_sensor = fan_speed;
    res.max_fan_speed_per_sensor = fan_cap;

    return res;
}

result_int64_t temperature(uint32_t dv_ind, uint32_t sensor, rsmi_temperature_metric_t metric)
{
    if (init.status != RSMI_STATUS_SUCCESS)
    {
        result_int64_t error = {init.status, 0};
        return error;
    }

    int64_t temp;
    rsmi_status_t ret = rsmi_dev_temp_metric_get(dv_ind, sensor, metric, &temp);

    result_int64_t res = {ret, temp};
    return res;
}