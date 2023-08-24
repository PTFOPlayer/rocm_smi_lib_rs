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

result_u32 num_devices()
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_u32 error = {init.status, 0};
    return error;
  }

  u32 num;
  rsmi_status_t ret = rsmi_num_monitor_devices(&num);

  result_u32 res = {ret, num};
  return res;
}

result_u16 device_id(u32 dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_u16 error = {init.status, 0};
    return error;
  }

  u16 id;
  rsmi_status_t ret = rsmi_dev_id_get(dv_ind, &id);

  result_u16 res = {ret, id};
  return res;
}

result_u16 device_vendor_id(u32 dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_u16 error = {init.status, 0};
    return error;
  }
  u16 id;
  rsmi_status_t ret = rsmi_dev_vendor_id_get(dv_ind, &id);

  result_u16 res = {ret, id};
  return res;
}

result_str device_name(u32 dv_ind)
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

result_str device_brand(u32 dv_ind)
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

result_str device_vendor_name(u32 dv_ind)
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

result_str device_vram_vendor_name(u32 dv_ind)
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

result_str device_serial(u32 dv_ind)
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

result_u16 device_subsystem_id(u32 dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_u16 error = {init.status, 0};
    return error;
  }
  u16 id;
  rsmi_status_t ret = rsmi_dev_subsystem_id_get(dv_ind, &id);

  result_u16 res = {ret, id};
  return res;
}

result_str device_subsystem_name(u32 dv_ind)
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

result_u32 device_drm_render(u32 dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_u32 error = {init.status, 0};
    return error;
  }
  u32 id;
  rsmi_status_t ret = rsmi_dev_drm_render_minor_get(dv_ind, &id);

  result_u32 res = {ret, id};
  return res;
}

result_u16 device_subsystem_vendor_id(u32 dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_u16 error = {init.status, 0};
    return error;
  }
  u16 id;
  rsmi_status_t ret = rsmi_dev_subsystem_vendor_id_get(dv_ind, &id);

  result_u16 res = {ret, id};
  return res;
}

result_u64 device_unique_id(u32 dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_u64 error = {init.status, 0};
    return error;
  }

  u64 id;
  rsmi_status_t ret = rsmi_dev_unique_id_get(dv_ind, &id);

  result_u64 res = {ret, id};
  return res;
}