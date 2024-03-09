#[cfg(feature = "fn_query")]
use functions::supported_fn::get_supported_fn;
use rocm_smi_lib_sys::{bindings::*, error::RocmErr, RawRsmi};

use queries::{
    error::EccData,
    identifiers::Identifiers,
    memory::Memory,
    pcie::Pcie,
    performance::{
        get_metrics, Frequency, FrequencyVoltageCurv, OverdriveLevels, PerformanceCounters,
    },
    physical::Fans,
    power::Power,
};
mod tests;

pub use rocm_smi_lib_sys::error;
pub mod device;
pub mod functions;
pub mod queries;
use device::*;

#[derive(Debug)]
pub struct DeleteStatus {
    pub remaining_instances: u32,
    pub deletion_status: RocmErr,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct RocmSmi {
    device_count: u32,
    raw: RawRsmi,
}

impl RocmSmi {
    pub(crate) fn new(count: u32, raw: RawRsmi) -> Self {
        RocmSmi {
            device_count: count,
            raw,
        }
    }
    /// # Functionality
    ///
    /// This function is used to initiate Rocm, there can be only instance of RocmSmi at time but if you try to initiate it second time it will just use previous instance
    ///
    /// # Errors
    ///
    /// This function will return an error if it's impossible to initiate Rocm.
    pub fn init() -> Result<Self, RocmErr> {
        let mut raw = unsafe { RawRsmi::new(0) }?;

        let mut num_dev = 0u32;
        unsafe { raw.rsmi_num_monitor_devices(&mut num_dev as *mut u32) }.try_err()?;
        return Ok(RocmSmi::new(num_dev, raw));
    }
    /// # Functionality
    /// This function converts general Rocm object into object for device with index = 0.
    #[cfg(feature = "device")]
    pub fn into_first_device(mut self) -> Result<RocmSmiDevice, RocmErr> {
        if 0 >= self.get_device_count() {
            return Err(RocmErr::RsmiStatusInputOutOfBounds);
        }
        Ok(RocmSmiDevice { id: 0, rocm: self })
    }
    /// # Functionality
    ///
    /// This function converts general Rocm object into object for device with provided `id`.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided `id` is not valid device identifier.
    #[cfg(feature = "device")]
    pub fn into_device(mut self, id: u32) -> Result<RocmSmiDevice, RocmErr> {
        if id >= self.get_device_count() {
            return Err(RocmErr::RsmiStatusInputOutOfBounds);
        }
        Ok(RocmSmiDevice { id, rocm: self })
    }

    /// This function returns device count, last valid device identifier is equal to device cound - 1.
    pub fn get_device_count(&mut self) -> u32 {
        self.device_count
    }
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
        unsafe { Identifiers::get_identifiers(&mut self.raw, dv_ind) }
    }

    /// # Functionality
    ///
    /// This function returns pcie information for given device.
    /// example:
    /// ```rust,no_compile,ignore
    /// use rocm_smi_lib::RocmSmi;
    /// use rocm_smi_lib::error::RocmErr;
    /// fn print_gpu_pcie_lines() -> Result<(), RocmErr> {
    ///     let rocm = RocmSmi::init()?;
    ///     let lines = rocm.get_device_pcie_data(0)?.get_bandwidth_and_throughput().lines;
    ///     println!("{}", lines);
    ///     Ok(())
    /// }
    /// ```
    /// for example for RX 7600 will print you:
    /// ```no_compile,ignore
    /// 8
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if `dv_ind` id not valid device identifier.
    pub fn get_device_pcie_data<'a>(&mut self, dv_ind: u32) -> Result<Pcie, RocmErr> {
        Pcie::get_pcie(&mut self.raw, dv_ind)
    }

    /// # Functionality
    ///
    /// This function returns power information for given device.
    /// example:
    /// ```rust,no_compile,ignore
    /// use rocm_smi_lib::RocmSmi;
    /// use rocm_smi_lib::error::RocmErr;
    /// fn print_gpu_pcie_lines() -> Result<(), RocmErr> {
    ///     let rocm = RocmSmi::init()?;
    ///     let sensors = rocm.get_device_power_data(0)?.sensor_count;
    ///     println!("{}", sensors);
    ///     Ok(())
    /// }
    /// ```
    /// for example for RX 7600 will print you:
    /// ```no_compile,ignore
    /// 1
    /// ```
    ///
    /// # Errors
    ///
    /// This function will return an error if `dv_ind` id not valid device identifier.
    pub fn get_device_power_data(&mut self, dv_ind: u32) -> Result<Power, RocmErr> {
        unsafe { Power::get_power(&mut self.raw, dv_ind) }
    }

    pub fn get_device_memory_data(&mut self, dv_ind: u32) -> Result<Memory, RocmErr> {
        unsafe { Memory::get_memory(&mut self.raw, dv_ind) }
    }

    pub fn get_device_fans_data(&mut self, dv_ind: u32) -> Result<Fans, RocmErr> {
        unsafe { Fans::get_fans(&mut self.raw, dv_ind) }
    }

    pub fn get_device_temperature_metric(
        &mut self,
        dv_ind: u32,
        sensor: RsmiTemperatureSensor,
        metric: RsmiTemperatureMetric,
    ) -> Result<f64, RocmErr> {
        let mut temp = 0i64;
        unsafe {
            self.raw
                .rsmi_dev_temp_metric_get(dv_ind, sensor, metric, &mut temp as *mut i64)
                .try_err()
        }?;
        Ok(temp as f64 / 1000.)
    }

    pub fn get_device_voltage_metric(
        &mut self,
        dv_ind: u32,
        metric: RsmiVoltageMetric,
    ) -> Result<f64, RocmErr> {
        let mut volt = 0i64;
        unsafe {
            self.raw
                .rsmi_dev_volt_metric_get(
                    dv_ind,
                    RsmiVoltageType::RsmiVoltTypeVddgfx,
                    metric,
                    &mut volt as *mut i64,
                )
                .try_err()
        }?;
        Ok(volt as f64 / 1000.)
    }

    pub fn get_device_busy_percent(&mut self, dv_ind: u32) -> Result<u32, RocmErr> {
        let mut percent = 0u32;
        unsafe {
            self.raw
                .rsmi_dev_busy_percent_get(dv_ind, &mut percent as *mut u32)
                .try_err()
        }?;
        Ok(percent)
    }

    pub fn get_device_performance_countes(
        &mut self,
        dv_ind: u32,
    ) -> Result<PerformanceCounters, RocmErr> {
        unsafe { PerformanceCounters::get_counters(&mut self.raw, dv_ind) }
    }

    pub fn get_device_performance_level(
        &mut self,
        dv_ind: u32,
    ) -> Result<PerformanceLevel, RocmErr> {
        unsafe {
            let mut level = PerformanceLevel::Unknown;
            self.raw
                .rsmi_dev_perf_level_get(dv_ind, &mut level as *mut PerformanceLevel)
                .try_err()?;
            Ok(level)
        }
    }

    pub fn get_device_overdrive_levels(&mut self, dv_ind: u32) -> Result<OverdriveLevels, RocmErr> {
        unsafe { OverdriveLevels::get_overdrive_levels(&mut self.raw, dv_ind) }
    }

    pub fn get_device_frequency(
        &mut self,
        dv_ind: u32,
        freq_type: RsmiClkType,
    ) -> Result<Frequency, RocmErr> {
        unsafe { Frequency::get_freq(&mut self.raw, dv_ind, freq_type) }
    }

    pub fn get_device_frequency_voltage_curve(
        &mut self,
        dv_ind: u32,
    ) -> Result<FrequencyVoltageCurv, RocmErr> {
        unsafe { FrequencyVoltageCurv::get_curve(&mut self.raw, dv_ind) }
    }

    pub fn get_device_full_metrics(&mut self, dv_ind: u32) -> Result<GpuMetrics, RocmErr> {
        unsafe { get_metrics(&mut self.raw, dv_ind) }
    }

    pub fn get_device_ecc_data(&mut self, dv_ind: u32) -> EccData {
        unsafe { EccData::new(&mut self.raw, dv_ind) }
    }

    pub fn get_device_vbios_version(&mut self, dv_ind: u32) -> Result<String, RocmErr> {
        unsafe {
            let buff = libc::malloc(128).cast();
            self.raw
                .rsmi_dev_vbios_version_get(dv_ind, buff, 128)
                .try_err()?;
            let temp = std::ffi::CString::from_raw(buff);
            return Ok(temp.to_string_lossy().to_string());
        }
    }

    #[cfg(feature = "fn_query")]
    pub fn get_supported_functions(&mut self) -> Result<Vec<String>, RocmErr> {
        unsafe { get_supported_fn(&mut self.raw) }
    }

    #[cfg(feature = "process")]
    pub fn get_compute_process_info<'a>(&mut self) -> Result<&'a [RsmiProcessInfoT], RocmErr> {
        let mut num_items = 0u32;
        let procs = vec![].as_mut_ptr();
        unsafe {
            self.raw
                .rsmi_compute_process_info_get(procs, &mut num_items as *mut u32)
                .try_err()?;
        }
        Ok(unsafe { std::slice::from_raw_parts_mut(procs, num_items as usize) })
    }

    #[cfg(feature = "process")]
    pub fn get_compute_process_info_by_pid(
        &mut self,
        pid: u32,
    ) -> Result<RsmiProcessInfoT, RocmErr> {
        let mut procs = RsmiProcessInfoT::default();
        unsafe {
            self.raw
                .rsmi_compute_process_info_by_pid_get(pid, &mut procs as *mut RsmiProcessInfoT)
                .try_err()?
        };
        Ok(procs)
    }
}

// pub fn rsmi_compute_process_gpus_get(pid: u32) -> Result<&[u32], RocmErr> {
//     let mut indices = vec![].as_mut_ptr();
//     let mut num_devices = 0u32;
// }
