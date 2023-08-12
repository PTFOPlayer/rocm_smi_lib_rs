#include <stdint.h>
#include <rocm_smi/rocm_smi.h>
#include <stdio.h>

#define NameSize 64

typedef struct init_status {
  int done;
  rsmi_status_t status;
} init_status;

typedef struct result_str {
  int status;
  char *data;
} result_str;
typedef struct result_int {
  int status;
  int data;
} result_int;

init_status init = {0, 0};

int init_c() {
  init.status = rsmi_init(0);
  if(init.status == RSMI_STATUS_SUCCESS) {
    init.done = 1;
  }
  return init.status;  
}

result_int num_devices() {
  if(init.done != 1 && init.status != RSMI_STATUS_SUCCESS) {
    result_int error = {14, 0};
    return error;
  }

  uint32_t num;
  rsmi_status_t ret = rsmi_num_monitor_devices(&num);

  result_int res = {ret, num};
  return res;

}

void basic()
{
  rsmi_status_t ret;
  uint32_t num_devices;

  ret = rsmi_init(0);
  ret = rsmi_num_monitor_devices(&num_devices);

  uint16_t dev_id;
  char *name = (char *)malloc(NameSize * sizeof(char));
  char *vendor = (char *)malloc(NameSize * sizeof(char));

  for (int i = 0; i < num_devices; ++i)
  {
    ret = rsmi_dev_id_get(i, &dev_id);
    ret = rsmi_dev_name_get(i, name, NameSize);
    ret = rsmi_dev_vendor_name_get(i, vendor, NameSize);
    printf("%d \n", dev_id);
    printf("%s \n", name);
    printf("%s \n", vendor);
  }

  free(name);
  free(vendor);

  ret = rsmi_shut_down();
}