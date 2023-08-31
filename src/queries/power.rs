use std::slice::from_raw_parts;

use crate::{bindings::*, error::RocmErr};

#[derive(Debug, Clone)]
pub struct Power<'a> {
    pub sensor_count: u16,
    pub default_power_cap: u64,
    pub power_per_sensor: &'a [u64],
    pub power_cap_per_sensor: &'a [u64],
    pub power_cap_min_sensor: &'a [u64],
    pub power_cap_max_sensor: &'a [u64],
}

impl Power<'_> {
    #[inline(always)]
    pub(crate) unsafe fn get_power(dv_ind: u32) -> Result<Power<'static>, RocmErr> {
        let data = power_data(dv_ind);
        check_res(data.status)?;
        Ok(Power {
            sensor_count: data.sensors,
            default_power_cap: data.default_power_cap,
            power_per_sensor: from_raw_parts(data.power_per_sensor, data.sensors.into()),
            power_cap_per_sensor: from_raw_parts(data.power_cap_per_sensor, data.sensors.into()),
            power_cap_min_sensor: from_raw_parts(data.power_cap_min_sensor, data.sensors.into()),
            power_cap_max_sensor: from_raw_parts(data.power_cap_max_sensor, data.sensors.into()),
        })
    }
}
