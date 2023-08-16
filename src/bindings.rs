#[link(name = "rsmi64", kind = "static")]
extern "C" {
    // identifiers
    pub(crate) fn init_c() -> u16;
    pub(crate) fn num_devices() -> ResultUint32T;
    pub(crate) fn device_id(dev_id: u32) -> ResultUint16T;
    pub(crate) fn device_name(dev_id: u32) -> ResultStr;
    pub(crate) fn device_vendor_id(dev_id: u32) -> ResultUint16T;
    pub(crate) fn device_brand(dev_id: u32) -> ResultStr;
    pub(crate) fn device_vendor_name(dev_id: u32) -> ResultStr;
    pub(crate) fn device_vram_vendor_name(dev_id: u32) -> ResultStr;
    pub(crate) fn device_serial(dev_id: u32) -> ResultStr;
    pub(crate) fn device_subsystem_id(dev_id: u32) -> ResultUint16T;
    pub(crate) fn device_subsystem_name(dev_id: u32) -> ResultStr;
    pub(crate) fn device_drm_render(dev_id: u32) -> ResultUint32T;
    pub(crate) fn device_subsystem_vendor_id(dev_id: u32) -> ResultUint16T;
    pub(crate) fn device_unique_id(dev_id: u32) -> ResultUint64T;
    // pcie
    pub(crate) fn pci_bandwidth(dev_id: u32) -> ResultPcieBandwidth;
    pub(crate) fn pcie_id(dev_id:u32) -> ResultUint64T;
    pub(crate) fn topo_numa_affinity(dev_id:u32) -> ResultUint32T;
    pub(crate) fn pci_throughput(dev_id:u32) -> ResultPcieThroughput;
}

use libc::c_char;

#[repr(C)]
pub(crate)struct ResultStr {
    pub(crate) status: u16,
    pub(crate) data: *mut c_char,
}

#[repr(C)]
pub(crate) struct ResultUint64T {
    pub(crate) status: u16,
    pub(crate) data: u64,
}

#[repr(C)]
pub(crate) struct ResultUint32T {
    pub(crate) status: u16,
    pub(crate) data: u32,
}

#[repr(C)]
pub(crate) struct ResultUint16T {
    pub(crate) status: u16,
    pub(crate) data: u16,
}

#[repr(C)]
pub(crate) struct ResultPcieBandwidth {
    pub(crate) status: u16,
    pub(crate) current: u32,
    pub(crate) num_supported: u32,
    pub(crate) lines: *const u32,
    pub(crate) frequencies: *const u64,
}

#[repr(C)]
pub(crate) struct ResultPcieThroughput {
    pub(crate) status: u16,
    pub(crate) sent: u64,
    pub(crate) recived: u64,
    pub(crate) max_pkg_size: u64,
}
