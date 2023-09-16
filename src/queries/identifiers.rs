use crate::{bindings::*, error::RocmErr};

#[derive(Debug)]
pub struct Identifiers {
    pub id: u16,
    pub name: String,
    pub vendor_id: u16,
    pub brand: String,
    pub vendor_name: String,
    pub vram_vendor_name: String,
    pub serial_number: String,
    pub subsystem_id: u16,
    pub subsystem_name: String,
    pub drm_render_minor: u32,
    pub subsystem_vendor_id: u16,
    unique_id: Result<u64, RocmErr>,
}

impl Identifiers {
    pub fn get_unique_id(&self) -> Result<u64, RocmErr> {
        self.unique_id
    }

    pub(crate) unsafe fn get_identifiers(dv_ind: u32) -> Result<Self, RocmErr> {
        Ok(Self {
            id: device_id(dv_ind).check()?.data,
            name: device_name(dv_ind).check()?.into_string()?,
            vendor_id: device_vendor_id(dv_ind).check()?.data,
            brand: device_brand(dv_ind).check()?.into_string()?,
            vendor_name: device_vendor_name(dv_ind).check()?.into_string()?,
            vram_vendor_name: device_vram_vendor_name(dv_ind).check()?.into_string()?,
            serial_number: device_serial(dv_ind).check()?.into_string()?,
            subsystem_id: device_subsystem_id(dv_ind).check()?.data,
            subsystem_name: device_subsystem_name(dv_ind).check()?.into_string()?,
            drm_render_minor: device_drm_render(dv_ind).check()?.data,
            subsystem_vendor_id: device_subsystem_vendor_id(dv_ind).check()?.data,
            unique_id: match device_unique_id(dv_ind).check() {
                Ok(res) => Ok(res.data),
                Err(err) => Err(err),
            },
        })
    }
}
