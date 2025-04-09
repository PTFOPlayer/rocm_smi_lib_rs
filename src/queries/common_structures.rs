use rocm_smi_lib_sys::bindings::{*};

pub type RsmiRange = rsmi_range_t;
pub const RSMI_RANGE_DEFAULT: RsmiRange = RsmiRange {
    lower_bound: 0,
    upper_bound: 0,
};
pub type RsmiOdVddcPoint = rsmi_od_vddc_point;

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum RsmiDevPerfLevel {
    /// Performance level is "auto"
    Auto = rsmi_dev_perf_level_t_RSMI_DEV_PERF_LEVEL_AUTO,
    /// Keep PowerPlay levels "low", regardless of workload
    Low = rsmi_dev_perf_level_t_RSMI_DEV_PERF_LEVEL_LOW,
    /// Keep PowerPlay levels "high", regardless of workload
    High = rsmi_dev_perf_level_t_RSMI_DEV_PERF_LEVEL_HIGH,
    /// Only use values defined by manually setting the RSMI_CLK_TYPE_SYS speed
    Manual = rsmi_dev_perf_level_t_RSMI_DEV_PERF_LEVEL_MANUAL,
    /// Stable power state with profiling clocks
    StableStd = rsmi_dev_perf_level_t_RSMI_DEV_PERF_LEVEL_STABLE_STD,
    /// Stable power state with peak clocks
    StablePeak = rsmi_dev_perf_level_t_RSMI_DEV_PERF_LEVEL_STABLE_PEAK,
    /// Stable power state with minimum memory clock
    StableMinMclk = rsmi_dev_perf_level_t_RSMI_DEV_PERF_LEVEL_STABLE_MIN_MCLK,
    /// Stable power state with minimum system clock
    StableMinSclk = rsmi_dev_perf_level_t_RSMI_DEV_PERF_LEVEL_STABLE_MIN_SCLK,
    /// Performance determinism state
    Determinism = rsmi_dev_perf_level_t_RSMI_DEV_PERF_LEVEL_DETERMINISM,
    /// Unknown performance level
    Unknown = rsmi_dev_perf_level_t_RSMI_DEV_PERF_LEVEL_UNKNOWN,
}

#[derive(Debug, Clone, Copy)]
#[repr(u64)]
pub enum RsmiGpuBlock {
    /// Used to indicate an invalid block
    Invalid = rsmi_gpu_block_t_RSMI_GPU_BLOCK_INVALID,
    /// UMC block
    Umc = rsmi_gpu_block_t_RSMI_GPU_BLOCK_UMC,
    /// SDMA block
    Sdma = rsmi_gpu_block_t_RSMI_GPU_BLOCK_SDMA,
    /// GFX block
    Gfx = rsmi_gpu_block_t_RSMI_GPU_BLOCK_GFX,
    /// MMHUB block
    Mmhub = rsmi_gpu_block_t_RSMI_GPU_BLOCK_MMHUB,
    /// ATHUB block
    Athub = rsmi_gpu_block_t_RSMI_GPU_BLOCK_ATHUB,
    /// PCIE_BIF block
    PcieBif = rsmi_gpu_block_t_RSMI_GPU_BLOCK_PCIE_BIF,
    /// HDP block
    Hdp = rsmi_gpu_block_t_RSMI_GPU_BLOCK_HDP,
    /// XGMI block
    XgmiWafl = rsmi_gpu_block_t_RSMI_GPU_BLOCK_XGMI_WAFL,
    /// DF block
    Df = rsmi_gpu_block_t_RSMI_GPU_BLOCK_DF,
    /// SMN block
    Smn = rsmi_gpu_block_t_RSMI_GPU_BLOCK_SMN,
    /// SEM block
    Sem = rsmi_gpu_block_t_RSMI_GPU_BLOCK_SEM,
    /// MP0 block
    Mp0 = rsmi_gpu_block_t_RSMI_GPU_BLOCK_MP0,
    /// MP1 block
    Mp1 = rsmi_gpu_block_t_RSMI_GPU_BLOCK_MP1,
    /// Fuse block
    Fuse = rsmi_gpu_block_t_RSMI_GPU_BLOCK_FUSE,
    /// Reserved block
    Reserved = rsmi_gpu_block_t_RSMI_GPU_BLOCK_RESERVED,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum RsmiRasErrState {
    /// No ECC errors
    None = rsmi_ras_err_state_t_RSMI_RAS_ERR_STATE_NONE,
    /// ECC is disabled
    Disabled = rsmi_ras_err_state_t_RSMI_RAS_ERR_STATE_DISABLED,
    /// ECC errors present, but type unknown
    Parity = rsmi_ras_err_state_t_RSMI_RAS_ERR_STATE_PARITY,
    /// Single correctable error
    SingC = rsmi_ras_err_state_t_RSMI_RAS_ERR_STATE_SING_C,
    /// Multiple uncorrectable errors
    MultUc = rsmi_ras_err_state_t_RSMI_RAS_ERR_STATE_MULT_UC,
    /// Firmware detected error and isolated page. Treat as uncorrectable
    Poison = rsmi_ras_err_state_t_RSMI_RAS_ERR_STATE_POISON,
    /// ECC is enabled
    Enabled = rsmi_ras_err_state_t_RSMI_RAS_ERR_STATE_ENABLED,
    /// Invalid ECC state
    Invalid = rsmi_ras_err_state_t_RSMI_RAS_ERR_STATE_INVALID,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]
pub enum RsmiVoltageMetric {
    /// Current voltage value
    Current = rsmi_voltage_metric_t_RSMI_VOLT_CURRENT,
    /// Voltage max value
    Max = rsmi_voltage_metric_t_RSMI_VOLT_MAX,
    /// Voltage critical min value
    MinCrit = rsmi_voltage_metric_t_RSMI_VOLT_MIN_CRIT,
    /// Voltage min value
    Min = rsmi_voltage_metric_t_RSMI_VOLT_MIN,
    /// Voltage critical max value
    MaxCrit = rsmi_voltage_metric_t_RSMI_VOLT_MAX_CRIT,
    /// Average voltage
    Average = rsmi_voltage_metric_t_RSMI_VOLT_AVERAGE,
    /// Historical minimum voltage
    Lowest = rsmi_voltage_metric_t_RSMI_VOLT_LOWEST,
    /// Historical maximum voltage
    Highest = rsmi_voltage_metric_t_RSMI_VOLT_HIGHEST,
}

#[derive(Debug, Clone, Copy)]
#[repr(u32)]

pub enum RsmiTemperatureMetric {
    /// Current temperature value
    Current = rsmi_temperature_metric_t_RSMI_TEMP_CURRENT,
    /// Maximum temperature value
    Max = rsmi_temperature_metric_t_RSMI_TEMP_MAX,
    /// Minimum temperature value
    Min = rsmi_temperature_metric_t_RSMI_TEMP_MIN,
    /// Temperature hysteresis value for max limit
    MaxHyst = rsmi_temperature_metric_t_RSMI_TEMP_MAX_HYST,
    /// Temperature hysteresis value for min limit
    MinHyst = rsmi_temperature_metric_t_RSMI_TEMP_MIN_HYST,
    /// Critical maximum temperature value
    Critical = rsmi_temperature_metric_t_RSMI_TEMP_CRITICAL,
    /// Temperature hysteresis value for critical limit
    CriticalHyst = rsmi_temperature_metric_t_RSMI_TEMP_CRITICAL_HYST,
    /// Emergency maximum temperature value
    Emergency = rsmi_temperature_metric_t_RSMI_TEMP_EMERGENCY,
    /// Temperature hysteresis value for emergency limit
    EmergencyHyst = rsmi_temperature_metric_t_RSMI_TEMP_EMERGENCY_HYST,
    /// Critical minimum temperature value
    CritMin = rsmi_temperature_metric_t_RSMI_TEMP_CRIT_MIN,
    /// Temperature hysteresis value for critical minimum limit
    CritMinHyst = rsmi_temperature_metric_t_RSMI_TEMP_CRIT_MIN_HYST,
    /// Temperature offset
    Offset = rsmi_temperature_metric_t_RSMI_TEMP_OFFSET,
    /// Historical minimum temperature
    Lowest = rsmi_temperature_metric_t_RSMI_TEMP_LOWEST,
    /// Invalid temperature metric type
    Invalid = rsmi_temperature_metric_t_RSMI_TEMP_LAST,
}

#[derive(Debug, Clone, Copy)]#[repr(u32)]

pub enum RsmiTemperatureType {
    /// Edge GPU temperature
    Edge = rsmi_temperature_type_t_RSMI_TEMP_TYPE_EDGE,
    /// Junction (hotspot) temperature
    Junction = rsmi_temperature_type_t_RSMI_TEMP_TYPE_JUNCTION,
    /// VRAM temperature
    Memory = rsmi_temperature_type_t_RSMI_TEMP_TYPE_MEMORY,
    /// HBM temperature instance 0
    Hbm0 = rsmi_temperature_type_t_RSMI_TEMP_TYPE_HBM_0,
    /// HBM temperature instance 1
    Hbm1 = rsmi_temperature_type_t_RSMI_TEMP_TYPE_HBM_1,
    /// HBM temperature instance 2
    Hbm2 = rsmi_temperature_type_t_RSMI_TEMP_TYPE_HBM_2,
    /// HBM temperature instance 3
    Hbm3 = rsmi_temperature_type_t_RSMI_TEMP_TYPE_HBM_3,
    /// Invalid type
    Invalid = rsmi_temperature_type_t_RSMI_TEMP_TYPE_INVALID,
}