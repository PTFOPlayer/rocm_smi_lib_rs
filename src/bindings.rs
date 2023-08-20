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
    pub(crate) fn pcie_id(dev_id: u32) -> ResultUint64T;
    pub(crate) fn topo_numa_affinity(dev_id: u32) -> ResultUint32T;
    pub(crate) fn pci_throughput(dev_id: u32) -> ResultPcieThroughput;

    // power
    pub(crate) fn power_sensor_count(dv_ind: u32) -> ResultUint16T;
    pub(crate) fn power_sensor(dv_ind: u32, sensor: u16) -> ResultUint64T;
    pub(crate) fn power_avg_all(dv_ind: u32) -> ResultUint64T;
    pub(crate) fn power_cap(dv_ind: u32, sensor: u16) -> ResultUint64T;
    pub(crate) fn default_power_cap(dv_ind: u32) -> ResultUint64T;
    pub(crate) fn power_cap_range(dv_ind: u32) -> ResultUint64TDual;
}

use libc::c_char;

use crate::error::RocmErr;

#[repr(C)]
pub(crate) struct ResultStr {
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

#[repr(C)]
pub(crate) struct ResultUint64TDual {
    pub(crate) status: u16,
    pub(crate) data1: u64,
    pub(crate) data2: u64,
}

#[inline(always)]
pub(crate) fn check_res(status: u16) -> Result<(), RocmErr> {
    if status != 0 {
        return Err(RocmErr::from_u16(status));
    }
    Ok(())
}

#[inline(always)]
pub(crate) fn string_from_ptr(ptr: *mut i8) -> Result<String, RocmErr> {
    let c_str = unsafe { std::ffi::CStr::from_ptr(ptr) };
    let data = c_str.to_str().to_owned();
    match data {
        Ok(res) => Ok(res.to_owned()),
        Err(_) => Err(RocmErr::RsmiStringConversionError),
    }
}
