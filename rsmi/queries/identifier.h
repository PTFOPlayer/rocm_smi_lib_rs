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

result_uint32_t num_devices()
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_uint32_t error = {init.status, 0};
    return error;
  }

  uint32_t num;
  rsmi_status_t ret = rsmi_num_monitor_devices(&num);

  result_uint32_t res = {ret, num};
  return res;
}

result_uint16_t device_id(uint32_t dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_uint16_t error = {init.status, 0};
    return error;
  }

  uint16_t id;
  rsmi_status_t ret = rsmi_dev_id_get(dv_ind, &id);

  result_uint16_t res = {ret, id};
  return res;
}

result_uint16_t device_vendor_id(uint32_t dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_uint16_t error = {init.status, 0};
    return error;
  }
  uint16_t id;
  rsmi_status_t ret = rsmi_dev_vendor_id_get(dv_ind, &id);

  result_uint16_t res = {ret, id};
  return res;
}

result_str device_name(uint32_t dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_str error = {init.status, ""};
    return error;
  }

  char *name = (char *)malloc(NameSize * sizeof(char));
  rsmi_status_t ret = rsmi_dev_name_get(dv_ind, name, NameSize);

  result_str res = {ret, name};
  return res;
}

result_str device_brand(uint32_t dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_str error = {init.status, ""};
    return error;
  }

  char *brand = (char *)malloc(NameSize * sizeof(char));
  rsmi_status_t ret = rsmi_dev_brand_get(dv_ind, brand, NameSize);

  result_str res = {ret, brand};
  return res;
}

result_str device_vendor_name(uint32_t dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_str error = {init.status, ""};
    return error;
  }

  char *vendor = (char *)malloc(NameSize * sizeof(char));
  rsmi_status_t ret = rsmi_dev_vendor_name_get(dv_ind, vendor, NameSize);

  result_str res = {ret, vendor};
  return res;
}

result_str device_vram_vendor_name(uint32_t dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_str error = {init.status, ""};
    return error;
  }

  char *vendor = (char *)malloc(NameSize * sizeof(char));
  rsmi_status_t ret = rsmi_dev_vram_vendor_get(dv_ind, vendor, NameSize);

  result_str res = {ret, vendor};
  return res;
}

result_str device_serial(uint32_t dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_str error = {init.status, ""};
    return error;
  }

  char *id = (char *)malloc(NameSize * sizeof(char));
  rsmi_status_t ret = rsmi_dev_serial_number_get(dv_ind, id, NameSize);

  result_str res = {ret, id};
  return res;
}

result_uint16_t device_subsystem_id(uint32_t dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_uint16_t error = {init.status, 0};
    return error;
  }
  uint16_t id;
  rsmi_status_t ret = rsmi_dev_subsystem_id_get(dv_ind, &id);

  result_uint16_t res = {ret, id};
  return res;
}

result_str device_subsystem_name(uint32_t dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_str error = {init.status, ""};
    return error;
  }

  char *name = (char *)malloc(NameSize * sizeof(char));
  rsmi_status_t ret = rsmi_dev_subsystem_name_get(dv_ind, name, NameSize);

  result_str res = {ret, name};
  return res;
}

result_uint32_t device_drm_render(uint32_t dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_uint32_t error = {init.status, 0};
    return error;
  }
  uint32_t id;
  rsmi_status_t ret = rsmi_dev_drm_render_minor_get(dv_ind, &id);

  result_uint32_t res = {ret, id};
  return res;
}

result_uint16_t device_subsystem_vendor_id(uint32_t dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_uint16_t error = {init.status, 0};
    return error;
  }
  uint16_t id;
  rsmi_status_t ret = rsmi_dev_subsystem_vendor_id_get(dv_ind, &id);

  result_uint16_t res = {ret, id};
  return res;
}

result_uint64_t device_unique_id(uint32_t dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_uint64_t error = {init.status, 0};
    return error;
  }

  uint64_t id;
  rsmi_status_t ret = rsmi_dev_unique_id_get(dv_ind, &id);

  result_uint64_t res = {ret, id};
  return res;
}