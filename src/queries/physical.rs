use std::slice::from_raw_parts;

use crate::{
    bindings::{fans, Check},
    error::RocmErr,
};

#[derive(Debug, Clone, Copy)]
pub struct Fans<'a> {
    pub sensor_count: u16,
    pub fan_rpm_per_sensor: &'a [i64],
    pub fan_speed_per_sensor: &'a [i64],
    pub max_fan_speed_per_sensor: &'a [u64],
}

impl Fans<'_> {
    #[inline(always)]
    pub(crate) unsafe fn get_fans(dv_ind: u32) -> Result<Self, RocmErr> {
        let data = fans(dv_ind).check()?;
        let len = data.sensors as usize;
        Ok(Self {
            sensor_count: data.sensors,
            fan_rpm_per_sensor: from_raw_parts(data.fan_rpm_per_sensor, len),
            fan_speed_per_sensor: from_raw_parts(data.fan_speed_per_sensor, len),
            max_fan_speed_per_sensor: from_raw_parts(data.max_fan_speed_per_sensor, len),
        })
    }
}
