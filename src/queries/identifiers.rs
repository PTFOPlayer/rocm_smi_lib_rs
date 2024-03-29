use rocm_smi_lib_sys::{error::RocmErr, RawRsmi};

use crate::{MapWithString, RocmSmi};

#[derive(Debug)]
pub struct Identifiers {
    pub id: Result<u16, RocmErr>,
    pub revision: Result<u16, RocmErr>,
    pub vendor_id: Result<u16, RocmErr>,
    pub name: Result<String, RocmErr>,
    pub brand: Result<String, RocmErr>,
    pub vendor_name: Result<String, RocmErr>,
    pub vram_vendor_name: Result<String, RocmErr>,
    pub serial_number: Result<String, RocmErr>,
    pub subsystem_id: Result<u16, RocmErr>,
    pub subsystem_name: Result<String, RocmErr>,
    pub drm_render_minor: Result<u32, RocmErr>,
    pub subsystem_vendor_id: Result<u16, RocmErr>,
    pub unique_id: Result<u64, RocmErr>,
    pub xgmi_physical_id: Result<u16, RocmErr>,
}

const NAME_SIZE: usize = 64;

impl RocmSmi {
    /// # Functionality
    ///
    /// This function returns identifiers for given device.
    /// example:
    /// ```rust,no_compile,ignore
    /// use rocm_smi_lib::RocmSmi;
    /// use rocm_smi_lib::error::RocmErr;
    /// fn print_gpu_name() -> Result<(), RocmErr> {
    ///     let rocm = RocmSmi::init()?;
    ///     let name = rocm.get_device_identifiers(0)?.name;
    ///     println!("{}", name);
    ///     Ok(())
    /// }
    /// ```
    /// for example for RX 7600 will print you:
    /// ```no_compile,ignore
    /// Navi 33 [Radeon RX 7700S/7600/7600S/7600M XT/PRO W7600]
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if `dv_ind` id not valid device identifier.
    pub fn get_device_identifiers(&mut self, dv_ind: u32) -> Result<Identifiers, RocmErr> {
        let raw: &mut RawRsmi = &mut self.raw;
        let mut id_data = 0u16;
        let id = unsafe {
            raw.rsmi_dev_id_get(dv_ind, &mut id_data as *mut u16)
                .try_err()
                .map(|_| id_data)
        };

        let mut revision_data = 0u16;
        let revision = unsafe {
            raw.rsmi_dev_revision_get(dv_ind, &mut revision_data as *mut u16)
                .try_err()
                .map(|_| revision_data)
        };

        let mut vendor_id_data = 0u16;
        let vendor_id = unsafe {
            raw.rsmi_dev_vendor_id_get(dv_ind, &mut vendor_id_data as *mut u16)
                .try_err()
                .map(|_| vendor_id_data)
        };

        let mut subsystem_id_data = 0u16;
        let subsystem_id = unsafe {
            raw.rsmi_dev_subsystem_id_get(dv_ind, &mut subsystem_id_data as *mut u16)
                .try_err()
                .map(|_| subsystem_id_data)
        };

        let mut drm_render_minor_data = 0u32;
        let drm_render_minor = unsafe {
            raw.rsmi_dev_drm_render_minor_get(dv_ind, &mut drm_render_minor_data as *mut u32)
                .try_err()
                .map(|_| drm_render_minor_data)
        };

        let mut temp_subsystem_vendor_id = 0u16;
        let subsystem_vendor_id = unsafe {
            raw.rsmi_dev_subsystem_vendor_id_get(dv_ind, &mut temp_subsystem_vendor_id as *mut u16)
                .try_err()
                .map(|_| temp_subsystem_vendor_id)
        };

        let mut unique_id_data = 0u64;
        let unique_id = unsafe {
            raw.rsmi_dev_unique_id_get(dv_ind, &mut unique_id_data as *mut u64)
                .try_err()
                .map(|_| unique_id_data)
        };

        let name = unsafe {
            let buff = libc::malloc(NAME_SIZE).cast();
            raw.rsmi_dev_name_get(dv_ind, buff, NAME_SIZE)
                .map_with_buff(buff)
        };

        let brand = unsafe {
            let buff = libc::malloc(NAME_SIZE).cast();
            raw.rsmi_dev_brand_get(dv_ind, buff, NAME_SIZE)
                .map_with_buff(buff)
        };

        let vendor_name = unsafe {
            let buff = libc::malloc(NAME_SIZE).cast();
            raw.rsmi_dev_vendor_name_get(dv_ind, buff, NAME_SIZE)
                .map_with_buff(buff)
        };
        let vram_vendor_name = unsafe {
            let buff = libc::malloc(NAME_SIZE).cast();
            raw.rsmi_dev_vram_vendor_get(dv_ind, buff, NAME_SIZE)
                .map_with_buff(buff)
        };
        let serial_number = unsafe {
            let buff = libc::malloc(NAME_SIZE).cast();
            raw.rsmi_dev_serial_number_get(dv_ind, buff, NAME_SIZE)
                .map_with_buff(buff)
        };

        let subsystem_name = unsafe {
            let buff = libc::malloc(NAME_SIZE).cast();
            raw.rsmi_dev_subsystem_name_get(dv_ind, buff, NAME_SIZE)
                .map_with_buff(buff)
        };

        let mut xgmi_physical_id_data = 0u16;
        let xgmi_physical_id = unsafe {
            raw.rsmi_dev_xgmi_physical_id_get(dv_ind, &mut xgmi_physical_id_data as *mut u16)
                .try_err()
                .map(|_| xgmi_physical_id_data)
        };

        Ok(Identifiers {
            id,
            revision,
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
            xgmi_physical_id,
        })
    }
}
