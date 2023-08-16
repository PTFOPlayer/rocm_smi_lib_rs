#include <stdint.h>
#include <stdlib.h>
#include <rocm_smi/rocm_smi.h>
#include <stdio.h>

#ifndef STRUCTS_H
#define STRUCTS_H
#include "../structs.h"
#endif

#ifndef IDENTIFIER_H
#define IDENTIFIER_H
#include "./identifier.h"
#endif

result_pcie_bandwidth pci_bandwidth(uint32_t dv_ind)
{
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_pcie_bandwidth error = {init.status, 0};
    return error;
  }

  rsmi_pcie_bandwidth_t bandwidth;

  rsmi_status_t ret = rsmi_dev_pci_bandwidth_get(dv_ind, &bandwidth);

  uint32_t supported = bandwidth.transfer_rate.num_supported;
  uint32_t current_indx = bandwidth.transfer_rate.current;

  uint32_t *lines = (uint32_t*)malloc(sizeof(uint32_t) * supported);
  uint64_t *frequencies = (uint64_t*)malloc(sizeof(uint64_t) * supported);

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

result_uint64_t pcie_id(uint32_t dv_ind) {
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_uint64_t error = {init.status, 0};
    return error;
  }

  uint64_t id;
  rsmi_status_t ret = rsmi_dev_pci_id_get(dv_ind, &id);

  result_uint64_t res = {ret, id};
  return res;
}

result_uint32_t topo_numa_affinity(uint32_t dv_ind) {
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_uint32_t error = {init.status, 0};
    return error;
  }

  uint32_t id;
  rsmi_status_t ret = rsmi_topo_numa_affinity_get(dv_ind, &id);

  result_uint32_t res = {ret, id};
  return res;
}

result_pcie_throughput pci_throughput(uint32_t dv_ind) {
  if (init.status != RSMI_STATUS_SUCCESS)
  {
    result_pcie_throughput error = {init.status, 0, 0, 0};
    return error;
  }

  result_pcie_throughput res = {0, 0, 0, 0};

  res.status = rsmi_dev_pci_throughput_get(dv_ind, &res.sent, &res.recived, &res.max_pkg_size); 
  
  return res;
}