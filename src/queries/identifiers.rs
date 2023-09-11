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
        let id = device_id(dv_ind).check()?.data;
        let name = device_name(dv_ind).check()?.into_string()?;
        let vendor_id = device_vendor_id(dv_ind).check()?.data;
        let brand = device_brand(dv_ind).check()?.into_string()?;
        let vendor_name = device_vendor_name(dv_ind).check()?.into_string()?;
        let vram_vendor_name = device_vram_vendor_name(dv_ind).check()?.into_string()?;
        let serial_number = device_serial(dv_ind).check()?.into_string()?;
        let subsystem_id = device_subsystem_id(dv_ind).check()?.data;
        let subsystem_name = device_subsystem_name(dv_ind).check()?.into_string()?;
        let drm_render_minor = device_drm_render(dv_ind).check()?.data;
        let subsystem_vendor_id = device_subsystem_vendor_id(dv_ind).check()?.data;
        let unique_id_temp = device_unique_id(dv_ind).check();
        let unique_id = {
            match unique_id_temp {
                Ok(res) => Ok(res.data),
                Err(err) => Err(err),
            }
        };

        Ok(Self {
            id,
            name,
            vendor_id,
            brand,
            vendor_name,
            vram_vendor_name,
            serial_number,
            subsystem_id,
            subsystem_name,
            drm_render_minor,
            subsystem_vendor_id,
            unique_id,
        })
    }
}
