#[link(name = "rsmi64", kind = "static")]
extern "C" {
    pub fn init_c() -> u16;
    pub fn num_devices() -> result_uint32_t;
    pub fn device_id(dev_id: u32) -> result_uint16_t;
    pub fn device_name(dev_id: u32) -> result_str;
    pub fn device_vendor_id(dev_id: u32) -> result_uint16_t;
    pub fn device_brand(dev_id: u32) -> result_str;
    pub fn device_vendor_name(dev_id: u32) -> result_str;
}

use libc::c_char;

#[repr(C)]
pub struct result_str {
    pub status: u16,
    pub data: *mut c_char,
}

#[repr(C)]
pub struct result_uint32_t {
    pub status: u16,
    pub data: u32,
}

#[repr(C)]
pub struct result_uint16_t {
    pub status: u16,
    pub data: u16,
}
