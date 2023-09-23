#include <stdint.h>
#include <stdlib.h>
#include <rocm_smi/rocm_smi.h>

#ifndef STRUCTS_H
#define STRUCTS_H
#include "../structs.h"
#endif


result_uint64_t pcie_id(uint32_t dv_ind) {
  uint64_t id;
  rsmi_status_t ret = rsmi_dev_pci_id_get(dv_ind, &id);

  result_uint64_t res = {ret, id};
  return res;
}

result_uint32_t topo_numa_affinity(uint32_t dv_ind) {
  uint32_t id;
  rsmi_status_t ret = rsmi_topo_numa_affinity_get(dv_ind, &id);

  result_uint32_t res = {ret, id};
  return res;
}

typedef struct result_pcie_throughput
{
  uint16_t status;
  uint64_t sent;
  uint64_t recived;
  uint64_t max_pkg_size;
} result_pcie_throughput;

result_pcie_throughput pci_throughput(uint32_t dv_ind) {
  uint64_t sent;
  uint64_t received;
  uint64_t max_pkg_size;

  rsmi_status_t ret = rsmi_dev_pci_throughput_get(dv_ind, &sent, &received, &max_pkg_size); 
  
  result_pcie_throughput res = {ret, sent, received, max_pkg_size };

  return res;
}