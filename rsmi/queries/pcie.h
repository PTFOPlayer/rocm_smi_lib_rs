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

result_pcie_bandwidth pci_bandwidth(u32 dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_pcie_bandwidth error = {init.status, 0};
    return error;
  }

  rsmi_pcie_bandwidth_t bandwidth;

  rsmi_status_t ret = rsmi_dev_pci_bandwidth_get(dv_ind, &bandwidth);

  u32 supported = bandwidth.transfer_rate.num_supported;
  u32 current_indx = bandwidth.transfer_rate.current;

  u32 *lines = (u32*)malloc(sizeof(u32) * supported);
  u64 *frequencies = (u64*)malloc(sizeof(u64) * supported);

  for (size_t i = 0; i < supported; i++)
  {
    lines[i] = bandwidth.lanes[i];
  }
  
  for (size_t i = 0; i < supported; i++)
  {
    frequencies[i] = bandwidth.transfer_rate.frequency[i];
  }
  
  result_pcie_bandwidth res = {ret, current_indx, supported, lines, frequencies};
  return res;
}

result_u64 pcie_id(u32 dv_ind) {
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_u64 error = {init.status, 0};
    return error;
  }

  u64 id;
  rsmi_status_t ret = rsmi_dev_pci_id_get(dv_ind, &id);

  result_u64 res = {ret, id};
  return res;
}

result_u32 topo_numa_affinity(u32 dv_ind) {
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_u32 error = {init.status, 0};
    return error;
  }

  u32 id;
  rsmi_status_t ret = rsmi_topo_numa_affinity_get(dv_ind, &id);

  result_u32 res = {ret, id};
  return res;
}

result_pcie_throughput pci_throughput(u32 dv_ind) {
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_pcie_throughput error = {init.status, 0, 0, 0};
    return error;
  }

  u64 sent;
  u64 received;
  u64 max_pkg_size;

  rsmi_status_t ret = rsmi_dev_pci_throughput_get(dv_ind, &sent, &received, &max_pkg_size); 
  
  result_pcie_throughput res = {ret, sent, received, max_pkg_size };

  return res;
}