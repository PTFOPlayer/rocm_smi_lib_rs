use std::slice::from_raw_parts;

use crate::{
    bindings::{
        frequency, overdrive_levels, perf_level, rsmi_dev_gpu_metrics_info_get, util_counters,
        volt_curve, Check, CurvePoint, GpuMetrics, RsmiClkType,
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
    Auto,
    Low,
    High,
    Manual,
    StableStd,
    StablePeak,
    StableMinMclk,
    StableMinSclk,
    Determinism,
    Unknown,
}

impl PerformanceLevel {
    pub(crate) unsafe fn get_performance_level(dv_ind: u32) -> Result<Self, RocmErr> {
        Ok(match perf_level(dv_ind).check()?.data {
            0 => PerformanceLevel::Auto,
            1 => PerformanceLevel::Low,
            2 => PerformanceLevel::High,
            3 => PerformanceLevel::Manual,
            4 => PerformanceLevel::StableStd,
            5 => PerformanceLevel::StablePeak,
            6 => PerformanceLevel::StableMinMclk,
            7 => PerformanceLevel::StableMinSclk,
            8 => PerformanceLevel::Determinism,
            _ => PerformanceLevel::Unknown,
        })
    }
}

impl ToString for PerformanceLevel {
    fn to_string(&self) -> String {
        match self {
            PerformanceLevel::Auto => "performance level: Auto".to_owned(),
            PerformanceLevel::Low => "performance level: Low".to_owned(),
            PerformanceLevel::High => "performance level: High".to_owned(),
            PerformanceLevel::Manual => "performance level: Manual".to_owned(),
            PerformanceLevel::StableStd => "performance level: Stable Std".to_owned(),
            PerformanceLevel::StablePeak => "performance level: Stable Peak".to_owned(),
            PerformanceLevel::StableMinMclk => "performance level: Stable Min MClk".to_owned(),
            PerformanceLevel::StableMinSclk => "performance level: Stable Min SClk".to_owned(),
            PerformanceLevel::Determinism => "performance level: Determinism".to_owned(),
            PerformanceLevel::Unknown => "performance level: Unknown".to_owned(),
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
        let curve_points = from_raw_parts(data.points, data.num_regions as usize);

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
            curve_points,
        })
    }
}

pub(crate) unsafe fn get_metrics(dv_ind: u32) -> Result<GpuMetrics, RocmErr> {
    let mut metrics: GpuMetrics = GpuMetrics::default();
    rsmi_dev_gpu_metrics_info_get(dv_ind, &mut metrics as *mut GpuMetrics).try_err()?;
    Ok(metrics)
}
