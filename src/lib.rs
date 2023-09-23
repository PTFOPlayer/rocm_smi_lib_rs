mod bindings;
use std::cell::RefCell;

use bindings::*;

pub mod error;
use error::*;
use queries::{
    identifiers::Identifiers,
    memory::Memory,
    pcie::Pcie,
    performance::{
        get_metrics, Frequency, FrequencyVoltageCurv, OverdriveLevels, PerformanceCounters,
        PerformanceLevel,
    },
    physical::Fans,
    power::Power,
};

mod tests;

pub mod queries;

pub mod device;
use device::*;

#[derive(Debug)]
struct RcStatus {
    count: u32,
    status: RocmErr,
}

#[derive(Debug)]
pub struct DeleteStatus {
    pub remaining_instances: u32,
    pub deletion_status: RocmErr,
}

thread_local!( static INIT_STATUS2: RefCell<RcStatus> = RefCell::new(RcStatus {
    count: 0,
    status: RocmErr::RsmiStatusInitError,
}));

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
        let status = INIT_STATUS2.with(|data| data.borrow().status);
        let count = INIT_STATUS2.with(|data| data.borrow().count);

        if status == RocmErr::RsmiStatusSuccess {
            INIT_STATUS2.with(|data| {
                RefCell::replace(
                    &data,
                    RcStatus {
                        count: count + 1,
                        status: RocmErr::RsmiStatusSuccess,
                    },
                )
            });

            let mut num_dev = 0u32;
            unsafe { rsmi_num_monitor_devices(&mut num_dev as *mut u32) }.try_err()?;
            return Ok(RocmSmi::new(num_dev));
        }

        let code = unsafe { rsmi_init(0) };

        if code != RocmErr::RsmiStatusSuccess {
            return Err(code);
        }

        INIT_STATUS2.with(|data| {
            RefCell::replace(
                &data,
                RcStatus {
                    count: 1,
                    status: code,
                },
            );
        });

        let mut num_dev = 0u32;
        unsafe { rsmi_num_monitor_devices(&mut num_dev as *mut u32) }.try_err()?;
        return Ok(RocmSmi::new(num_dev));
    }
    /// # Functionality
    /// This function converts general Rocm object into object for device with index = 0.
    pub fn into_first_device(self) -> Result<RocmSmiDevice, RocmErr> {
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
    pub fn into_device(self, id: u32) -> Result<RocmSmiDevice, RocmErr> {
        if id >= self.get_device_count() {
            return Err(RocmErr::RsmiStatusInputOutOfBounds);
        }
        Ok(RocmSmiDevice { id, rocm: self })
    }

    /// This function returns device count, last valid device identifier is equal to device cound - 1.
    pub fn get_device_count(&self) -> u32 {
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
    pub fn get_device_identifiers(&self, dv_ind: u32) -> Result<Identifiers, RocmErr> {
        unsafe { Identifiers::get_identifiers(dv_ind) }
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
        unsafe { get_metrics(dv_ind) }
    }
}

impl Drop for RocmSmi {
    fn drop(&mut self) {
        let count = INIT_STATUS2.with(|data| data.borrow().count);
        if count > 1 {
            INIT_STATUS2.with(|data| {
                RefCell::replace(
                    &data,
                    RcStatus {
                        count: count - 1,
                        status: RocmErr::RsmiStatusSuccess,
                    },
                );
            });
        } else {
            INIT_STATUS2.with(|data| {
                RefCell::replace(
                    &data,
                    RcStatus {
                        count: 0,
                        status: RocmErr::RsmiStatusInitError,
                    },
                );
            });
            unsafe {
                rsmi_shut_down();
            }
        }
    }
}
