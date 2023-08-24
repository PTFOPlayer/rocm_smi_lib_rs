#include <stdint.h>
#include <rocm_smi/rocm_smi.h>

#define u8 uint8_t
#define u16 uint16_t
#define u32 uint32_t
#define u64 uint64_t
#define i64 int64_t

typedef struct init_status
{
  rsmi_status_t status;
  u8 initiated;
} init_status;

typedef struct result_str
{
  u16 status;
  char *data;
} result_str;

typedef struct result_u64
{
  u16 status;
  u64 data;
} result_u64;

typedef struct result_i64
{
  u16 status;
  i64 data;
} result_i64;

typedef struct result_u32
{
  u16 status;
  u32 data;
} result_u32;

typedef struct result_u16
{
  u16 status;
  u16 data;
} result_u16;

typedef struct result_pcie_bandwidth
{
  u16 status;
  u32 current_index;
  u32 num_supported;
  u32 *lines;
  u64 *frequencies;
} result_pcie_bandwidth;

typedef struct result_pcie_throughput
{
  u16 status;
  u64 sent;
  u64 recived;
  u64 max_pkg_size;
} result_pcie_throughput;

typedef struct result_u64_dual
{
  u16 status;
  u64 data1;
  u64 data2;
} result_u64_dual;