#include <stdint.h>
#include <rocm_smi/rocm_smi.h>
#include <stdio.h>

#define NameSize 64

typedef struct init_status
{
  int done;
  rsmi_status_t status;
} init_status;

typedef struct result_str
{
  uint16_t status;
  char *data;
} result_str;

typedef struct result_uint32_t
{
  uint16_t status;
  uint32_t data;
} result_uint32_t;

typedef struct result_uint16_t
{
  uint16_t status;
  uint16_t data;
} result_uint16_t;

init_status init = {0, 0};

uint16_t init_c()
{
  init.status = rsmi_init(0);
  if (init.status == RSMI_STATUS_SUCCESS)
  {
    init.done = 1;
  }
  return init.status;
}

result_uint32_t num_devices()
{
  if (init.done != 1 && init.status != RSMI_STATUS_SUCCESS)
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
  if (init.done != 1 && init.status != RSMI_STATUS_SUCCESS)
  {
    result_uint16_t error = {init.status, 0};
    return error;
  }

  uint16_t id;
  rsmi_status_t ret = rsmi_dev_id_get(dv_ind, &id);

  result_uint16_t res = {ret, id};
  return res;
}

result_uint16_t device_vendor_id(uint32_t dv_ind) {
  if (init.done != 1 && init.status != RSMI_STATUS_SUCCESS)
  {
    result_uint16_t error = {init.status, 0};
    return error;
  }
  uint16_t id;
  rsmi_status_t ret = rsmi_dev_vendor_id_get(dv_ind, &id);

  result_uint16_t res = {ret, id};
  return res;
}

result_str device_name(uint32_t dv_ind) {
  if (init.done != 1 && init.status != RSMI_STATUS_SUCCESS)
  {
    result_str error = {init.status, ""};
    return error;
  }

  char *name = (char*)malloc(NameSize*sizeof(char));
  rsmi_status_t ret = rsmi_dev_name_get(dv_ind, name, NameSize);

  result_str res = {ret, name};
  return res; 
}

result_str 	device_brand (uint32_t dv_ind) {
  if (init.done != 1 && init.status != RSMI_STATUS_SUCCESS)
  {
    result_str error = {init.status, ""};
    return error;
  }

  char *brand = (char*)malloc(NameSize*sizeof(char));
  rsmi_status_t ret = rsmi_dev_brand_get(dv_ind, brand, NameSize);

  result_str res = {ret, brand};
  return res; 
}

result_str 	device_vendor_name (uint32_t dv_ind) {
  if (init.done != 1 && init.status != RSMI_STATUS_SUCCESS)
  {
    result_str error = {init.status, ""};
    return error;
  }

  char *vendor = (char*)malloc(NameSize*sizeof(char));
  rsmi_status_t ret = rsmi_dev_vendor_name_get(dv_ind, vendor, NameSize);

  result_str res = {ret, vendor};
  return res; 
}