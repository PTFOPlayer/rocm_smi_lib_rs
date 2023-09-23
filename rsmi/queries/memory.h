#include <stdint.h>
#include <stdlib.h>
#include <rocm_smi/rocm_smi.h>

#ifndef STRUCTS_H
#define STRUCTS_H
#include "../structs.h"
#endif

#define VRAM RSMI_MEM_TYPE_VRAM 
#define VIS_VRAM RSMI_MEM_TYPE_VIS_VRAM
#define GTT RSMI_MEM_TYPE_GTT

result_uint64_t mem_total(uint32_t dv_ind, rsmi_memory_type_t mem_type) {

  uint64_t ammount;
  rsmi_status_t ret = rsmi_dev_memory_total_get(dv_ind, mem_type, &ammount);

  result_uint64_t res = {ret, ammount};
  return res;
}

result_uint64_t mem_used(uint32_t dv_ind, rsmi_memory_type_t mem_type) {
  uint64_t ammount;
  rsmi_status_t ret = rsmi_dev_memory_usage_get(dv_ind, mem_type, &ammount);

  result_uint64_t res = {ret, ammount};
  return res;
}

result_uint64_t mem_total_vram(uint32_t dv_ind) {
  return mem_total(dv_ind, VRAM);
}

result_uint64_t mem_total_vis_vram(uint32_t dv_ind) {
  return mem_total(dv_ind, VIS_VRAM);
}

result_uint64_t mem_total_gtt(uint32_t dv_ind) {
  return mem_total(dv_ind, GTT);
}

result_uint64_t mem_used_vram(uint32_t dv_ind) {
  return mem_used(dv_ind, VRAM);
}

result_uint64_t mem_used_vis_vram(uint32_t dv_ind) {
  return mem_used(dv_ind, VIS_VRAM);
}

result_uint64_t mem_used_gtt(uint32_t dv_ind) {
  return mem_used(dv_ind, GTT);
}

result_uint32_t memory_busy_percent(uint32_t dv_ind) {
  uint32_t percent;
  rsmi_status_t ret = rsmi_dev_memory_busy_percent_get(dv_ind, &percent);

  result_uint32_t res = {ret, percent};
  return res;
}