use crate::error::RocmErr;

// Guaranteed maximum possible number of supported frequencies
pub const RSMI_MAX_NUM_FREQUENCIES: usize = 32;

// Maximum possible value for fan speed. Should be used as the denominator
// when determining fan speed percentage.
pub const RSMI_MAX_FAN_SPEED: usize = 255;

// The number of points that make up a voltage-frequency curve definition
pub const RSMI_NUM_VOLTAGE_CURVE_POINTS: usize = 3;

#[link(name = "rsmi64", kind = "static")]
extern "C" {
    // init/shutdown
    pub fn rsmi_init(init_status: u32) -> RocmErr;
    pub fn rsmi_shut_down() -> RocmErr;
    // identifiers
    pub fn rsmi_num_monitor_devices(num_devices: *mut u32) -> RocmErr;
    pub fn rsmi_dev_id_get(dv_ind: u32, id: *mut u16) -> RocmErr;
    pub fn rsmi_dev_name_get(dv_ind: u32, name: *mut i8, name_length: usize) -> RocmErr;
    pub fn rsmi_dev_vendor_id_get(dv_ind: u32, id: *mut u16) -> RocmErr;
    pub fn rsmi_dev_brand_get(dv_ind: u32, brand: *mut i8, name_length: usize) -> RocmErr;
    pub fn rsmi_dev_vendor_name_get(
        dv_ind: u32,
        vendor: *mut i8,
        name_length: usize,
    ) -> RocmErr;
    pub fn rsmi_dev_vram_vendor_get(
        dv_ind: u32,
        vendor: *mut i8,
        name_length: usize,
    ) -> RocmErr;
    pub fn rsmi_dev_serial_number_get(
        dv_ind: u32,
        serial_number: *mut i8,
        name_length: usize,
    ) -> RocmErr;
    pub fn rsmi_dev_subsystem_id_get(dv_ind: u32, id: *mut u16) -> RocmErr;
    pub fn rsmi_dev_subsystem_name_get(
        dv_ind: u32,
        subsystem_name: *mut i8,
        name_length: usize,
    ) -> RocmErr;
    pub fn rsmi_dev_drm_render_minor_get(dv_ind: u32, render_minor: *mut u32) -> RocmErr;
    pub fn rsmi_dev_subsystem_vendor_id_get(
        dv_ind: u32,
        subsystem_vendor_id: *mut u16,
    ) -> RocmErr;
    pub fn rsmi_dev_unique_id_get(dv_ind: u32, unique_id: *mut u64) -> RocmErr;

    // pcie

    pub fn rsmi_dev_pci_bandwidth_get(
        dv_ind: u32,
        bandwidth: *mut RsmiPcieBandwidthT,
    ) -> RocmErr;
    pub fn rsmi_dev_pci_id_get(dv_ind: u32, id: *mut u64) -> RocmErr;
    pub fn rsmi_topo_numa_affinity_get(dv_ind: u32, numa: *mut u32) -> RocmErr;
    pub fn rsmi_dev_pci_throughput_get(
        dv_ind: u32,
        sent: *mut u64,
        received: *mut u64,
        max_pkt_sz: *mut u64,
    ) -> RocmErr;

    // power
    pub fn rsmi_dev_power_ave_get(dv_ind: u32, sensor: u32, ave: *mut u64) -> RocmErr;
    pub fn rsmi_dev_power_cap_get(dv_ind: u32, sensor: u32, cap: *mut u64) -> RocmErr;
    pub fn rsmi_dev_power_cap_range_get(
        dv_ind: u32,
        sensor: u32,
        max: *mut u64,
        min: *mut u64,
    ) -> RocmErr;
    pub fn rsmi_dev_power_cap_default_get(dv_ind: u32, default: *mut u64) -> RocmErr;

    // memory
    pub fn rsmi_dev_memory_total_get(
        dv_ind: u32,
        mem_type: RsmiMemoryType,
        total: *mut u64,
    ) -> RocmErr;
    pub fn rsmi_dev_memory_usage_get(
        dv_ind: u32,
        mem_type: RsmiMemoryType,
        usage: *mut u64,
    ) -> RocmErr;
    pub fn rsmi_dev_memory_busy_percent_get(dv_ind: u32, percent: *mut u32) -> RocmErr;

    //physical
    pub fn rsmi_dev_fan_rpms_get(dv_ind: u32, sensor: u32, rpm: *mut i64) -> RocmErr;
    pub fn rsmi_dev_fan_speed_get(dv_ind: u32, sensor: u32, speed: *mut i64) -> RocmErr;
    pub fn rsmi_dev_fan_speed_max_get(
        dv_ind: u32,
        sensor: u32,
        speed_max: *mut u64,
    ) -> RocmErr;
    pub fn rsmi_dev_temp_metric_get(
        dv_ind: u32,
        sensor: RsmiTemperatureSensor,
        metric: RsmiTemperatureMetric,
        temperature: *mut i64,
    ) -> RocmErr;
    pub fn rsmi_dev_volt_metric_get(
        dv_ind: u32,
        voltage_type: RsmiVoltageTypeT,
        metric: RsmiVoltageMetric,
        volt: *mut i64,
    ) -> RocmErr;

    //performance
    pub fn rsmi_dev_busy_percent_get(dv_ind: u32, percent: *mut u32) -> RocmErr;
    pub fn rsmi_dev_perf_level_get(dv_ind: u32, level: *mut PerformanceLevel) -> RocmErr;
    pub fn rsmi_utilization_count_get(
        dv_ind: u32,
        counter: *mut RsmiUtilizationCounterT,
        count: u32,
        timestamp: *mut u64,
    ) -> RocmErr;
    pub fn rsmi_dev_gpu_clk_freq_get(
        dv_ind: u32,
        clk_type: RsmiClkType,
        clk: *mut RsmiFrequenciesT,
    ) -> RocmErr;

    pub fn rsmi_dev_overdrive_level_get(dv_ind: u32, level: *mut u32) -> RocmErr;
    pub fn rsmi_dev_mem_overdrive_level_get(dv_ind: u32, level: *mut u32) -> RocmErr;

    pub fn rsmi_dev_od_volt_info_get(dv_ind: u32, ov_volt: *mut RsmiOdVoltFreqDataT) -> RocmErr;

    pub fn rsmi_dev_gpu_metrics_info_get(dv_ind: u32, metrics: *mut GpuMetrics) -> RocmErr;
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
#[derive(Debug, Clone, Copy)]
pub enum RsmiMemoryType {
    RsmiMemTypeVram,
    RsmiMemTypeVisVram,
    RsmiMemTypeGtt,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub enum RsmiVoltageTypeT {
    RsmiVoltTypeVddgfx = 0,
    #[default]
    RsmiVoltTypeInvalid = 0xFFFFFFFF,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
    Unknown = 0x100,
}

#[repr(C)]
#[derive(Default)]
pub struct RsmiFrequenciesT {
    pub num_supported: u32,
    pub current: u32,
    pub frequency: [u64; RSMI_MAX_NUM_FREQUENCIES],
}

#[repr(C)]
#[derive(Default)]
pub struct RsmiPcieBandwidthT {
    pub transfer_rate: RsmiFrequenciesT,
    pub lanes: [u32; RSMI_MAX_NUM_FREQUENCIES],
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RsmiUtilizationCounterType {
    RsmiCoarseGrainGfxActivity,
    RsmiCoarseGrainMemActivity,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct RsmiUtilizationCounterT {
    pub counter_type: RsmiUtilizationCounterType,
    pub value: u64,
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct RsmiRange {
    pub lower_bound: u32,
    pub upper_bound: u32,
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct RsmiOdVddcPoint {
    pub frequency: u64,
    pub voltage: u64,
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct RsmiOdVoltCurveT {
    pub vc_points: [RsmiOdVddcPoint; RSMI_NUM_VOLTAGE_CURVE_POINTS],
}

#[repr(C)]
#[derive(Debug, Default, Clone)]
pub struct RsmiOdVoltFreqDataT {
    pub curr_sclk_range: RsmiRange,
    pub curr_mclk_range: RsmiRange,
    pub sclk_freq_limits: RsmiRange,
    pub mclk_freq_limits: RsmiRange,
    pub curve: RsmiOdVoltCurveT,
    pub num_regions: u32,
}

#[inline(always)]
pub unsafe fn string_from_fn(
    dv_ind: u32,
    name_size: usize,
    f: unsafe extern "C" fn(u32, *mut i8, usize) -> RocmErr,
) -> Result<String, RocmErr> {
    let buff = libc::malloc(name_size).cast();
    f(dv_ind, buff, name_size).try_err()?;
    let temp = std::ffi::CString::from_raw(buff);
    return Ok(temp.to_string_lossy().to_string());
}
