use crate::{
    bindings::{
        rsmi_dev_gpu_clk_freq_get, rsmi_dev_gpu_metrics_info_get, rsmi_dev_mem_overdrive_level_get,
        rsmi_dev_overdrive_level_get, rsmi_dev_perf_level_get, rsmi_utilization_count_get,
        GpuMetrics, PerformanceLevel, RsmiClkType, RsmiFrequenciesT,
        RsmiUtilizationCounterT, RsmiUtilizationCounterType, rsmi_dev_od_volt_info_get, RsmiOdVoltFreqDataT, RsmiRange, RsmiOdVddcPoint,
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
        let count_gfx = RsmiUtilizationCounterT {
            counter_type: RsmiUtilizationCounterType::RsmiCoarseGrainGfxActivity,
            value: 0,
        };
        let count_mem = RsmiUtilizationCounterT {
            counter_type: RsmiUtilizationCounterType::RsmiCoarseGrainMemActivity,
            value: 0,
        };
        let mut timestamp = 0u64;
        let mut data = [count_gfx, count_mem];
        rsmi_utilization_count_get(dv_ind, data.as_mut_ptr(), 2, &mut timestamp as *mut u64)
            .try_err()?;

        Ok(Self {
            counter_gfx: data[0].value,
            counter_mem: data[1].value,
        })
    }
}

impl PerformanceLevel {
    pub(crate) unsafe fn get_performance_level(dv_ind: u32) -> Result<Self, RocmErr> {
        let mut level = PerformanceLevel::Unknown;
        rsmi_dev_perf_level_get(dv_ind, &mut level as *mut PerformanceLevel).try_err()?;
        Ok(level)
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
        let mut graphics = 0u32;
        rsmi_dev_overdrive_level_get(dv_ind, &mut graphics as *mut u32);
        let mut memory = 0u32;
        rsmi_dev_mem_overdrive_level_get(dv_ind, &mut memory as *mut u32);
        Ok(Self { graphics, memory })
    }
}

#[derive(Debug)]
pub struct Frequency {
    pub clk_type: RsmiClkType,
    pub current: u64,
    pub supported: Vec<u64>,
}

impl Frequency {
    pub(crate) unsafe fn get_freq(
        dv_ind: u32,
        clk_type: RsmiClkType,
    ) -> Result<Frequency, RocmErr> {
        let mut clk = RsmiFrequenciesT::default();
        rsmi_dev_gpu_clk_freq_get(dv_ind, clk_type, &mut clk as *mut RsmiFrequenciesT).try_err()?;
        Ok(Frequency {
            clk_type,
            current: clk.frequency[clk.current as usize],
            supported: clk.frequency[0..clk.num_supported as usize].into(),
        })
    }
}

#[derive(Debug)]
pub struct FrequencyVoltageCurv {
    pub sclk_current_range: RsmiRange,
    pub sclk_limits: RsmiRange,
    pub mclk_current_range: RsmiRange,
    pub mclk_limits: RsmiRange,
    pub curve_points: Vec<RsmiOdVddcPoint>,
}

impl FrequencyVoltageCurv {
    pub(crate) unsafe fn get_curve(dv_ind: u32) -> Result<FrequencyVoltageCurv, RocmErr> {
        let mut od_volt = RsmiOdVoltFreqDataT::default();
        rsmi_dev_od_volt_info_get(dv_ind, &mut od_volt as *mut RsmiOdVoltFreqDataT).try_err()?;

        Ok(FrequencyVoltageCurv {
            sclk_current_range: od_volt.curr_sclk_range,
            sclk_limits: od_volt.sclk_freq_limits,
            mclk_current_range: od_volt.curr_mclk_range, 
            mclk_limits:  od_volt.mclk_freq_limits,
            curve_points: od_volt.curve.vc_points[0..od_volt.num_regions as usize].into(),
            
        })
    }
}

pub(crate) unsafe fn get_metrics(dv_ind: u32) -> Result<GpuMetrics, RocmErr> {
    let mut metrics: GpuMetrics = GpuMetrics::default();
    rsmi_dev_gpu_metrics_info_get(dv_ind, &mut metrics as *mut GpuMetrics).try_err()?;
    Ok(metrics)
}
