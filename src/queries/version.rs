use std::ptr::null;

use rocm_smi_lib_sys::bindings::{rsmi_version_get, rsmi_version_t};

use crate::{
    error::{IntoRocmErr, RocmErr},
    RocmSmi,
};

pub type RsmiVersion = rsmi_version_t;

impl RocmSmi {
    pub fn get_rsmi_version(&mut self) -> Result<String, RocmErr> {
        let mut v = RsmiVersion {
            major: 0,
            minor: 0,
            patch: 0,
            build: null::<i8>() as *mut i8,
        };
        

        unsafe {
            rsmi_version_get(&mut v as *mut RsmiVersion).into_rocm_err()?;
        }
        Ok(format!(
            "version: {}.{}, patch: {}",
            v.major, v.minor, v.patch
        ))
    }
}
