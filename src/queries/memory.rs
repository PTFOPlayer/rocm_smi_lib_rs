use crate::{bindings::*, error::RocmErr};

#[derive(Debug, Clone, Copy)]
pub struct MemoryTotal {
    pub vram: u64,
    pub vis_vram: u64,
    pub gtt: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryUsed {
    pub vram: u64,
    pub vis_vram: u64,
    pub gtt: u64,
}

#[derive(Debug, Clone, Copy)]
pub struct Memory {
    pub vram_total: u64,
    pub vram_used: u64,
    pub vis_vram_total: u64,
    pub vis_vram_used: u64,
    pub gtt_total: u64,
    pub gtt_used: u64,
    pub busy_percent: u32,
}

impl Memory {
    pub(crate) unsafe fn get_memory(dv_ind: u32) -> Result<Memory, RocmErr> {
        let mut vram_total = 0u64;
        let mut vram_used = 0u64;
        rsmi_dev_memory_total_get(
            dv_ind,
            RsmiMemoryType::RsmiMemTypeVram,
            &mut vram_total as *mut u64,
        )
        .try_err()?;
        rsmi_dev_memory_usage_get(
            dv_ind,
            RsmiMemoryType::RsmiMemTypeVram,
            &mut vram_used as *mut u64,
        )
        .try_err()?;

        let mut vis_vram_total = 0u64;
        let mut vis_vram_used = 0u64;
        rsmi_dev_memory_total_get(
            dv_ind,
            RsmiMemoryType::RsmiMemTypeVisVram,
            &mut vis_vram_total as *mut u64,
        )
        .try_err()?;
        rsmi_dev_memory_usage_get(
            dv_ind,
            RsmiMemoryType::RsmiMemTypeVisVram,
            &mut vis_vram_used as *mut u64,
        )
        .try_err()?;

        let mut gtt_total = 0u64;
        let mut gtt_used = 0u64;
        rsmi_dev_memory_total_get(
            dv_ind,
            RsmiMemoryType::RsmiMemTypeGtt,
            &mut gtt_total as *mut u64,
        )
        .try_err()?;
        rsmi_dev_memory_usage_get(
            dv_ind,
            RsmiMemoryType::RsmiMemTypeGtt,
            &mut gtt_used as *mut u64,
        )
        .try_err()?;

        let mut busy_percent = 0u32;
        rsmi_dev_memory_busy_percent_get(dv_ind, &mut busy_percent as *mut u32).try_err()?;

        Ok(Memory {
            busy_percent,
            vram_used,
            vis_vram_used,
            gtt_used,
            vram_total,
            vis_vram_total,
            gtt_total,
        })
    }
}
