use rocm_smi_lib_sys::{bindings::*, error::RocmErr};

#[derive(Debug)]
pub struct Identifiers {
    pub id: u16,
    pub name: String,
    pub vendor_id: u16,
    pub brand: String,
    pub vendor_name: String,
    pub vram_vendor_name: String,
    pub serial_number: Result<String, RocmErr>,
    pub subsystem_id: u16,
    pub subsystem_name: String,
    pub drm_render_minor: u32,
    pub subsystem_vendor_id: u16,
    unique_id: Option<u64>,
}

const NAME_SIZE: usize = 64;

impl Identifiers {
    pub fn get_unique_id(&self) -> Option<u64> {
        self.unique_id
    }

    #[inline(always)]
    pub(crate) unsafe fn get_identifiers(dv_ind: u32) -> Result<Self, RocmErr> {
        let mut id = 0u16;
        rsmi_dev_id_get(dv_ind, &mut id as *mut u16).try_err()?;

        let mut vendor_id = 0u16;
        rsmi_dev_vendor_id_get(dv_ind, &mut vendor_id as *mut u16).try_err()?;

        let mut subsystem_id = 0u16;
        rsmi_dev_subsystem_id_get(dv_ind, &mut subsystem_id as *mut u16).try_err()?;

        let mut drm_render_minor = 0u32;
        rsmi_dev_drm_render_minor_get(dv_ind, &mut drm_render_minor as *mut u32).try_err()?;

        let mut subsystem_vendor_id = 0u16;
        rsmi_dev_subsystem_vendor_id_get(dv_ind, &mut subsystem_vendor_id as *mut u16).try_err()?;

        let mut unique_id_data = 0u64;
        let unique_id = match rsmi_dev_unique_id_get(dv_ind, &mut unique_id_data as *mut u64) {
            RocmErr::RsmiStatusSuccess => Some(unique_id_data),
            _ => None,
        };

        Ok(Self {
            id,
            name: string_from_fn(dv_ind, NAME_SIZE, rsmi_dev_name_get)?,
            vendor_id,
            brand: string_from_fn(dv_ind, NAME_SIZE, rsmi_dev_brand_get)?,
            vendor_name: string_from_fn(dv_ind, NAME_SIZE, rsmi_dev_vendor_name_get)?,
            vram_vendor_name: string_from_fn(dv_ind, NAME_SIZE, rsmi_dev_vram_vendor_get)?,
            serial_number: string_from_fn(dv_ind, NAME_SIZE, rsmi_dev_serial_number_get),
            subsystem_id,
            subsystem_name: string_from_fn(dv_ind, NAME_SIZE, rsmi_dev_subsystem_name_get)?,
            drm_render_minor,
            subsystem_vendor_id,
            unique_id,
        })
    }
}
