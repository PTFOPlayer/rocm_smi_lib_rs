use rocm_smi_lib_sys::error::RocmErr;

use crate::RocmSmi;

impl RocmSmi {
    pub fn get_device_topo_numa_node_number(&mut self, dv_ind: u32) -> Result<u32, RocmErr> {
        let mut numa = 0u32;
        unsafe {
            self.raw
                .rsmi_topo_get_numa_node_number(dv_ind, &mut numa as *mut u32)
        }
        .try_err()?;

        Ok(numa)
    }
}
