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
    pub total: MemoryTotal,
    pub used: MemoryUsed,
    pub busy_percent: u32,
}

impl Memory {
    pub(crate) unsafe fn get_memory(dv_ind: u32) -> Result<Memory, RocmErr> {
        Ok(Memory {
            total: MemoryTotal {
                vram: mem_total_vram(dv_ind).check()?.data,
                vis_vram: mem_total_vis_vram(dv_ind).check()?.data,
                gtt: mem_total_gtt(dv_ind).check()?.data,
            },
            used: MemoryUsed {
                vram: mem_used_vram(dv_ind).check()?.data,
                vis_vram: mem_used_vis_vram(dv_ind).check()?.data,
                gtt: mem_used_gtt(dv_ind).check()?.data,
            },
            busy_percent: memory_busy_percent(dv_ind).check()?.data,
        })
    }
}
