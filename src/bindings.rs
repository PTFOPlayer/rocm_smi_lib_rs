use crate::error::RocmErr;

// Guaranteed maximum possible number of supported frequencies
const RSMI_MAX_NUM_FREQUENCIES: usize = 32;

// Maximum possible value for fan speed. Should be used as the denominator
// when determining fan speed percentage.
const RSMI_MAX_FAN_SPEED: usize = 255;

// The number of points that make up a voltage-frequency curve definition
const RSMI_NUM_VOLTAGE_CURVE_POINTS: usize = 3;

#[link(name = "rsmi64", kind = "static")]
extern "C" {
    // identifiers
    pub(crate) fn rsmi_init(init_status: u32) -> RocmErr;
    pub(crate) fn rsmi_shut_down() -> RocmErr;
    pub(crate) fn rsmi_num_monitor_devices(num_devices: *mut u32) -> RocmErr;
    pub(crate) fn rsmi_dev_id_get(dv_ind: u32, id: *mut u16) -> RocmErr;
    pub(crate) fn rsmi_dev_name_get(dv_ind: u32, name: *mut i8, name_length: usize) -> RocmErr;
    pub(crate) fn rsmi_dev_vendor_id_get(dv_ind: u32, id: *mut u16) -> RocmErr;
    pub(crate) fn rsmi_dev_brand_get(dv_ind: u32, brand: *mut i8, name_length: usize) -> RocmErr;
    pub(crate) fn rsmi_dev_vendor_name_get(
        dv_ind: u32,
        vendor: *mut i8,
        name_length: usize,
    ) -> RocmErr;
    pub(crate) fn rsmi_dev_vram_vendor_get(
        dv_ind: u32,
        vendor: *mut i8,
        name_length: usize,
    ) -> RocmErr;
    pub(crate) fn rsmi_dev_serial_number_get(
        dv_ind: u32,
        serial_number: *mut i8,
        name_length: usize,
    ) -> RocmErr;
    pub(crate) fn rsmi_dev_subsystem_id_get(dv_ind: u32, id: *mut u16) -> RocmErr;
    pub(crate) fn rsmi_dev_subsystem_name_get(
        dv_ind: u32,
        subsystem_name: *mut i8,
        name_length: usize,
    ) -> RocmErr;
    pub(crate) fn rsmi_dev_drm_render_minor_get(dv_ind: u32, render_minor: *mut u32) -> RocmErr;
    pub(crate) fn rsmi_dev_subsystem_vendor_id_get(
        dv_ind: u32,
        subsystem_vendor_id: *mut u16,
    ) -> RocmErr;
    pub(crate) fn rsmi_dev_unique_id_get(dv_ind: u32, unique_id: *mut u64) -> RocmErr;

    // pcie

    pub(crate) fn rsmi_dev_pci_bandwidth_get(
        dv_ind: u32,
        bandwidth: *mut RsmiPcieBandwidthT,
    ) -> RocmErr;

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
    pub(crate) fn rsmi_dev_gpu_metrics_info_get(dv_ind: u32, metrics: *mut GpuMetrics) -> RocmErr;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
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
pub(crate) struct ResultPcieThroughput {
    pub(crate) status: u16,
    pub(crate) sent: u64,
    pub(crate) recived: u64,
    pub(crate) max_pkg_size: u64,
}

#[repr(C)]
#[derive(Default)]
pub(crate) struct RsmiFrequenciesT {
    pub(crate) num_supported: u32,
    pub(crate) current: u32,
    pub(crate) frequency: [u64; RSMI_MAX_NUM_FREQUENCIES],
}

#[repr(C)]
#[derive(Default)]
pub(crate) struct RsmiPcieBandwidthT {
    pub(crate) transfer_rate: RsmiFrequenciesT,
    pub(crate) lanes: [u32; RSMI_MAX_NUM_FREQUENCIES],
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Default, Clone)]
pub struct MeticHeader {
    pub structure_size: u16,
    pub format_revision: u8,
    pub content_revision: u8,
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
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
    ResultUint32T,
    ResultUint64T,
    ResultInt64T,
    ResultPcieThroughput,
    ResultPower,
    ResultFans,
    ResultUtilCounter,
    ResultOverdriveLevels,
    ResultFrequencies,
    ResultVoltCurve
);

#[inline(always)]
pub(crate) unsafe fn string_from_fn(
    dv_ind: u32,
    name_size: usize,
    f: unsafe extern "C" fn(u32, *mut i8, usize) -> RocmErr,
) -> Result<String, RocmErr> {
    let buff = libc::malloc(name_size).cast();
    f(dv_ind, buff, name_size).try_err()?;
    let temp = std::ffi::CString::from_raw(buff);
    return Ok(temp.to_string_lossy().to_string());
}
