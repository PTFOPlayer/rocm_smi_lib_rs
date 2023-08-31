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

// to be delited
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
  while (ret == 0)
  {
    count++;
    ret = rsmi_dev_power_ave_get(dv_ind, count, &pwr);
  }

  result_uint16_t res = {init.status, count};
  return res;
}

// to be delited
result_uint64_t power_sensor(uint32_t dv_ind, uint16_t sensor)
{
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

// to be delited
result_uint64_t power_avg_all(uint32_t dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_uint64_t error = {init.status, 0};
    return error;
  }

  uint64_t pwr = 0;
  uint16_t count = 0;
  rsmi_status_t ret = rsmi_dev_power_ave_get(dv_ind, count, &pwr);
  while (ret == 0)
  {
    count++;
    uint64_t temp_pwr = 0;
    ret = rsmi_dev_power_ave_get(dv_ind, count, &temp_pwr);
    pwr += temp_pwr;
  }

  result_uint64_t res = {init.status, pwr};
  return res;
}

// to be delited
result_uint64_t power_cap(uint32_t dv_ind, uint16_t sensor)
{
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

// to be delited
result_uint64_t default_power_cap(uint32_t dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_uint64_t error = {init.status, 0};
    return error;
  }

  uint64_t pwr;
  rsmi_status_t ret = rsmi_dev_power_cap_default_get(dv_ind, &pwr);

  result_uint64_t res = {init.status, pwr};
  return res;
}

// to be delited
result_uint64_t_dual power_cap_range(uint32_t dv_ind, uint16_t sensor)
{
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

result_power power_data(uint32_t dv_ind)
{
  result_power res = {
      .status = 0,
      .sensors = 0,
      .default_power_cap = 0,
      .power_per_sensor = NULL,
      .power_cap_per_sensor = NULL,
      .power_cap_min_sensor = NULL,
      .power_cap_max_sensor = NULL
  };
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_power error = {init.status};
    return error;
  }

  uint64_t *pwr_ave = (uint64_t *)malloc(sizeof(uint64_t));
  uint64_t *pwr_cap = (uint64_t *)malloc(sizeof(uint64_t));
  uint64_t *pwr_cap_min = (uint64_t *)malloc(sizeof(uint64_t));
  uint64_t *pwr_cap_max = (uint64_t *)malloc(sizeof(uint64_t));

  uint16_t count = 0;

  rsmi_status_t ret_ave = rsmi_dev_power_ave_get(dv_ind, count, &pwr_ave[count]);
  rsmi_status_t ret_cap = rsmi_dev_power_cap_get(dv_ind, count, &pwr_cap[count]);
  rsmi_status_t ret_range = rsmi_dev_power_cap_range_get(dv_ind, count, &pwr_cap_max[count], &pwr_cap_min[count]);

  count++;

  if (ret_ave != RSMI_STATUS_SUCCESS)
  {
    res.status = ret_ave;
    return res;
  }
  if (ret_cap != RSMI_STATUS_SUCCESS)
  {
    res.status = ret_cap;
    return res;
  }
  if (ret_range != RSMI_STATUS_SUCCESS)
  {
    res.status = ret_range;
    return res;
  }

  while (ret_ave == ret_cap == 0)
  {
    ret_ave = rsmi_dev_power_ave_get(dv_ind, count, &pwr_ave[count]);
    ret_cap = rsmi_dev_power_cap_get(dv_ind, count, &pwr_cap[count]);
    ret_range = rsmi_dev_power_cap_range_get(dv_ind, count, &pwr_cap_max[count], &pwr_cap_min[count]);
    if (ret_ave != RSMI_STATUS_SUCCESS || ret_cap != RSMI_STATUS_SUCCESS || ret_range != RSMI_STATUS_SUCCESS)
      break;

    count++;
    pwr_ave = (uint64_t *)realloc(pwr_ave, count * sizeof(uint64_t));
    pwr_cap = (uint64_t *)realloc(pwr_cap, count * sizeof(uint64_t));
    pwr_cap_min = (uint64_t *)realloc(pwr_cap_min, count * sizeof(uint64_t));
    pwr_cap_max = (uint64_t *)realloc(pwr_cap_max, count * sizeof(uint64_t));
  }

  uint64_t pwr_cap_def;
  rsmi_status_t ret_def = rsmi_dev_power_cap_default_get(dv_ind, &pwr_cap_def);

  if (ret_def != RSMI_STATUS_SUCCESS)
  {
    res.status = ret_def;
    return res;
  }

  res.sensors = count;
  res.default_power_cap = pwr_cap_def;
  res.power_per_sensor = pwr_ave;
  res.power_cap_per_sensor = pwr_cap;
  res.power_cap_min_sensor = pwr_cap_min;
  res.power_cap_max_sensor = pwr_cap_max;

  return res;
}
