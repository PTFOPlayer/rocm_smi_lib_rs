use crate::{bindings::*, error::RocmErr};

#[derive(Debug, Clone, Copy)]
pub struct MemoryTotal<T> {
    pub vram: T,
    pub vis_vram: T,
    pub gtt: T,
}

#[derive(Debug, Clone, Copy)]
pub struct MemoryUsed<T> {
    pub vram: T,
    pub vis_vram: T,
    pub gtt: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Memory<T> {
    pub total: MemoryTotal<T>,
    pub used: MemoryUsed<T>,
    pub busy_percent: u32,
}

impl Memory<u64> {
    pub(crate) unsafe fn get_memory(dv_ind: u32) -> Result<Memory<u64>, RocmErr> {
        let total_vram = mem_total_vram(dv_ind).check()?;
        let total_vis_vram = mem_total_vis_vram(dv_ind).check()?;
        let total_gtt = mem_total_gtt(dv_ind).check()?;
        let used_vram = mem_used_vram(dv_ind).check()?;
        let used_vis_vram = mem_used_vis_vram(dv_ind).check()?;
        let used_gtt = mem_used_gtt(dv_ind).check()?;
        let busy_percent = memory_busy_percent(dv_ind).check()?;

        Ok(Memory {
            total: MemoryTotal {
                vram: total_vram.data,
                vis_vram: total_vis_vram.data,
                gtt: total_gtt.data,
            },
            used: MemoryUsed {
                vram: used_vram.data,
                vis_vram: used_vis_vram.data,
                gtt: used_gtt.data,
            },
            busy_percent: busy_percent.data,
        })
    }

    pub fn into_f64_gb(&self) -> Memory<f64> {
        Memory {
            total: MemoryTotal {
                vram: self.total.vram as f64 / 1000000.,
                vis_vram: self.total.vis_vram as f64 / 1000000.,
                gtt: self.total.gtt as f64 / 1000000.,
            },
            used: MemoryUsed {
                vram: self.used.vram as f64 / 1000000.,
                vis_vram: self.used.vis_vram as f64 / 1000000.,
                gtt: self.used.gtt as f64 / 1000000.,
            },
            busy_percent: self.busy_percent,
        }
    }
}
