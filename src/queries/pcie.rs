use crate::{bindings::*, error::RocmErr};
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
    pub(crate) fn get_pcie(dv_ind: u32) -> Result<Self, RocmErr> {
        unsafe {
            let mut bandwidth = RsmiPcieBandwidthT::default();
            rsmi_dev_pci_bandwidth_get(dv_ind, &mut bandwidth as *mut RsmiPcieBandwidthT)
                .try_err()?;
            let id = pcie_id(dv_ind).check()?;
            let numa = topo_numa_affinity(dv_ind).check()?;
            let throughput = pci_throughput(dv_ind).check()?;

            let len = bandwidth.transfer_rate.num_supported as usize;
            Ok(Pcie {
                id: id.data,
                associated_numa_node: numa.data,
                current_index: bandwidth.transfer_rate.current,
                lanes: bandwidth.lanes[0..len].to_vec(),
                frequency: bandwidth.transfer_rate.frequency[0..len].to_vec(),
                pkg_sent: throughput.sent,
                pkg_recived: throughput.recived,
                max_pkg_size: throughput.max_pkg_size,
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
