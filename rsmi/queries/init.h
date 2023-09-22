#include <stdint.h>
#include <stdlib.h>
#include <rocm_smi/rocm_smi.h>

#ifndef STRUCTS_H
#define STRUCTS_H
#include "../structs.h"
#endif

#define NameSize 64

init_status init = {RSMI_STATUS_INIT_ERROR, 0};

uint16_t init_c()
{
  if (!init.initiated)
  {
    init.status = rsmi_init(0);
    init.initiated = 1;
  }
  return init.status;
}

uint16_t shutdown_c()
{
  if (!init.initiated)
  {
    init.status = rsmi_shut_down();
    init.initiated = 0;
    if (init.status == 0)
      init.status = RSMI_STATUS_INIT_ERROR;
  }
  return init.status;
}