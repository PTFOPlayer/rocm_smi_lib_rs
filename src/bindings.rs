#[link(name = "rsmi64", kind = "static")]
extern "C" {
    pub fn init_c() -> u16;
    pub fn num_devices() -> ResultUint32T;
    pub fn device_id(dev_id: u32) -> ResultUint16T;
    pub fn device_name(dev_id: u32) -> ResultStr;
    pub fn device_vendor_id(dev_id: u32) -> ResultUint16T;
    pub fn device_brand(dev_id: u32) -> ResultStr;
    pub fn device_vendor_name(dev_id: u32) -> ResultStr;
    pub fn device_vram_vendor_name(dev_id: u32) -> ResultStr;
    pub fn device_serial(dev_id: u32) -> ResultStr;
    pub fn device_subsystem_id(dev_id:  u32) -> ResultUint16T;
    pub fn device_subsystem_name(dev_id:  u32) -> ResultStr;
    pub fn device_drm_render(dev_id:  u32) -> ResultUint32T;
    pub fn device_subsystem_vendor_id(dev_id:  u32) -> ResultUint16T;
    pub fn device_unique_id(dev_id:  u32) -> ResultUint64T;
}

use libc::c_char;

#[repr(C)]
pub struct ResultStr {
    pub status: u16,
    pub data: *mut c_char,
}

#[repr(C)]
pub struct ResultUint64T {
    pub status: u16,
    pub data: u64,
}

#[repr(C)]
pub struct ResultUint32T {
    pub status: u16,
    pub data: u32,
}

#[repr(C)]
pub struct ResultUint16T {
    pub status: u16,
    pub data: u16,
}
