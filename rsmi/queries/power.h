#include <stdint.h>
#include <stdlib.h>
#include <rocm_smi/rocm_smi.h>
#include <stdio.h>

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