use std::ptr::null;

use rocm_smi_lib_sys::bindings::{rsmi_dev_vbios_version_get, rsmi_sw_component_t_RSMI_SW_COMP_DRIVER, rsmi_version_get, rsmi_version_str_get, rsmi_version_t};

use crate::{
    error::{IntoRocmErr, RocmErr}, MapWithString, RocmSmi
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

    
    pub fn get_rsmi_driver_version(&mut self) -> Result<String, RocmErr> {
        const RSMI_DRIVER_SIZE: usize = 256;    
        unsafe {
            
            let buff = libc::malloc(RSMI_DRIVER_SIZE).cast();
            return Into::<RocmErr>::into(rsmi_version_str_get(rsmi_sw_component_t_RSMI_SW_COMP_DRIVER, buff, RSMI_DRIVER_SIZE as u32)).map_with_buff(buff);
        }
    }

    pub fn get_device_rsmi_vbios_version(&mut self, dv_ind: u32) -> Result<String, RocmErr> {
        const RSMI_VBIOS_SIZE: usize = 256;    
        unsafe {
            let buff = libc::malloc(RSMI_VBIOS_SIZE).cast();
            return Into::<RocmErr>::into(rsmi_dev_vbios_version_get(dv_ind, buff, RSMI_VBIOS_SIZE as u32)).map_with_buff(buff);
        }
    }
}

#[cfg(test)]
mod test{
    use crate::{RocmErr, RocmSmi};

    #[test]
    fn get_rsmi_version() -> Result<(), RocmErr> {
        println!("get_rsmi_version {}", RocmSmi::init()?.get_rsmi_version()?);
        Ok(())
    }


    #[test]
    fn get_rsmi_driver_version() -> Result<(), RocmErr> {
        println!("get_rsmi_driver_version {}", RocmSmi::init()?.get_rsmi_driver_version()?);
        Ok(())
    }

    #[test]
    fn get_rsmi_vbios_version() -> Result<(), RocmErr> {
        println!("get_rsmi_vbios_version {}", RocmSmi::init()?.get_device_rsmi_vbios_version(0)?);
        Ok(())
    }
}
