use rocm_smi_lib_sys::{bindings::*, error::RocmErr, RawRsmi};
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

impl Pcie {
    #[inline(always)]
    pub(crate) fn get_pcie(raw: &mut RawRsmi, dv_ind: u32) -> Result<Self, RocmErr> {
        unsafe {
            let bandwidth = &mut RsmiPcieBandwidth::default();
            raw.rsmi_dev_pci_bandwidth_get(dv_ind, bandwidth as *mut RsmiPcieBandwidth).try_err()?;

            let mut id = 0u64;
            raw.rsmi_dev_pci_id_get(dv_ind, &mut id as *mut u64).try_err()?;

            let mut numa = 0u32;
            raw.rsmi_topo_numa_affinity_get(dv_ind, &mut numa as *mut u32);

            let mut pkg_sent = 0u64;
            let mut pkg_recived = 0u64;
            let mut max_pkg_size = 0u64;
            raw.rsmi_dev_pci_throughput_get(
                dv_ind,
                &mut pkg_sent as *mut u64,
                &mut pkg_recived as *mut u64,
                &mut max_pkg_size as *mut u64,
            )
            .try_err()?;

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
