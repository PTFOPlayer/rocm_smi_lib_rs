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

result_u16 power_sensor_count(u32 dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_u16 error = {init.status, 0};
    return error;
  }
  u64 pwr;
  u16 count = 0;
  rsmi_status_t ret = rsmi_dev_power_ave_get(dv_ind, count, &pwr);
  while (ret == 0) {
    count++;
    ret = rsmi_dev_power_ave_get(dv_ind, count, &pwr);
  }

  result_u16 res = {init.status, count};
  return res;
}

result_u64 power_sensor(u32 dv_ind, u16 sensor) {
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_u64 error = {init.status, 0};
    return error;
  }

  u64 pwr;
  rsmi_status_t ret = rsmi_dev_power_ave_get(dv_ind, sensor, &pwr);

  result_u64 res = {init.status, pwr};
  return res;
}

result_u64 power_avg_all(u32 dv_ind) {
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_u64 error = {init.status, 0};
    return error;
  }

  u64 pwr = 0;
  u16 count = 0;
  rsmi_status_t ret = rsmi_dev_power_ave_get(dv_ind, count, &pwr);
  while (ret == 0) {
    count++;
    u64 temp_pwr = 0;
    ret = rsmi_dev_power_ave_get(dv_ind, count, &temp_pwr);
    pwr+= temp_pwr;
  }

  result_u64 res = {init.status, pwr};
  return res;
}

result_u64 power_cap(u32 dv_ind, u16 sensor) {
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_u64 error = {init.status, 0};
    return error;
  }

  u64 pwr;
  rsmi_status_t ret = rsmi_dev_power_cap_get(dv_ind, sensor, &pwr);

  result_u64 res = {init.status, pwr};
  return res;
}

result_u64 default_power_cap(u32 dv_ind) {
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_u64 error = {init.status, 0};
    return error;
  }

  u64 pwr;
  rsmi_status_t ret = rsmi_dev_power_cap_default_get(dv_ind,&pwr);

  result_u64 res = {init.status, pwr};
  return res;
}

result_u64_dual power_cap_range(u32 dv_ind, u16 sensor) {
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_u64_dual error = {init.status, 0};
    return error;
  }

  u64 max, min;
  rsmi_status_t ret = rsmi_dev_power_cap_range_get(dv_ind, sensor, &max, &min);

  result_u64_dual res = {init.status, max, min};
  return res;
}

