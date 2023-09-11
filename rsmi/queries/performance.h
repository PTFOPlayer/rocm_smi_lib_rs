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

result_uint32_t busy_percent(uint32_t dv_ind)
{
    if (init.status != RSMI_STATUS_SUCCESS)
    {
        result_uint32_t error = {init.status, 0};
        return error;
    }

    uint32_t percent;
    rsmi_status_t ret = rsmi_dev_busy_percent_get(dv_ind, &percent);

    result_uint32_t res = {ret, percent};
    return res;
}

typedef struct result_util_counter
{
    uint16_t status;
    uint64_t gfx;
    uint64_t mem;
} result_util_counter;

result_util_counter util_counters(uint32_t dv_ind)
{
    if (init.status != RSMI_STATUS_SUCCESS)
    {
        result_util_counter error = {init.status, 0, 0};
        return error;
    }
    uint32_t count = 2;
    uint64_t timestamp;
    rsmi_utilization_counter_t *counters = (rsmi_utilization_counter_t *)malloc(sizeof(rsmi_utilization_counter_t) * count);
    counters[0].type = RSMI_UTILIZATION_COUNTER_FIRST;
    counters[1].type = RSMI_COARSE_GRAIN_MEM_ACTIVITY;

    rsmi_status_t ret = rsmi_utilization_count_get(dv_ind, counters, count, &timestamp);

    result_util_counter res = {ret, counters[0].value, counters[1].value};
    return res;
}