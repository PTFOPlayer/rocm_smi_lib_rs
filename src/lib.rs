pub use error::{IntoRocmErr, RocmErr};
// #[cfg(feature = "fn_query")]
// pub use functions::supported_fn::get_supported_fn;
pub use queries::common_structures::*;
use queries::performance::get_metrics;
pub use queries::performance::RsmiClkType;
pub use rocm_smi_lib_sys::bindings::*;

pub use queries::{
    error::EccData,
    memory::Memory,
    performance::{Frequency, FrequencyVoltageCurv, OverdriveLevels, PerformanceCounters},
    physical::Fans,
};
pub mod device;
pub mod error;
pub mod functions;
pub mod queries;
mod tests;
pub use device::*;

#[derive(Debug)]
pub struct DeleteStatus {
    pub remaining_instances: u32,
    pub deletion_status: RocmErr,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct RocmSmi {
    device_count: u32,
}

impl RocmSmi {
    pub(crate) fn new(count: u32) -> Self {
        RocmSmi {
            device_count: count,
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
        unsafe { rsmi_init(0) };
        let mut num_dev = 0u32;

        unsafe {
            Into::<RocmErr>::into(rsmi_num_monitor_devices(&mut num_dev as *mut u32)).try_err()?
        };
        return Ok(RocmSmi::new(num_dev));
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

    /// #  Functionality
    ///
    /// This function returns info about memory ammount and its utilization for given device.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided `id` is not valid device identifier.
    pub fn get_device_memory_data(&mut self, dv_ind: u32) -> Result<Memory, RocmErr> {
        unsafe { Memory::get_memory(dv_ind) }
    }

    /// #  Functionality
    ///
    /// This function returns info about fans speed.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided `id` is not valid device identifier.
    pub fn get_device_fans_data(&mut self, dv_ind: u32) -> Result<Fans, RocmErr> {
        unsafe { Fans::get_fans(dv_ind) }
    }

    /// #  Functionality
    ///
    /// This function returns info about temperature at selected sensor.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided `id` is not valid device identifier, or if sensor or temperature metric is not available.
    pub fn get_device_temperature_metric(
        &mut self,
        dv_ind: u32,
        sensor: RsmiTemperatureType,
        metric: RsmiTemperatureMetric,
    ) -> Result<f64, RocmErr> {
        let mut temp = 0i64;
        unsafe {
            rsmi_dev_temp_metric_get(dv_ind, sensor as u32, metric as u32, &mut temp as *mut i64)
                .into_rocm_err()?
        };
        Ok(temp as f64 / 1000.)
    }

    /// #  Functionality
    ///
    /// This function returns info about voltage.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided `id` is not valid device identifier, or if data for given metric is not available.
    pub fn get_device_voltage_metric(
        &mut self,
        dv_ind: u32,
        metric: RsmiVoltageMetric,
    ) -> Result<f64, RocmErr> {
        let mut volt = 0i64;
        unsafe {
           
                rsmi_dev_volt_metric_get(
                    dv_ind,
                    0,
                    metric as u32,
                    &mut volt as *mut i64,
                )
                .into_rocm_err()?
        };
        Ok(volt as f64 / 1000.)
    }

    /// #  Functionality
    ///
    /// Returns ammount of time GPU is busy doing any processing.
    ///
    /// # Errors
    ///
    /// This function will return an error if provided `id` is not valid device identifier,.
    pub fn get_device_busy_percent(&mut self, dv_ind: u32) -> Result<u32, RocmErr> {
        let mut percent = 0u32;
        unsafe {
           
                rsmi_dev_busy_percent_get(dv_ind, &mut percent as *mut u32)
                .into_rocm_err()
        }?;
        Ok(percent)
    }

    pub fn get_device_performance_countes(
        &mut self,
        dv_ind: u32,
    ) -> Result<PerformanceCounters, RocmErr> {
        unsafe { PerformanceCounters::get_counters(dv_ind) }
    }

    pub fn get_device_performance_level(
        &mut self,
        dv_ind: u32,
    ) -> Result<RsmiDevPerfLevel, RocmErr> {
        unsafe {
            let mut level = RsmiDevPerfLevel::Unknown;
           
                rsmi_dev_perf_level_get(dv_ind, (&mut level as *mut RsmiDevPerfLevel).cast::<u32>() )
                .into_rocm_err()?;
            Ok(level)
        }
    }

    pub fn get_device_overdrive_levels(&mut self, dv_ind: u32) -> Result<OverdriveLevels, RocmErr> {
        unsafe { OverdriveLevels::get_overdrive_levels(dv_ind) }
    }

    pub fn get_device_frequency(
        &mut self,
        dv_ind: u32,
        freq_type: RsmiClkType,
    ) -> Result<Frequency, RocmErr> {
        unsafe { Frequency::get_freq(dv_ind, freq_type) }
    }

    pub fn get_device_frequency_voltage_curve(
        &mut self,
        dv_ind: u32,
    ) -> Result<FrequencyVoltageCurv, RocmErr> {
        unsafe { FrequencyVoltageCurv::get_curve(dv_ind) }
    }

    pub fn get_device_full_metrics(&mut self, dv_ind: u32) -> Result<rsmi_gpu_metrics_t, RocmErr> {
        unsafe { get_metrics(dv_ind) }
    }

    pub fn get_device_ecc_data(&mut self, dv_ind: u32) -> EccData {
        unsafe { EccData::new(dv_ind) }
    }

    pub fn get_device_vbios_version(&mut self, dv_ind: u32) -> Result<String, RocmErr> {
        unsafe {
            let buff = libc::malloc(128).cast();
           
                rsmi_dev_vbios_version_get(dv_ind, buff, 128)
                .into_rocm_err()?;
            let temp = std::ffi::CString::from_raw(buff);
            return Ok(temp.to_string_lossy().to_string());
        }
    }

    // #[cfg(feature = "fn_query")]
    // pub fn get_supported_functions(&mut self) -> Result<Vec<String>, RocmErr> {
    //     unsafe { get_supported_fn() }
    // }

    // #[cfg(feature = "process")]
    // pub fn get_compute_process_info<'a>(&mut self) -> Result<&'a [RsmiProcessInfoT], RocmErr> {
    //     let mut num_items = 0u32;
    //     let procs = vec![].as_mut_ptr();
    //     unsafe {
           
    //             rsmi_compute_process_info_get(procs, &mut num_items as *mut u32)
    //             .into_rocm_err()?;
    //     }
    //     Ok(unsafe { std::slice::from_raw_parts_mut(procs, num_items as usize) })
    // }

    // #[cfg(feature = "process")]
    // pub fn get_compute_process_info_by_pid(
    //     &mut self,
    //     pid: u32,
    // ) -> Result<RsmiProcessInfoT, RocmErr> {
    //     let mut procs = RsmiProcessInfoT::default();
    //     unsafe {
           
    //             rsmi_compute_process_info_by_pid_get(pid, &mut procs as *mut RsmiProcessInfoT)
    //             .into_rocm_err()?
    //     };
    //     Ok(procs)
    // }
}

// pub fn rsmi_compute_process_gpus_get(pid: u32) -> Result<&[u32], RocmErr> {
//     let mut indices = vec![].as_mut_ptr();
//     let mut num_devices = 0u32;
// }

pub(crate) trait MapWithString {
    unsafe fn map_with_buff(&self, buff: *mut i8) -> Result<String, RocmErr>;
}

impl MapWithString for RocmErr {
    unsafe fn map_with_buff(&self, buff: *mut i8) -> Result<String, RocmErr> {
        self.try_err().map(|_| {
            std::ffi::CString::from_raw(buff)
                .to_string_lossy()
                .to_string()
        })
    }
}
