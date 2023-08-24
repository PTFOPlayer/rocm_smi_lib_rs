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

#define VRAM RSMI_MEM_TYPE_VRAM 
#define VIS_VRAM RSMI_MEM_TYPE_VIS_VRAM
#define GTT RSMI_MEM_TYPE_GTT

result_u64 mem_total(u32 dv_ind, rsmi_memory_type_t mem_type) {
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_u64 error = {init.status, 0};
    return error;
  }

  u64 ammount;
  rsmi_status_t ret = rsmi_dev_memory_total_get(dv_ind, mem_type, &ammount);

  result_u64 res = {ret, ammount};
  return res;
}

result_u64 mem_used(u32 dv_ind, rsmi_memory_type_t mem_type) {
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_u64 error = {init.status, 0};
    return error;
  }

  u64 ammount;
  rsmi_status_t ret = rsmi_dev_memory_usage_get(dv_ind, mem_type, &ammount);

  result_u64 res = {ret, ammount};
  return res;
}

result_u64 mem_total_vram(u32 dv_ind) {
  return mem_total(dv_ind, VRAM);
}

result_u64 mem_total_vis_vram(u32 dv_ind) {
  return mem_total(dv_ind, VIS_VRAM);
}

result_u64 mem_total_gtt(u32 dv_ind) {
  return mem_total(dv_ind, GTT);
}

result_u64 mem_used_vram(u32 dv_ind) {
  return mem_used(dv_ind, VRAM);
}

result_u64 mem_used_vis_vram(u32 dv_ind) {
  return mem_used(dv_ind, VIS_VRAM);
}

result_u64 mem_used_gtt(u32 dv_ind) {
  return mem_used(dv_ind, GTT);
}

result_u32 memory_busy_percent(u32 dv_ind) {
    if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_u32 error = {init.status, 0};
    return error;
  }

  u32 percent;
  rsmi_status_t ret = rsmi_dev_memory_busy_percent_get(dv_ind, &percent);

  result_u32 res = {ret, percent};
  return res;
}