use libc::c_char;

use crate::error::RocmErr;

#[link(name = "rsmi64", kind = "static")]
extern "C" {
    // identifiers
    pub(crate) fn init_c() -> u16;
    pub(crate) fn num_devices() -> ResultUint32T;
    pub(crate) fn device_id(dv_ind: u32) -> ResultUint16T;
    pub(crate) fn device_name(dv_ind: u32) -> ResultStr;
    pub(crate) fn device_vendor_id(dv_ind: u32) -> ResultUint16T;
    pub(crate) fn device_brand(dv_ind: u32) -> ResultStr;
    pub(crate) fn device_vendor_name(dv_ind: u32) -> ResultStr;
    pub(crate) fn device_vram_vendor_name(dv_ind: u32) -> ResultStr;
    pub(crate) fn device_serial(dv_ind: u32) -> ResultStr;
    pub(crate) fn device_subsystem_id(dv_ind: u32) -> ResultUint16T;
    pub(crate) fn device_subsystem_name(dv_ind: u32) -> ResultStr;
    pub(crate) fn device_drm_render(dv_ind: u32) -> ResultUint32T;
    pub(crate) fn device_subsystem_vendor_id(dv_ind: u32) -> ResultUint16T;
    pub(crate) fn device_unique_id(dv_ind: u32) -> ResultUint64T;

    // pcie
    pub(crate) fn pci_bandwidth(dv_ind: u32) -> ResultPcieBandwidth;
    pub(crate) fn pcie_id(dv_ind: u32) -> ResultUint64T;
    pub(crate) fn topo_numa_affinity(dv_ind: u32) -> ResultUint32T;
    pub(crate) fn pci_throughput(dv_ind: u32) -> ResultPcieThroughput;

    // power
    pub(crate) fn power_data(dv_ind: u32) -> ResultPower;

    // memory
    pub(crate) fn mem_total_vram(dv_ind: u32) -> ResultUint64T;
    pub(crate) fn mem_total_vis_vram(dv_ind: u32) -> ResultUint64T;
    pub(crate) fn mem_total_gtt(dv_ind: u32) -> ResultUint64T;
    pub(crate) fn mem_used_vram(dv_ind: u32) -> ResultUint64T;
    pub(crate) fn mem_used_vis_vram(dv_ind: u32) -> ResultUint64T;
    pub(crate) fn mem_used_gtt(dv_ind: u32) -> ResultUint64T;
    pub(crate) fn memory_busy_percent(dv_ind: u32) -> ResultUint32T;

    //physical
    pub(crate) fn fans(dv_ind: u32) -> ResultFans;
    pub(crate) fn temperature(
        dv_ind: u32,
        sensor: RsmiTemperatureSensor,
        metric: RsmiTemperatureMetric,
    ) -> ResultInt64T;
    pub(crate) fn voltage(dv_ind: u32, metric: RsmiVoltageMetric) -> ResultInt64T;

    //performance
    pub(crate) fn util_counters(dv_ind: u32) -> ResultUtilCounter;
    pub(crate) fn busy_percent(dv_ind: u32) -> ResultUint32T;
    pub(crate) fn perf_level(dv_ind: u32) -> ResultUint32T;
    pub(crate) fn overdrive_levels(dv_ind: u32) -> ResultOverdriveLevels;
    pub(crate) fn frequency(dv_ind: u32, clk_type: RsmiClkType) -> ResultFrequencies;
    pub(crate) fn volt_curve(dv_ind: u32) -> ResultVoltCurve;
    pub(crate) fn metrics(dv_ind: u32) -> ResultGpuMetrics;
}

#[repr(C)]
pub enum RsmiTemperatureMetric {
    RsmiTempCurrent,
    RsmiTempMax,
    RsmiTempMin,
    RsmiTempMaxHyst,
    RsmiTempMinHyst,
    RsmiTempCritical,
    RsmiTempCriticalHyst,
    RsmiTempEmergency,
    RsmiTempEmergencyHyst,
    RsmiTempCritMin,
    RsmiTempCritMinHyst,
    RsmiTempOffset,
    RsmiTempLowest,
    RsmiTempHighest,
}

#[repr(C)]
pub enum RsmiTemperatureSensor {
    RsmiTempTypeEdge,
    RsmiTempTypeJunction,
    RsmiTempTypeMemory,
    RsmiTempTypeHbm0,
    RsmiTempTypeHbm1,
    RsmiTempTypeHbm2,
    RsmiTempTypeHbm3,
    RsmiTempTypeInvalid,
}

#[repr(C)]
pub enum RsmiVoltageMetric {
    RsmiVoltCurrent,
    RsmiVoltMax,
    RsmiVoltMinCrit,
    RsmiVoltMin,
    RsmiVoltMaxCrit,
    RsmiVoltAverage,
    RsmiVoltLowest,
    RsmiVoltHighest,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RsmiClkType {
    RsmiClkTypeSys,
    RsmiClkTypeDf,
    RsmiClkTypeDcef,
    RsmiClkTypeSoc,
    RsmiClkTypeMem,
    RsmiClkTypePcie,
}

#[repr(C)]
pub(crate) struct ResultStr {
    pub(crate) status: u16,
    pub(crate) data: *mut c_char,
}

#[repr(C)]
pub(crate) struct ResultInt64T {
    pub(crate) status: u16,
    pub(crate) data: i64,
}

#[repr(C)]
pub(crate) struct ResultUint64T {
    pub(crate) status: u16,
    pub(crate) data: u64,
}

#[repr(C)]
pub(crate) struct ResultUint32T {
    pub(crate) status: u16,
    pub(crate) data: u32,
}

#[repr(C)]
pub(crate) struct ResultUint16T {
    pub(crate) status: u16,
    pub(crate) data: u16,
}

#[repr(C)]
pub(crate) struct ResultPcieBandwidth {
    pub(crate) status: u16,
    pub(crate) current: u32,
    pub(crate) num_supported: u32,
    pub(crate) lines: *const u32,
    pub(crate) frequencies: *const u64,
}

#[repr(C)]
pub(crate) struct ResultPcieThroughput {
    pub(crate) status: u16,
    pub(crate) sent: u64,
    pub(crate) recived: u64,
    pub(crate) max_pkg_size: u64,
}

#[repr(C)]
pub(crate) struct ResultPower {
    pub(crate) status: u16,
    pub(crate) sensors: u16,
    pub(crate) default_power_cap: u64,
    pub(crate) power_per_sensor: *const u64,
    pub(crate) power_cap_per_sensor: *const u64,
    pub(crate) power_cap_min_sensor: *const u64,
    pub(crate) power_cap_max_sensor: *const u64,
}

#[repr(C)]
pub(crate) struct ResultFans {
    pub(crate) status: u16,
    pub(crate) sensors: u16,
    pub(crate) fan_rpm_per_sensor: *const i64,
    pub(crate) fan_speed_per_sensor: *const i64,
    pub(crate) max_fan_speed_per_sensor: *const u64,
}

#[repr(C)]
pub(crate) struct ResultUtilCounter {
    pub(crate) status: u16,
    pub(crate) counter_gfx: u64,
    pub(crate) counter_mem: u64,
}

#[repr(C)]
pub(crate) struct ResultOverdriveLevels {
    pub(crate) status: u16,
    pub(crate) graphics: u32,
    pub(crate) memory: u32,
}

#[repr(C)]
pub(crate) struct ResultFrequencies {
    pub(crate) status: u16,
    pub(crate) num_supported: u32,
    pub(crate) current: u32,
    pub(crate) frequency: *mut u64,
}

#[repr(C)]
#[derive(Debug)]
pub struct CurvePoint {
    frequency: u64,
    voltage: u64,
}

#[repr(C)]
pub(crate) struct ResultVoltCurve {
    pub(crate) status: u16,
    pub(crate) num_regions: u32,
    pub(crate) curr_sclk_range_min: u64,
    pub(crate) curr_sclk_range_max: u64,
    pub(crate) sclk_limit_min: u64,
    pub(crate) sclk_limit_max: u64,
    pub(crate) curr_mclk_range_min: u64,
    pub(crate) curr_mclk_range_max: u64,
    pub(crate) mclk_limit_min: u64,
    pub(crate) mclk_limit_max: u64,
    pub(crate) points: *mut CurvePoint,
}

#[repr(C)]
#[derive(Debug)]
pub(crate) struct ResultGpuMetrics {
    pub(crate) status: u16,
    pub(crate) metrics: GpuMetrics,
}

#[repr(C)]
#[derive(Debug)]
pub struct MeticHeader {
    pub structure_size: u16,
    pub format_revision: u8,
    pub content_revision: u8,
}

#[repr(C)]
#[derive(Debug)]
pub struct GpuMetrics {
    /// metric header
    pub headers: MeticHeader,
    /// Temperature
    pub temperature_edge: u16,
    pub temperature_hotspot: u16,
    pub temperature_mem: u16,
    pub temperature_vrgfx: u16,
    pub temperature_vrsoc: u16,
    pub temperature_vrmem: u16,
    /// Utilization
    pub average_gfx_activity: u16,
    pub average_umc_activity: u16, // memory controller
    pub average_mm_activity: u16,  // UVD or VCN
    /// Power/Energy
    pub average_socket_power: u16,
    pub energy_accumulator: u64,
    /// Driver attached timestamp (in ns)
    pub system_clock_counter: u64,
    /// Average clocks
    pub average_gfxclk_frequency: u16,
    /// needs filter
    pub average_socclk_frequency: u16,
    pub average_uclk_frequency: u16,
    pub average_vclk0_frequency: u16,
    pub average_dclk0_frequency: u16,
    pub average_vclk1_frequency: u16,
    pub average_dclk1_frequency: u16,
    /// Current clocks
    pub current_gfxclk: u16,
    /// needs filter
    pub current_socclk: u16,
    /// needs filter
    pub current_uclk: u16,
    pub current_vclk0: u16,
    pub current_dclk0: u16,
    pub current_vclk1: u16,
    pub current_dclk1: u16,

    pub throttle_status: u32,

    pub current_fan_speed: u16,

    pub pcie_link_width: u16,
    pub pcie_link_speed: u16,

    /// not sure what it is for
    /// needs filter
    pub padding: u16,

    pub gfx_activity_acc: u32,
    pub mem_actvity_acc: u32,
    /// needs filter
    pub temperature_hbm: [u16; 4],
}

pub(crate) trait Check: Sized {
    fn check(self) -> Result<Self, RocmErr>;
}

macro_rules! auto_impl {
    ($($t:ty),+) => {
        $(impl Check for $t {
            #[inline(always)]
            fn check(self) -> Result<Self, RocmErr>
            {
                if self.status != 0 {
                    return Err(RocmErr::from_u16(self.status));
                }
                Ok(self)
            }
        })*
    }
}

auto_impl!(
    ResultUint16T,
    ResultUint32T,
    ResultUint64T,
    ResultInt64T,
    ResultStr,
    ResultPcieBandwidth,
    ResultPcieThroughput,
    ResultPower,
    ResultFans,
    ResultUtilCounter,
    ResultOverdriveLevels,
    ResultFrequencies,
    ResultVoltCurve,
    ResultGpuMetrics
);

impl ResultStr {
    #[inline(always)]
    pub(crate) fn into_string(self) -> Result<String, RocmErr> {
        let c_str = unsafe { std::ffi::CStr::from_ptr(self.data) };
        let data = c_str.to_str().to_owned();
        match data {
            Ok(res) => Ok(res.to_owned()),
            Err(_) => Err(RocmErr::RsmiStringConversionError),
        }
    }
}
