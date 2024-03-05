use std::ptr::null;

use rocm_smi_lib_sys::{bindings::RsmiVersion, error::RocmErr};

use crate::RocmSmi;

impl RocmSmi {
    pub fn get_rsmi_version(&mut self) -> Result<String, RocmErr> {
        let mut v = RsmiVersion {
            major: 0,
            minor: 0,
            patch: 0,
            build: null::<i8>() as *mut i8,
        };

        unsafe {
            self.raw
                .rsmi_version_get(&mut v as *mut RsmiVersion)
                .try_err()?;
        }
        Ok(format!(
            "version: {}.{}, patch: {}",
            v.major, v.minor, v.patch
        ))
    }
}
