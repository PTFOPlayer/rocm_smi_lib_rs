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

result_uint16_t power_sensor_count(uint32_t dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_uint16_t error = {init.status, 0};
    return error;
  }
  uint64_t pwr;
  uint16_t count = 0;
  rsmi_status_t ret = rsmi_dev_power_ave_get(dv_ind, count, &pwr);
  while (ret == 0) {
    count++;
    ret = rsmi_dev_power_ave_get(dv_ind, count, &pwr);
  }

  result_uint16_t res = {init.status, count};
  return res;
}

result_uint64_t power_sensor(uint32_t dv_ind, uint16_t sensor) {
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_uint64_t error = {init.status, 0};
    return error;
  }

  uint64_t pwr;
  rsmi_status_t ret = rsmi_dev_power_ave_get(dv_ind, sensor, &pwr);

  result_uint64_t res = {init.status, pwr};
  return res;
}

result_uint64_t power_avg_all(uint32_t dv_ind) {
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_uint64_t error = {init.status, 0};
    return error;
  }

  uint64_t pwr = 0;
  uint16_t count = 0;
  rsmi_status_t ret = rsmi_dev_power_ave_get(dv_ind, count, &pwr);
  while (ret == 0) {
    count++;
    uint64_t temp_pwr = 0;
    ret = rsmi_dev_power_ave_get(dv_ind, count, &temp_pwr);
    pwr+= temp_pwr;
  }

  result_uint64_t res = {init.status, pwr};
  return res;
}

result_uint64_t power_cap(uint32_t dv_ind, uint16_t sensor) {
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_uint64_t error = {init.status, 0};
    return error;
  }

  uint64_t pwr;
  rsmi_status_t ret = rsmi_dev_power_cap_get(dv_ind, sensor, &pwr);

  result_uint64_t res = {init.status, pwr};
  return res;
}

result_uint64_t default_power_cap(uint32_t dv_ind) {
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_uint64_t error = {init.status, 0};
    return error;
  }

  uint64_t pwr;
  rsmi_status_t ret = rsmi_dev_power_cap_default_get(dv_ind,&pwr);

  result_uint64_t res = {init.status, pwr};
  return res;
}

result_uint64_t_dual power_cap_range(uint32_t dv_ind, uint16_t sensor) {
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_uint64_t_dual error = {init.status, 0};
    return error;
  }

  uint64_t max, min;
  rsmi_status_t ret = rsmi_dev_power_cap_range_get(dv_ind, sensor, &max, &min);

  result_uint64_t_dual res = {init.status, max, min};
  return res;
}

