mod bindings;
use bindings::*;

pub mod error;
use error::*;
use queries::{identifiers::Identifiers, memory::Memory, pcie::Pcie, physical::Fans, power::Power};

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

    pub fn get_device_identifiers(&self, dv_ind: u32) -> Result<Identifiers, RocmErr> {
        unsafe { Identifiers::get_identifiers(dv_ind) }
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

    pub fn get_device_temperature_metric(
        &self,
        dv_ind: u32,
        sensor: RsmiTemperatureSensor,
        metric: RsmiTemperatureMetric,
    ) -> Result<f64, RocmErr> {
        Ok(unsafe { temperature(dv_ind, sensor, metric).check()?.data as f64 / 1000. })
    }

    pub fn get_device_voltage_metric(
        &self,
        dv_ind: u32,
        metric: RsmiVoltageMetric,
    ) -> Result<f64, RocmErr> {
        Ok(unsafe { voltage(dv_ind, metric).check()?.data as f64 / 1000. })
    }

    pub fn get_device_busy_percent(&self, dv_ind: u32) -> Result<u32, RocmErr> {
        Ok(unsafe { busy_percent(dv_ind).check()?.data })
    }
}

#[cfg(test)]
mod test {
    use crate::{
        bindings::{RsmiTemperatureMetric, RsmiTemperatureSensor, RsmiVoltageMetric},
        error::RocmErr,
        queries::performance::PerformanceCounters,
        RocmSmi,
    };

    #[test]
    fn full_test() -> Result<(), RocmErr> {
        let res = RocmSmi::init()?.into_first_device()?;
        let identifiers = res.get_identifiers()?;
        println!("identifiers: {:?}", identifiers);
        println!(
            "unique id (might fail if there is only one gpu) {:?}",
            identifiers.get_unique_id()
        );
        println!("pcie data: {:?}", res.get_pcie_data());
        println!("power data: {:?}", res.get_power_data());
        println!("memory data: {:?}", res.get_memory_data());
        println!("fans data: {:?}", res.get_fans_data());
        println!(
            "junction temperature data: {:?}",
            res.get_temperature_metric(
                RsmiTemperatureSensor::RsmiTempTypeJunction,
                RsmiTemperatureMetric::RsmiTempCurrent
            )
        );
        println!(
            "memory temperature data: {:?}",
            res.get_temperature_metric(
                RsmiTemperatureSensor::RsmiTempTypeMemory,
                RsmiTemperatureMetric::RsmiTempCurrent
            )
        );
        println!(
            "voltage data: {:?}",
            res.get_voltage_metric(RsmiVoltageMetric::RsmiVoltCurrent)
        );
        println!("busy percent: {:?}", res.get_busy_percent());

        unsafe {
            println!("util c: {:?}", PerformanceCounters::get_counters(0));
        }

        Ok(())
    }
}
