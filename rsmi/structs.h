#include <stdint.h>
#include <rocm_smi/rocm_smi.h>

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

typedef struct result_uint64_t
{
  uint16_t status;
  uint64_t data;
} result_uint64_t;


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


typedef struct result_pcie_bandwidth {
  uint16_t status;
} result_pcie_bandwidth;