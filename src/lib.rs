mod bindings;
use bindings::*;

pub mod error;
use error::*;
use queries::{memory::Memory, pcie::Pcie, physical::Fans, power::Power};

pub mod queries;

mod device;
use device::*;

#[derive(Debug)]
pub struct RocmSmi {
    pub device_count: u32,
}

impl RocmSmi {
    pub fn init() -> Result<Self, RocmErr> {
        let code = unsafe { init_c() };
        if code == 0 {
            return Ok(RocmSmi {
                device_count: unsafe { num_devices().check()? }.data,
            });
        }
        return Err(RocmErr::from_u16(code));
    }

    pub fn into_first_device(self) -> Result<RocmSmiDevice, RocmErr> {
        RocmSmiDevice::new_from_rocm(self, 0)
    }

    pub fn into_device(self, dv_ind: u32) -> Result<RocmSmiDevice, RocmErr> {
        RocmSmiDevice::new_from_rocm(self, dv_ind)
    }

    pub fn get_device_count(&self) -> u32 {
        self.device_count
    }

    pub fn get_device_id(&self, dv_ind: u32) -> Result<u16, RocmErr> {
        Ok(unsafe { device_id(dv_ind).check()? }.data)
    }

    pub fn get_device_name(&self, dv_ind: u32) -> Result<String, RocmErr> {
        unsafe { device_name(dv_ind).check()?.into_string() }
    }

    pub fn get_device_vendor_id(&self, dv_ind: u32) -> Result<u16, RocmErr> {
        Ok(unsafe { device_vendor_id(dv_ind).check()? }.data)
    }

    pub fn get_device_brand(&self, dv_ind: u32) -> Result<String, RocmErr> {
        unsafe { device_brand(dv_ind).check()? }.into_string()
    }

    pub fn get_device_vendor_name(&self, dv_ind: u32) -> Result<String, RocmErr> {
        unsafe { device_vendor_name(dv_ind).check()?.into_string() }
    }
    pub fn get_device_vram_vendor_name(&self, dv_ind: u32) -> Result<String, RocmErr> {
        unsafe { device_vram_vendor_name(dv_ind).check()?.into_string() }
    }

    pub fn get_device_serial_number(&self, dv_ind: u32) -> Result<String, RocmErr> {
        unsafe { device_serial(dv_ind).check()?.into_string() }
    }

    pub fn get_device_subsystem_id(&self, dv_ind: u32) -> Result<u16, RocmErr> {
        Ok(unsafe { device_subsystem_id(dv_ind).check()? }.data)
    }

    pub fn get_device_subsystem_name(&self, dv_ind: u32) -> Result<String, RocmErr> {
        unsafe { device_subsystem_name(dv_ind).check()?.into_string() }
    }

    pub fn get_device_drm_render_minor(&self, dv_ind: u32) -> Result<u32, RocmErr> {
        Ok(unsafe { device_drm_render(dv_ind).check()? }.data)
    }

    pub fn get_device_subsystem_vendor_id(&self, dv_ind: u32) -> Result<u16, RocmErr> {
        Ok(unsafe { device_subsystem_vendor_id(dv_ind) }.check()?.data)
    }

    pub fn get_device_unique_id(&self, dv_ind: u32) -> Result<u64, RocmErr> {
        Ok(unsafe { device_unique_id(dv_ind) }.check()?.data)
    }

    pub fn get_device_pcie_data<'a>(&self, dv_ind: u32) -> Result<Pcie<'a>, RocmErr> {
        Pcie::get_pcie(dv_ind)
    }

    pub fn get_device_power_data(&self, dv_ind: u32) -> Result<Power, RocmErr> {
        unsafe { Power::get_power(dv_ind) }
    }

    pub fn get_device_memory_data(&self, dv_ind: u32) -> Result<Memory<u64>, RocmErr> {
        unsafe { Memory::get_memory(dv_ind) }
    }

    pub fn get_device_fans_data(&self, dv_ind: u32) -> Result<Fans, RocmErr> {
        unsafe { Fans::get_fans(dv_ind) }
    }
}

#[cfg(test)]
mod test {
    use crate::RocmSmi;

    #[test]
    fn full_test() {
        match RocmSmi::init() {
            Ok(res) => {
                let count = res.get_device_count();
                println!("Count: {:?}", count);
                println!("ID: {:?}", res.get_device_id(0));
                println!("name: {:?}", res.get_device_name(0));
                println!("vendor id: {:?}", res.get_device_vendor_id(0));
                println!("brand: {:?}", res.get_device_brand(0));
                println!("vendor name: {:?}", res.get_device_vendor_name(0));
                println!("vram vendor name: {:?}", res.get_device_vram_vendor_name(0));
                println!("serial: {:?}", res.get_device_serial_number(0));
                println!("subsystem id: {:?}", res.get_device_subsystem_id(0));
                println!("subsystem name: {:?}", res.get_device_subsystem_name(0));
                println!("drm render minor: {:?}", res.get_device_drm_render_minor(0));
                println!(
                    "subsystem vendor id {:?}",
                    res.get_device_subsystem_vendor_id(0)
                );
                println!(
                    "unique id (might fail if there is only one gpu) {:?}",
                    res.get_device_unique_id(0)
                );
                println!("pcie data: {:?}", res.get_device_pcie_data(0));
                println!("power data: {:?}", res.get_device_power_data(0));
                println!(
                    "memory data: {:?}",
                    res.get_device_memory_data(0).unwrap().into_f64_gb()
                );
                println!("fans data: {:?}", res.get_device_fans_data(0).unwrap());
            }
            Err(err) => println!("{:?}", err),
        }
    }

    #[test]
    fn device_test() {
        match RocmSmi::init() {
            Ok(res) => match res.into_first_device() {
                Ok(dev) => println!("succesfully got first device: {:?}", dev.get_brand()),
                Err(err) => println!("{:?}", err),
            },
            Err(err) => println!("{:?}", err),
        }
    }
}
