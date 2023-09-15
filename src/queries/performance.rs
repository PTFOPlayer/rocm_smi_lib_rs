use std::slice::from_raw_parts;

use crate::{
    bindings::{
        frequency, overdrive_levels, perf_level, util_counters, volt_curve, Check, CurvePoint,
        RsmiClkType,
    },
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
                "Rsmi device performance level: Stable Std".to_owned()
            }
            PerformanceLevel::RsmiDevPerfLevelStablePeak => {
                "Rsmi device performance level: Stable Peak".to_owned()
            }
            PerformanceLevel::RsmiDevPerfLevelStableMinMclk => {
                "Rsmi device performance level: Stable Min Memory Clk".to_owned()
            }
            PerformanceLevel::RsmiDevPerfLevelStableMinSclk => {
                "Rsmi device performance level: Stable Min Silicone Clk".to_owned()
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

#[derive(Debug)]
pub struct Frequency<'a> {
    pub clk_type: RsmiClkType,
    pub current: u64,
    pub supported: &'a [u64],
}

impl Frequency<'_> {
    pub(crate) unsafe fn get_freq<'a>(
        dv_ind: u32,
        clk_type: RsmiClkType,
    ) -> Result<Frequency<'a>, RocmErr> {
        let data = frequency(dv_ind, clk_type).check()?;
        let slice = from_raw_parts(data.frequency, data.num_supported as usize);
        Ok(Frequency {
            clk_type,
            current: slice[data.current as usize],
            supported: slice,
        })
    }
}

#[derive(Debug)]
pub struct ClkRange {
    pub upper_limit: u64,
    pub lower_limit: u64,
}

#[derive(Debug)]
pub struct FrequencyVoltageCurv<'a> {
    pub sclk_current_range: ClkRange,
    pub sclk_limits: ClkRange,
    pub mclk_current_range: ClkRange,
    pub mclk_limits: ClkRange,
    pub curve_points: &'a [CurvePoint],
}

impl FrequencyVoltageCurv<'_> {
    pub(crate) unsafe fn get_curve<'a>(dv_ind: u32) -> Result<FrequencyVoltageCurv<'a>, RocmErr> {
        let data = volt_curve(dv_ind).check()?;
        let slice = from_raw_parts(data.points, data.num_regions as usize);

        Ok(FrequencyVoltageCurv {
            sclk_current_range: ClkRange {
                upper_limit: data.curr_sclk_range_max,
                lower_limit: data.curr_sclk_range_min,
            },
            sclk_limits: ClkRange {
                upper_limit: data.sclk_limit_max,
                lower_limit: data.sclk_limit_min,
            },
            mclk_current_range: ClkRange {
                upper_limit: data.curr_mclk_range_max,
                lower_limit: data.curr_mclk_range_min,
            },
            mclk_limits: ClkRange {
                upper_limit: data.mclk_limit_max,
                lower_limit: data.mclk_limit_min,
            },
            curve_points: slice,
        })
    }
}
