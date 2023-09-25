use std::{mem::size_of, slice::from_raw_parts};

use libc::{malloc, realloc};

use crate::{
    bindings::{rsmi_dev_fan_rpms_get, rsmi_dev_fan_speed_get, rsmi_dev_fan_speed_max_get},
    error::RocmErr,
};

#[derive(Debug, Clone, Copy)]
pub struct Fans<'a> {
    pub sensor_count: u32,
    pub fan_rpm_per_sensor: &'a [i64],
    pub fan_speed_per_sensor: &'a [i64],
    pub max_fan_speed_per_sensor: &'a [u64],
}

impl Fans<'_> {
    #[inline(always)]
    pub(crate) unsafe fn get_fans(dv_ind: u32) -> Result<Self, RocmErr> {
        let mut sensor_count = 0;
        let mut rpm = malloc(size_of::<i64>()).cast();
        let mut speed = malloc(size_of::<i64>()).cast();
        let mut speed_max = malloc(size_of::<i64>()).cast();

        rsmi_dev_fan_rpms_get(dv_ind, sensor_count, rpm);
        rsmi_dev_fan_speed_get(dv_ind, sensor_count, speed);
        rsmi_dev_fan_speed_max_get(dv_ind, sensor_count, speed_max);

        sensor_count += 1;
        let mut counter = sensor_count as usize + 1;
        loop {
            rpm = realloc(rpm.cast(), counter * size_of::<i64>()).cast();
            speed = realloc(speed.cast(), counter * size_of::<i64>()).cast();
            speed_max = realloc(speed_max.cast(), counter * size_of::<i64>()).cast();

            let ret_rpm = rsmi_dev_fan_rpms_get(
                dv_ind,
                sensor_count,
                rpm.add(sensor_count as usize * size_of::<i64>()),
            );

            let ret_speed = rsmi_dev_fan_speed_get(
                dv_ind,
                sensor_count,
                speed.add(sensor_count as usize * size_of::<i64>()),
            );

            let ret_speed_max = rsmi_dev_fan_speed_max_get(
                dv_ind,
                sensor_count,
                speed_max.add(sensor_count as usize * size_of::<i64>()),
            );

            if ret_rpm != RocmErr::RsmiStatusSuccess
                || ret_speed != RocmErr::RsmiStatusSuccess
                || ret_speed_max != RocmErr::RsmiStatusSuccess
            {
                break;
            }

            sensor_count += 1;
            counter += 1;
        }

        let len = sensor_count as usize;
        Ok(Self {
            sensor_count,
            fan_rpm_per_sensor: from_raw_parts(rpm, len),
            fan_speed_per_sensor: from_raw_parts(speed, len),
            max_fan_speed_per_sensor: from_raw_parts(speed_max, len),
        })
    }
}
