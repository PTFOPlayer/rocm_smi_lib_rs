#include <stdint.h>
#include <stdlib.h>
#include <rocm_smi/rocm_smi.h>

#ifndef STRUCTS_H
#define STRUCTS_H
#include "../structs.h"
#endif

#ifndef IDENTIFIER_H
#define IDENTIFIER_H
#include "./identifier.h"
#endif

result_pcie_bandwidth pci_bandwidth(uint32_t dv_ind) {
    if (init.done != 1 && init.status != RSMI_STATUS_SUCCESS)
  {
    result_pcie_bandwidth error = {0};
    return error;
  }

}