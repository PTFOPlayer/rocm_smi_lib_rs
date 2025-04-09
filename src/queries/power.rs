use std::{mem::size_of, slice::from_raw_parts};

use libc::{malloc, realloc};

use rocm_smi_lib_sys::bindings::*;

use crate::{
    error::{IntoRocmErr, RocmErr},
    RocmSmi,
};

pub type RsmiPowerProfileStatus = rsmi_power_profile_status_t;

#[derive(Debug, Clone)]
pub struct Power<'a> {
    pub sensor_count: u32,
    pub default_power_cap: u64,
    pub current_power: u64,
    pub power_per_sensor: &'a [u64],
    pub power_cap_per_sensor: &'a [u64],
    pub power_cap_min_sensor: &'a [u64],
    pub power_cap_max_sensor: &'a [u64],
    pub power_profile_preset: RsmiPowerProfileStatus,
}

impl RocmSmi {
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
        let mut sensor_count = 0;
        let mut ave = unsafe { malloc(size_of::<u64>()).cast() };
        let mut cap = unsafe { malloc(size_of::<u64>()).cast() };
        let mut max = unsafe { malloc(size_of::<u64>()).cast() };
        let mut min = unsafe { malloc(size_of::<u64>()).cast() };
        unsafe {
            rsmi_dev_power_ave_get(dv_ind, sensor_count, ave).into_rocm_err()?;
            rsmi_dev_power_cap_get(dv_ind, sensor_count, cap).into_rocm_err()?;
            rsmi_dev_power_cap_range_get(dv_ind, sensor_count, max, min).into_rocm_err()?;
        }
        sensor_count += 1;
        let mut counter = sensor_count as usize + 1;
        loop {
            unsafe {
                ave = realloc(ave.cast(), counter * size_of::<u64>()).cast();
                cap = realloc(cap.cast(), counter * size_of::<u64>()).cast();
                max = realloc(max.cast(), counter * size_of::<u64>()).cast();
                min = realloc(min.cast(), counter * size_of::<u64>()).cast();

                let ret_ave = rsmi_dev_power_ave_get(
                    dv_ind,
                    sensor_count,
                    ave.add(sensor_count as usize * size_of::<u64>()),
                );

                let ret_cap = rsmi_dev_power_cap_get(
                    dv_ind,
                    sensor_count,
                    cap.add(sensor_count as usize * size_of::<u64>()),
                );

                let ret_rng = rsmi_dev_power_cap_range_get(
                    dv_ind,
                    sensor_count,
                    max.add(sensor_count as usize * size_of::<u64>()),
                    min.add(sensor_count as usize * size_of::<u64>()),
                );

                if ret_ave != RocmErr::RsmiStatusSuccess.into()
                    || ret_cap != RocmErr::RsmiStatusSuccess.into()
                    || ret_rng != RocmErr::RsmiStatusSuccess.into()
                {
                    break;
                }
                sensor_count += 1;
                counter += 1;
            }
        }

        let mut current_power = 0u64;
        let mut default_power_cap = 0u64;
        let power_profile_preset: *mut RsmiPowerProfileStatus = std::ptr::null_mut();
        unsafe {
            rsmi_dev_current_socket_power_get(dv_ind, &mut current_power as *mut u64)
                .into_rocm_err()?;
            rsmi_dev_power_cap_default_get(dv_ind, &mut default_power_cap as *mut u64)
                .into_rocm_err()?;
            rsmi_dev_power_profile_presets_get(dv_ind, 0, power_profile_preset).into_rocm_err()?
        };

        Ok(Power {
            sensor_count,
            default_power_cap,
            current_power,
            power_per_sensor: unsafe { from_raw_parts(ave, sensor_count as usize) },
            power_cap_per_sensor: unsafe { from_raw_parts(cap, sensor_count as usize) },
            power_cap_min_sensor: unsafe { from_raw_parts(min, sensor_count as usize) },
            power_cap_max_sensor: unsafe { from_raw_parts(max, sensor_count as usize) },
            power_profile_preset: unsafe { (*power_profile_preset).clone() },
        })
    }
}
