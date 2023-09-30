use std::{mem::size_of, slice::from_raw_parts};

use libc::{malloc, realloc};

use rocm_smi_lib_sys::{bindings::*, error::RocmErr};

#[derive(Debug, Clone)]
pub struct Power<'a> {
    pub sensor_count: u32,
    pub default_power_cap: u64,
    pub power_per_sensor: &'a [u64],
    pub power_cap_per_sensor: &'a [u64],
    pub power_cap_min_sensor: &'a [u64],
    pub power_cap_max_sensor: &'a [u64],
}

impl Power<'_> {
    #[inline(always)]
    pub(crate) unsafe fn get_power<'a>(dv_ind: u32) -> Result<Power<'a>, RocmErr> {
        let mut sensor_count = 0;
        let mut ave = malloc(size_of::<u64>()).cast();
        let mut cap = malloc(size_of::<u64>()).cast();
        let mut max = malloc(size_of::<u64>()).cast();
        let mut min = malloc(size_of::<u64>()).cast();

        rsmi_dev_power_ave_get(dv_ind, sensor_count, ave).try_err()?;
        rsmi_dev_power_cap_get(dv_ind, sensor_count, cap).try_err()?;
        rsmi_dev_power_cap_range_get(dv_ind, sensor_count, max, min).try_err()?;

        sensor_count += 1;
        let mut counter = sensor_count as usize + 1;
        loop {
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

            if ret_ave != RocmErr::RsmiStatusSuccess
                || ret_cap != RocmErr::RsmiStatusSuccess
                || ret_rng != RocmErr::RsmiStatusSuccess
            {
                break;
            }
            sensor_count += 1;
            counter += 1;
        }

        let mut default_power_cap = 0u64;
        rsmi_dev_power_cap_default_get(dv_ind, &mut default_power_cap as *mut u64).try_err()?;

        Ok(Power {
            sensor_count: sensor_count,
            default_power_cap,
            power_per_sensor: from_raw_parts(ave, sensor_count as usize),
            power_cap_per_sensor: from_raw_parts(cap, sensor_count as usize),
            power_cap_min_sensor: from_raw_parts(min, sensor_count as usize),
            power_cap_max_sensor: from_raw_parts(max, sensor_count as usize),
        })
    }
}
