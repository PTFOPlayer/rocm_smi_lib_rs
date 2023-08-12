mod bindings;
use bindings::*;

mod error;
use error::*;

pub struct RocmSmi {}

impl RocmSmi {
    pub fn init() -> Result<Self, RocmErr> {
        let code = unsafe { init_c() };
        if code == 0 {
            return Ok(RocmSmi {});
        }
        Err(RocmErr::from_u16(code))
    }

    pub fn get_device_count(&self) -> Result<u32, RocmErr> {
        let res = unsafe { num_devices() };

        if res.status != 0 {
            return Err(RocmErr::from_u16(res.status));
        }

        Ok(res.data)
    }

    pub fn get_device_id(&self, dev_id: u32) -> Result<u16, RocmErr> {
        let res = unsafe { device_id(dev_id) };

        if res.status != 0 {
            return Err(RocmErr::from_u16(res.status));
        }

        Ok(res.data)
    }

    pub fn get_device_name(&self, dev_id: u32) -> Result<String, RocmErr> {
        let res = unsafe { device_name(dev_id) };

        if res.status != 0 {
            return Err(RocmErr::from_u16(res.status));
        }
        let c_str = unsafe { std::ffi::CStr::from_ptr(res.data) };
        let data = c_str.to_str().to_owned();
        match data {
            Ok(res) => Ok(res.to_owned()),
            Err(_) => Err(RocmErr::RsmiStatusUnknownError),
        }
    }

    pub fn get_vendor_id(&self, dev_id: u32) -> Result<u16, RocmErr> {
        let res = unsafe { device_vendor_id(dev_id) };

        if res.status != 0 {
            return Err(RocmErr::from_u16(res.status));
        }

        return Ok(res.data)
    }
}

#[cfg(test)]
mod test {
    use crate::RocmSmi;

    #[test]
    fn main_test() {
        match RocmSmi::init() {
            Ok(res) => {
                println!("{:?}", res.get_device_count());
                println!("{:?}", res.get_device_id(0));
                println!("{:?}", res.get_device_name(0));
                println!("{:?}", res.get_vendor_id(0));
            }
            Err(err) => println!("{:?}", err),
        }
    }
}
