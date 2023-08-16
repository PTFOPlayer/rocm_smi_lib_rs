use crate::{bindings::pci_bandwidth, error::RocmErr};
use std::slice::from_raw_parts;
#[derive(Debug)]
pub struct Pcie<'a> {
    pub current_index: u32,
    pub lines: &'a [u32],
    pub frequencies: &'a [u64],
}

#[derive(Debug)]
pub struct CurrentPcie {
    pub lines: u32,
    pub frequency: u64,
}

impl Pcie<'_> {
    pub(crate) fn get_pcie(dev_id: u32) -> Result<Self, RocmErr> {
        unsafe {
            let pcie = pci_bandwidth(dev_id);

            if pcie.status != 0 {
                return Err(RocmErr::from_u16(pcie.status));
            }
            Ok(Self {
                current_index: pcie.current,
                lines: from_raw_parts(pcie.lines, pcie.num_supported as usize),
                frequencies: from_raw_parts(pcie.frequencies, pcie.num_supported as usize),
            })
        }
    }

    pub fn get_current(&self) -> CurrentPcie {
        CurrentPcie {
            lines: self.lines[self.current_index as usize],
            frequency: self.frequencies[self.current_index as usize],
        }
    }
}
