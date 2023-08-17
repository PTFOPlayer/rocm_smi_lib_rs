use crate::{bindings::*, error::RocmErr};
use std::slice::from_raw_parts;
#[derive(Debug)]
pub struct Pcie<'a> {
    pub(crate) id: u64,
    pub(crate) associated_numa_node: u32,
    pub(crate) current_index: u32,
    pub(crate) lines: &'a [u32],
    pub(crate) frequencies: &'a [u64],
    pub(crate) pkg_sent: u64,
    pub(crate) pkg_recived: u64,
    pub(crate) max_pkg_size: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct PcieBandwidthAndThroughput {
    pub lines: u32,
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

impl Pcie<'_> {
    #[inline(always)]
    pub(crate) fn get_pcie(dev_id: u32) -> Result<Self, RocmErr> {
        unsafe {
            let bandwidth = pci_bandwidth(dev_id);
            let id = pcie_id(dev_id);
            let numa = topo_numa_affinity(dev_id);
            let throughput = pci_throughput(dev_id);

            check_res(bandwidth.status)?;
            check_res(id.status)?;
            check_res(numa.status)?;
            check_res(throughput.status)?;

            Ok(Self {
                id: id.data,
                associated_numa_node: numa.data,
                current_index: bandwidth.current,
                lines: from_raw_parts(bandwidth.lines, bandwidth.num_supported as usize),
                frequencies: from_raw_parts(
                    bandwidth.frequencies,
                    bandwidth.num_supported as usize,
                ),
                pkg_sent: throughput.sent,
                pkg_recived: throughput.recived,
                max_pkg_size: throughput.max_pkg_size,
            })
        }
    }

    pub fn get_bandwidth_and_throughput(&self) -> PcieBandwidthAndThroughput {
        PcieBandwidthAndThroughput {
            lines: self.lines[self.current_index as usize],
            frequency: self.frequencies[self.current_index as usize],
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
