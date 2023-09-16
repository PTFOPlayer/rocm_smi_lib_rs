mod bindings;
use bindings::*;

pub mod error;
use error::*;
use queries::{
    identifiers::Identifiers,
    memory::Memory,
    pcie::Pcie,
    performance::{
        Frequency, FrequencyVoltageCurv, OverdriveLevels, PerformanceCounters, PerformanceLevel,
    },
    physical::Fans,
    power::Power,
};

mod tests;

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
        if 0 >= self.get_device_count() {
            return Err(RocmErr::RsmiStatusInputOutOfBounds);
        }
        Ok(RocmSmiDevice { id: 0, rocm: self })
    }

    pub fn into_device(self, id: u32) -> Result<RocmSmiDevice, RocmErr> {
        if id >= self.get_device_count() {
            return Err(RocmErr::RsmiStatusInputOutOfBounds);
        }
        Ok(RocmSmiDevice { id, rocm: self })
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

    pub fn get_device_memory_data(&self, dv_ind: u32) -> Result<Memory, RocmErr> {
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

    pub fn get_device_performance_countes(
        &self,
        dv_ind: u32,
    ) -> Result<PerformanceCounters, RocmErr> {
        unsafe { PerformanceCounters::get_counters(dv_ind) }
    }

    pub fn get_device_performance_level(&self, dv_ind: u32) -> Result<PerformanceLevel, RocmErr> {
        unsafe { PerformanceLevel::get_performance_level(dv_ind) }
    }

    pub fn get_device_overdrive_levels(&self, dv_ind: u32) -> Result<OverdriveLevels, RocmErr> {
        unsafe { OverdriveLevels::get_overdrive_levels(dv_ind) }
    }

    pub fn get_device_frequency<'a>(
        &self,
        dv_ind: u32,
        freq_type: RsmiClkType,
    ) -> Result<Frequency<'a>, RocmErr> {
        unsafe { Frequency::get_freq(dv_ind, freq_type) }
    }

    pub fn get_device_frequency_voltage_curve<'a>(
        &self,
        dv_ind: u32,
    ) -> Result<FrequencyVoltageCurv<'a>, RocmErr> {
        unsafe { FrequencyVoltageCurv::get_curve(dv_ind) }
    }

    pub fn get_device_full_metrics(&self, dv_ind: u32) -> Result<GpuMetrics, RocmErr> {
        unsafe{ Ok(metrics(dv_ind).check()?.metrics)}
    }
}
