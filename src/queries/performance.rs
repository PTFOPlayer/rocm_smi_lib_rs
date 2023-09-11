use crate::{bindings::{util_counters, Check}, error::RocmErr};

#[derive(Debug)]
pub struct PerformanceCounters {
    pub counter_gfx: u64,
    pub counter_mem: u64,
}

impl PerformanceCounters{
    pub unsafe fn get_counters(dv_ind: u32) -> Result<Self, RocmErr>{
        let counters = util_counters(dv_ind).check()?;
        Ok(Self { counter_gfx: counters.counter_gfx, counter_mem: counters.counter_mem })
    }
}