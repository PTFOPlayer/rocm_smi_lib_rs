use crate::{
    bindings::{RsmiClkType, RsmiTemperatureMetric, RsmiTemperatureSensor, RsmiVoltageMetric},
    error::RocmErr,
    queries::{
        identifiers::Identifiers,
        memory::Memory,
        pcie::Pcie,
        performance::{
            Frequency, FrequencyVoltageCurv, OverdriveLevels, PerformanceCounters, PerformanceLevel,
        },
        physical::Fans,
        power::Power,
    },
    RocmSmi,
};

pub struct RocmSmiDevice {
    id: u32,
    rocm: RocmSmi,
}

impl RocmSmiDevice {
    #[inline(always)]
    pub(crate) fn new_from_rocm(rocm: RocmSmi, id: u32) -> Result<Self, RocmErr> {
        if id >= rocm.get_device_count() {
            return Err(RocmErr::RsmiStatusInputOutOfBounds);
        }
        Ok(Self { id, rocm })
    }

    pub fn new(id: u32) -> Result<Self, RocmErr> {
        let rocm = RocmSmi::init()?;
        if id >= rocm.get_device_count() {
            return Err(RocmErr::RsmiStatusInputOutOfBounds);
        }
        Ok(Self { id, rocm })
    }

    pub fn get_identifiers(&self) -> Result<Identifiers, RocmErr> {
        self.rocm.get_device_identifiers(self.id)
    }

    pub fn get_pcie_data<'a>(&self) -> Result<Pcie<'a>, RocmErr> {
        self.rocm.get_device_pcie_data(self.id)
    }

    pub fn get_power_data(&self) -> Result<Power, RocmErr> {
        self.rocm.get_device_power_data(self.id)
    }

    pub fn get_memory_data(&self) -> Result<Memory<u64>, RocmErr> {
        self.rocm.get_device_memory_data(self.id)
    }

    pub fn get_fans_data(&self) -> Result<Fans, RocmErr> {
        self.rocm.get_device_fans_data(self.id)
    }

    pub fn get_temperature_metric(
        &self,
        sensor: RsmiTemperatureSensor,
        metric: RsmiTemperatureMetric,
    ) -> Result<f64, RocmErr> {
        self.rocm
            .get_device_temperature_metric(self.id, sensor, metric)
    }

    pub fn get_voltage_metric(&self, metric: RsmiVoltageMetric) -> Result<f64, RocmErr> {
        self.rocm.get_device_voltage_metric(self.id, metric)
    }

    pub fn get_busy_percent(&self) -> Result<u32, RocmErr> {
        self.rocm.get_device_busy_percent(self.id)
    }

    pub fn get_performance_countes(&self) -> Result<PerformanceCounters, RocmErr> {
        self.rocm.get_device_performance_countes(self.id)
    }

    pub fn get_performance_level(&self) -> Result<PerformanceLevel, RocmErr> {
        self.rocm.get_device_performance_level(self.id)
    }

    pub fn get_overdrive_levels(&self) -> Result<OverdriveLevels, RocmErr> {
        self.rocm.get_device_overdrive_levels(self.id)
    }

    pub fn get_frequency<'a>(&self, freq_type: RsmiClkType) -> Result<Frequency<'a>, RocmErr> {
        self.rocm.get_device_frequency(self.id, freq_type)
    }

    pub fn get_frequency_voltage_curve<'a>(&self) -> Result<FrequencyVoltageCurv<'a>, RocmErr> {
        self.rocm.get_device_frequency_voltage_curve(self.id)
    }
}
