use rocm_smi_lib_sys::{error::RocmErr, bindings::*};

use crate::{
    queries::{
        identifiers::Identifiers,
        memory::Memory,
        pcie::Pcie,
        performance::{
            Frequency, FrequencyVoltageCurv, OverdriveLevels, PerformanceCounters,
        },
        physical::Fans,
        power::Power, error::EccData,
    },
    RocmSmi,
};

pub struct RocmSmiDevice {
    pub(crate) id: u32,
    pub(crate) rocm: RocmSmi,
}

impl RocmSmiDevice {
    pub fn new(id: u32) -> Result<Self, RocmErr> {
        let mut rocm = RocmSmi::init()?;
        if id >= rocm.get_device_count() {
            return Err(RocmErr::RsmiStatusInputOutOfBounds);
        }
        Ok(Self { id, rocm })
    }

    pub fn get_identifiers(&mut self) -> Result<Identifiers, RocmErr> {
        self.rocm.get_device_identifiers(self.id)
    }

    pub fn get_pcie_data<'a>(&mut self) -> Result<Pcie, RocmErr> {
        self.rocm.get_device_pcie_data(self.id)
    }

    pub fn get_power_data(&mut self) -> Result<Power, RocmErr> {
        self.rocm.get_device_power_data(self.id)
    }

    pub fn get_memory_data(&mut self) -> Result<Memory, RocmErr> {
        self.rocm.get_device_memory_data(self.id)
    }

    pub fn get_fans_data(&mut self) -> Result<Fans, RocmErr> {
        self.rocm.get_device_fans_data(self.id)
    }

    pub fn get_temperature_metric(
        &mut self,
        sensor: RsmiTemperatureSensor,
        metric: RsmiTemperatureMetric,
    ) -> Result<f64, RocmErr> {
        self.rocm
            .get_device_temperature_metric(self.id, sensor, metric)
    }

    pub fn get_voltage_metric(&mut self, metric: RsmiVoltageMetric) -> Result<f64, RocmErr> {
        self.rocm.get_device_voltage_metric(self.id, metric)
    }

    pub fn get_busy_percent(&mut self) -> Result<u32, RocmErr> {
        self.rocm.get_device_busy_percent(self.id)
    }

    pub fn get_performance_countes(&mut self) -> Result<PerformanceCounters, RocmErr> {
        self.rocm.get_device_performance_countes(self.id)
    }

    pub fn get_performance_level(&mut self) -> Result<PerformanceLevel, RocmErr> {
        self.rocm.get_device_performance_level(self.id)
    }

    pub fn get_overdrive_levels(&mut self) -> Result<OverdriveLevels, RocmErr> {
        self.rocm.get_device_overdrive_levels(self.id)
    }

    pub fn get_frequency(&mut self, freq_type: RsmiClkType) -> Result<Frequency, RocmErr> {
        self.rocm.get_device_frequency(self.id, freq_type)
    }

    pub fn get_frequency_voltage_curve<'a>(&mut self) -> Result<FrequencyVoltageCurv, RocmErr> {
        self.rocm.get_device_frequency_voltage_curve(self.id)
    }

    pub fn get_full_metrics(&mut self) -> Result<RsmiGpuMetrics, RocmErr> {
        self.rocm.get_device_full_metrics(self.id)
    }

    pub fn get_ecc_data(&mut self) -> EccData {
        self.rocm.get_device_ecc_data(self.id)
    }

    pub fn get_vbios_version(&mut self) -> Result<String, RocmErr> {
        self.rocm.get_device_vbios_version(self.id)
    }
    
    #[cfg(feature = "fn_query")]
    pub fn get_supported_functions(&mut self) -> Result<Vec<String>, RocmErr> {
        self.rocm.get_supported_functions()
    }

    pub fn get_rsmi_version(&mut self) -> Result<String, RocmErr> {
        self.rocm.get_rsmi_version()
    }
}
