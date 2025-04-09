pub use rocm_smi_lib_sys::bindings::*;

use crate::error::{IntoRocmErr, RocmErr};

use super::common_structures::{RsmiOdVddcPoint, RsmiRange, RSMI_RANGE_DEFAULT};

#[derive(Debug)]
pub struct PerformanceCounters {
    pub counter_gfx: u64,
    pub counter_mem: u64,
}

impl PerformanceCounters {
    pub(crate) unsafe fn get_counters(dv_ind: u32) -> Result<Self, RocmErr> {
        let count_gfx = rsmi_utilization_counter_t {
            value: 0,
            type_: RSMI_UTILIZATION_COUNTER_TYPE_RSMI_COARSE_GRAIN_GFX_ACTIVITY,
        };
        let count_mem = rsmi_utilization_counter_t {
            value: 0,
            type_: RSMI_UTILIZATION_COUNTER_TYPE_RSMI_COARSE_GRAIN_MEM_ACTIVITY,
        };
        let mut timestamp = 0u64;
        let mut data = [count_gfx, count_mem];
        rsmi_utilization_count_get(dv_ind, data.as_mut_ptr(), 2, &mut timestamp as *mut u64)
            .into_rocm_err()?;

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
    pub(crate) unsafe fn get_overdrive_levels(dv_ind: u32) -> Result<Self, RocmErr> {
        let mut graphics = 0u32;
        rsmi_dev_overdrive_level_get(dv_ind, &mut graphics as *mut u32);
        let mut memory = 0u32;
        rsmi_dev_mem_overdrive_level_get(dv_ind, &mut memory as *mut u32);
        Ok(Self { graphics, memory })
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RsmiClkType {
    /// System clock
    RsmiClkTypeSys = rsmi_clk_type_t_RSMI_CLK_TYPE_SYS as isize,
    /// Data Fabric clock (for ASICs running on a separate clock)
    RsmiClkTypeDf = rsmi_clk_type_t_RSMI_CLK_TYPE_DF as isize,
    /// Display Controller Engine clock
    RsmiClkTypeDcef = rsmi_clk_type_t_RSMI_CLK_TYPE_DCEF as isize,
    /// SOC clock
    RsmiClkTypeSoc = rsmi_clk_type_t_RSMI_CLK_TYPE_SOC as isize,
    /// Memory clock
    RsmiClkTypeMem = rsmi_clk_type_t_RSMI_CLK_TYPE_MEM as isize,
    /// PCIE clock
    RsmiClkTypePcie = rsmi_clk_type_t_RSMI_CLK_TYPE_PCIE as isize,
    /// Invalid clock type
    RsmiClkInvalid = rsmi_clk_type_t_RSMI_CLK_INVALID as isize,
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
        let mut clk = rsmi_frequencies {
            has_deep_sleep: false,
            num_supported: 0,
            current: 0,
            frequency: [0; 33],
        };
        rsmi_dev_gpu_clk_freq_get(dv_ind, clk_type as u32, &mut clk as *mut rsmi_frequencies)
            .into_rocm_err()?;
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
        let mut od_volt = rsmi_od_volt_freq_data {
            curr_sclk_range: RSMI_RANGE_DEFAULT,
            curr_mclk_range: RSMI_RANGE_DEFAULT,
            sclk_freq_limits: RSMI_RANGE_DEFAULT,
            mclk_freq_limits: RSMI_RANGE_DEFAULT,
            curve: rsmi_od_volt_curve_t {
                vc_points: [rsmi_od_vddc_point {
                    frequency: 0,
                    voltage: 0,
                }; 3],
            },
            num_regions: 0,
        };
        rsmi_dev_od_volt_info_get(dv_ind, &mut od_volt as *mut rsmi_od_volt_freq_data)
            .into_rocm_err()?;

        Ok(FrequencyVoltageCurv {
            sclk_current_range: od_volt.curr_sclk_range,
            sclk_limits: od_volt.sclk_freq_limits,
            mclk_current_range: od_volt.curr_mclk_range,
            mclk_limits: od_volt.mclk_freq_limits,
            curve_points: od_volt.curve.vc_points[0..od_volt.num_regions as usize].into(),
        })
    }
}

pub(crate) unsafe fn get_metrics(dv_ind: u32) -> Result<rsmi_gpu_metrics_t, RocmErr> {
    let mut metrics: rsmi_gpu_metrics_t = std::mem::zeroed();
    rsmi_dev_gpu_metrics_info_get(dv_ind, &mut metrics as *mut rsmi_gpu_metrics_t)
        .into_rocm_err()?;
    Ok(metrics)
}
