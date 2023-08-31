#include <stdint.h>
#include <rocm_smi/rocm_smi.h>

typedef struct init_status
{
  rsmi_status_t status;
  uint8_t initiated;
} init_status;

typedef struct result_str
{
  uint16_t status;
  char *data;
} result_str;

typedef struct result_uint64_t
{
  uint16_t status;
  uint64_t data;
} result_uint64_t;

typedef struct result_int64_t
{
  uint16_t status;
  int64_t data;
} result_int64_t;

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

typedef struct result_pcie_bandwidth
{
  uint16_t status;
  uint32_t current_index;
  uint32_t num_supported;
  uint32_t *lines;
  uint64_t *frequencies;
} result_pcie_bandwidth;

typedef struct result_pcie_throughput
{
  uint16_t status;
  uint64_t sent;
  uint64_t recived;
  uint64_t max_pkg_size;
} result_pcie_throughput;

typedef struct result_power {
  uint16_t status;
  uint16_t sensors;
  uint64_t default_power_cap;
  uint64_t *power_per_sensor;
  uint64_t *power_cap_per_sensor;
  uint64_t *power_cap_min_sensor;
  uint64_t *power_cap_max_sensor;
} result_power;

typedef struct result_uint64_t_dual
{
  uint16_t status;
  uint64_t data1;
  uint64_t data2;
} result_uint64_t_dual;