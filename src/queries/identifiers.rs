use rocm_smi_lib_sys::{error::RocmErr, RawRsmi};

#[derive(Debug)]
pub struct Identifiers {
    pub id: Result<u16, RocmErr>,
    pub name: Result<String, RocmErr>,
    pub vendor_id: Result<u16, RocmErr>,
    pub brand: Result<String, RocmErr>,
    pub vendor_name: Result<String, RocmErr>,
    pub vram_vendor_name: Result<String, RocmErr>,
    pub serial_number: Result<String, RocmErr>,
    pub subsystem_id: Result<u16, RocmErr>,
    pub subsystem_name: Result<String, RocmErr>,
    pub drm_render_minor: Result<u32, RocmErr>,
    pub subsystem_vendor_id: Result<u16, RocmErr>,
    pub unique_id: Result<u64, RocmErr>,
}

const NAME_SIZE: usize = 64;

impl Identifiers {
    #[inline(always)]
    pub(crate) unsafe fn get_identifiers(raw: &mut RawRsmi, dv_ind: u32) -> Result<Self, RocmErr> {
        let mut id_data = 0u16;
        let id = raw
            .rsmi_dev_id_get(dv_ind, &mut id_data as *mut u16)
            .try_err()
            .map(|_| id_data);

        let mut vendor_id_data = 0u16;
        let vendor_id = raw
            .rsmi_dev_vendor_id_get(dv_ind, &mut vendor_id_data as *mut u16)
            .try_err()
            .map(|_| vendor_id_data);

        let mut subsystem_id_data = 0u16;
        let subsystem_id = raw
            .rsmi_dev_subsystem_id_get(dv_ind, &mut subsystem_id_data as *mut u16)
            .try_err()
            .map(|_| subsystem_id_data);

        let mut drm_render_minor_data = 0u32;
        let drm_render_minor = raw
            .rsmi_dev_drm_render_minor_get(dv_ind, &mut drm_render_minor_data as *mut u32)
            .try_err()
            .map(|_| drm_render_minor_data);

        let mut temp_subsystem_vendor_id = 0u16;
        let subsystem_vendor_id = match raw
            .rsmi_dev_subsystem_vendor_id_get(dv_ind, &mut temp_subsystem_vendor_id as *mut u16)
            .try_err()
        {
            Ok(_) => Ok(temp_subsystem_vendor_id),
            Err(err) => Err(err),
        };

        let mut unique_id_data = 0u64;
        let unique_id = raw
            .rsmi_dev_unique_id_get(dv_ind, &mut unique_id_data as *mut u64)
            .try_err()
            .map(|_| unique_id_data);

        let name = {
            let buff = libc::malloc(NAME_SIZE).cast();
            raw.rsmi_dev_name_get(dv_ind, buff, NAME_SIZE)
                .try_err()
                .map(|_| {
                    std::ffi::CString::from_raw(buff)
                        .to_string_lossy()
                        .to_string()
                })
        };

        let brand = {
            let buff = libc::malloc(NAME_SIZE).cast();
            raw.rsmi_dev_brand_get(dv_ind, buff, NAME_SIZE)
                .try_err()
                .map(|_| {
                    std::ffi::CString::from_raw(buff)
                        .to_string_lossy()
                        .to_string()
                })
        };

        let vendor_name = {
            let buff = libc::malloc(NAME_SIZE).cast();
            raw.rsmi_dev_vendor_name_get(dv_ind, buff, NAME_SIZE)
                .try_err()
                .map(|_| {
                    std::ffi::CString::from_raw(buff)
                        .to_string_lossy()
                        .to_string()
                })
        };
        let vram_vendor_name = {
            let buff = libc::malloc(NAME_SIZE).cast();
            raw.rsmi_dev_vram_vendor_get(dv_ind, buff, NAME_SIZE)
                .try_err()
                .map(|_| {
                    std::ffi::CString::from_raw(buff)
                        .to_string_lossy()
                        .to_string()
                })
        };
        let serial_number = {
            let buff = libc::malloc(NAME_SIZE).cast();
            raw.rsmi_dev_serial_number_get(dv_ind, buff, NAME_SIZE)
                .try_err()
                .map(|_| {
                    std::ffi::CString::from_raw(buff)
                        .to_string_lossy()
                        .to_string()
                })
        };

        let subsystem_name = {
            let buff = libc::malloc(NAME_SIZE).cast();
            raw.rsmi_dev_subsystem_name_get(dv_ind, buff, NAME_SIZE)
                .try_err()
                .map(|_| {
                    std::ffi::CString::from_raw(buff)
                        .to_string_lossy()
                        .to_string()
                })
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
