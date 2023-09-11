use crate::{
    bindings::{overdrive_levels, perf_level, util_counters, Check},
    error::RocmErr,
};

#[derive(Debug)]
pub struct PerformanceCounters {
    pub counter_gfx: u64,
    pub counter_mem: u64,
}

impl PerformanceCounters {
    pub(crate) unsafe fn get_counters(dv_ind: u32) -> Result<Self, RocmErr> {
        let counters = util_counters(dv_ind).check()?;
        Ok(Self {
            counter_gfx: counters.counter_gfx,
            counter_mem: counters.counter_mem,
        })
    }
}

#[derive(Debug)]
pub enum PerformanceLevel {
    RsmiDevPerfLevelAuto,
    RsmiDevPerfLevelLow,
    RsmiDevPerfLevelHigh,
    RsmiDevPerfLevelManual,
    RsmiDevPerfLevelStableStd,
    RsmiDevPerfLevelStablePeak,
    RsmiDevPerfLevelStableMinMclk,
    RsmiDevPerfLevelStableMinSclk,
    RsmiDevPerfLevelDeterminism,
    RsmiDevPerfLevelUnknown,
}

impl PerformanceLevel {
    pub(crate) unsafe fn get_performance_level(dv_ind: u32) -> Result<Self, RocmErr> {
        Ok(PerformanceLevel::from_u32(perf_level(dv_ind).check()?.data))
    }
    pub(crate) fn from_u32(level: u32) -> Self {
        match level {
            0 => PerformanceLevel::RsmiDevPerfLevelAuto,
            1 => PerformanceLevel::RsmiDevPerfLevelLow,
            2 => PerformanceLevel::RsmiDevPerfLevelHigh,
            3 => PerformanceLevel::RsmiDevPerfLevelManual,
            4 => PerformanceLevel::RsmiDevPerfLevelStableStd,
            5 => PerformanceLevel::RsmiDevPerfLevelStablePeak,
            6 => PerformanceLevel::RsmiDevPerfLevelStableMinMclk,
            7 => PerformanceLevel::RsmiDevPerfLevelStableMinSclk,
            8 => PerformanceLevel::RsmiDevPerfLevelDeterminism,
            _ => PerformanceLevel::RsmiDevPerfLevelUnknown,
        }
    }
}

impl ToString for PerformanceLevel {
    fn to_string(&self) -> String {
        match self {
            PerformanceLevel::RsmiDevPerfLevelAuto => {
                "Rsmi device performance level: Auto".to_owned()
            }
            PerformanceLevel::RsmiDevPerfLevelLow => {
                "Rsmi device performance level: Low".to_owned()
            }
            PerformanceLevel::RsmiDevPerfLevelHigh => {
                "Rsmi device performance level: High".to_owned()
            }
            PerformanceLevel::RsmiDevPerfLevelManual => {
                "Rsmi device performance level: Manual".to_owned()
            }
            PerformanceLevel::RsmiDevPerfLevelStableStd => {
                "Rsmi device performance level: StableStd".to_owned()
            }
            PerformanceLevel::RsmiDevPerfLevelStablePeak => {
                "Rsmi device performance level: StablePeak".to_owned()
            }
            PerformanceLevel::RsmiDevPerfLevelStableMinMclk => {
                "Rsmi device performance level: StableMinMclk".to_owned()
            }
            PerformanceLevel::RsmiDevPerfLevelStableMinSclk => {
                "Rsmi device performance level: StableMinSclk".to_owned()
            }
            PerformanceLevel::RsmiDevPerfLevelDeterminism => {
                "Rsmi device performance level: Determinism".to_owned()
            }
            PerformanceLevel::RsmiDevPerfLevelUnknown => {
                "Rsmi device performance level: Unknown".to_owned()
            }
        }
    }
}

#[derive(Debug)]
pub struct OverdriveLevels {
    pub graphics: u32,
    pub memory: u32,
}

impl OverdriveLevels {
    pub(crate) unsafe fn get_overdrive_levels(dv_ind: u32) -> Result<Self, RocmErr> {
        let data = overdrive_levels(dv_ind).check()?;
        Ok(Self {
            graphics: data.graphics,
            memory: data.memory,
        })
    }
}
