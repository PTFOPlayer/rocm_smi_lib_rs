use rocm_smi_lib_sys::{bindings::*, error::RocmErr, RawRsmi};

use crate::RocmSmi;
#[derive(Debug)]
pub struct Pcie {
    pub id: u64,
    pub associated_numa_node: u32,
    pub current_index: u32,
    pub lanes: Vec<u32>,
    pub frequency: Vec<u64>,
    pub pkg_sent: u64,
    pub pkg_recived: u64,
    pub max_pkg_size: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct PcieBandwidthAndThroughput {
    pub lanes: u32,
    pub frequency: u64,
    pub pkg_sent: u64,
    pub pkg_recived: u64,
    pub max_pkg_size: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct PcieIdentifiers {
    pub id: u64,
    pub associated_numa_node: u32,
}

impl RocmSmi {
    /// # Functionality
    ///
    /// This function returns pcie information for given device.
    /// example:
    /// ```rust,no_compile,ignore
    /// use rocm_smi_lib::RocmSmi;
    /// use rocm_smi_lib::error::RocmErr;
    /// fn print_gpu_pcie_lines() -> Result<(), RocmErr> {
    ///     let rocm = RocmSmi::init()?;
    ///     let lines = rocm.get_device_pcie_data(0)?.get_bandwidth_and_throughput().lines;
    ///     println!("{}", lines);
    ///     Ok(())
    /// }
    /// ```
    /// for example for RX 7600 will print you:
    /// ```no_compile,ignore
    /// 8
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if `dv_ind` id not valid device identifier.
    pub fn get_device_pcie_data<'a>(&mut self, dv_ind: u32) -> Result<Pcie, RocmErr> {
        let raw: &mut RawRsmi = &mut self.raw;

        let bandwidth = &mut RsmiPcieBandwidth::default();

        let mut id = 0u64;

        let mut numa = 0u32;

        let mut pkg_sent = 0u64;
        let mut pkg_recived = 0u64;
        let mut max_pkg_size = 0u64;

        unsafe {
            raw.rsmi_dev_pci_bandwidth_get(dv_ind, bandwidth as *mut RsmiPcieBandwidth)
                .try_err()?;
            raw.rsmi_dev_pci_id_get(dv_ind, &mut id as *mut u64)
                .try_err()?;
            raw.rsmi_topo_numa_affinity_get(dv_ind, &mut numa as *mut u32);
            raw.rsmi_dev_pci_throughput_get(
                dv_ind,
                &mut pkg_sent as *mut u64,
                &mut pkg_recived as *mut u64,
                &mut max_pkg_size as *mut u64,
            )
            .try_err()?;
        }

        let len = bandwidth.transfer_rate.num_supported as usize;
        Ok(Pcie {
            id,
            associated_numa_node: numa,
            current_index: bandwidth.transfer_rate.current,
            lanes: bandwidth.lanes[0..len].to_vec(),
            frequency: bandwidth.transfer_rate.frequency[0..len].to_vec(),
            pkg_sent,
            pkg_recived,
            max_pkg_size,
        })
    }
}

impl Pcie {
    pub fn get_bandwidth_and_throughput(&self) -> PcieBandwidthAndThroughput {
        PcieBandwidthAndThroughput {
            lanes: self.lanes[self.current_index as usize],
            frequency: self.frequency[self.current_index as usize],
            pkg_sent: self.pkg_sent,
            pkg_recived: self.pkg_recived,
            max_pkg_size: self.max_pkg_size,
        }
    }

    pub fn get_pcie_identifiers(&self) -> PcieIdentifiers {
        PcieIdentifiers {
            id: self.id,
            associated_numa_node: self.associated_numa_node,
        }
    }
}
