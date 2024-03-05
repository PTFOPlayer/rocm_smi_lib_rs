use rocm_smi_lib_sys::{

    error::RocmErr, RawRsmi,
};

pub use rocm_smi_lib_sys::bindings::{
    GpuMetrics, RsmiClkType, RsmiFrequencies, RsmiOdVddcPoint, RsmiOdVoltFreqData, RsmiRange,
    RsmiUtilizationCounter, RsmiUtilizationCounterType,
};

#[derive(Debug)]
pub struct PerformanceCounters {
    pub counter_gfx: u64,
    pub counter_mem: u64,
}

impl PerformanceCounters {
    pub(crate) unsafe fn get_counters(raw: &mut RawRsmi, dv_ind: u32) -> Result<Self, RocmErr> {
        let count_gfx = RsmiUtilizationCounter {
            counter_type: RsmiUtilizationCounterType::RsmiCoarseGrainGfxActivity,
            value: 0,
        };
        let count_mem = RsmiUtilizationCounter {
            counter_type: RsmiUtilizationCounterType::RsmiCoarseGrainMemActivity,
            value: 0,
        };
        let mut timestamp = 0u64;
        let mut data = [count_gfx, count_mem];
        raw.rsmi_utilization_count_get(dv_ind, data.as_mut_ptr(), 2, &mut timestamp as *mut u64)
            .try_err()?;

        Ok(Self {
            counter_gfx: data[0].value,
            counter_mem: data[1].value,
        })
    }
}

#[derive(Debug)]
pub struct OverdriveLevels {
    pub graphics: u32,
    pub memory: u32,
}

impl OverdriveLevels {
    pub(crate) unsafe fn get_overdrive_levels(raw: &mut RawRsmi, dv_ind: u32) -> Result<Self, RocmErr> {
        let mut graphics = 0u32;
        raw.rsmi_dev_overdrive_level_get(dv_ind, &mut graphics as *mut u32);
        let mut memory = 0u32;
        raw.rsmi_dev_mem_overdrive_level_get(dv_ind, &mut memory as *mut u32);
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
    pub(crate) unsafe fn get_freq(raw: &mut RawRsmi,
        dv_ind: u32,
        clk_type: RsmiClkType,
    ) -> Result<Frequency, RocmErr> {
        let mut clk = RsmiFrequencies::default();
        raw.rsmi_dev_gpu_clk_freq_get(dv_ind, clk_type, &mut clk as *mut RsmiFrequencies).try_err()?;
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
    pub(crate) unsafe fn get_curve(raw: &mut RawRsmi, dv_ind: u32) -> Result<FrequencyVoltageCurv, RocmErr> {
        let mut od_volt = RsmiOdVoltFreqData::default();
        raw.rsmi_dev_od_volt_info_get(dv_ind, &mut od_volt as *mut RsmiOdVoltFreqData).try_err()?;

        Ok(FrequencyVoltageCurv {
            sclk_current_range: od_volt.curr_sclk_range,
            sclk_limits: od_volt.sclk_freq_limits,
            mclk_current_range: od_volt.curr_mclk_range,
            mclk_limits: od_volt.mclk_freq_limits,
            curve_points: od_volt.curve.vc_points[0..od_volt.num_regions as usize].into(),
        })
    }
}

pub(crate) unsafe fn get_metrics(raw: &mut RawRsmi, dv_ind: u32) -> Result<GpuMetrics, RocmErr> {
    let mut metrics: GpuMetrics = GpuMetrics::default();
    raw.rsmi_dev_gpu_metrics_info_get(dv_ind, &mut metrics as *mut GpuMetrics).try_err()?;
    Ok(metrics)
}
