use crate::bindings::pci_bandwidth;

pub struct Pcie {}

impl Pcie {
    pub fn get_pcie() {
        unsafe {
            let pcie = pci_bandwidth(0);
            println!("{:?}", pcie.status);

            println!("{:?}", pcie.current);
            println!("{:?}",pcie.num_supported );
            println!("{:?}",std::slice::from_raw_parts(pcie.lines, pcie.num_supported as usize));
            println!("{:?}",std::slice::from_raw_parts(pcie.frequencies, pcie.num_supported as usize));
        }
    }
}
