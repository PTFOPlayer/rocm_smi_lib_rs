
use rocm_smi_lib_sys::bindings::{rsmi_dev_ecc_count_get, rsmi_dev_ecc_status_get, rsmi_error_count_t};

use crate::error::IntoRocmErr;

use super::common_structures::{RsmiGpuBlock, RsmiRasErrState};

#[derive(Debug)]
pub struct Block {
    pub entry: RsmiGpuBlock,
    pub counters: rsmi_error_count_t,
}

#[derive(Debug)]
pub struct State {
    pub state: RsmiRasErrState,
    pub block: Option<Block>,
}

#[derive(Debug)]
pub struct EccData {
    pub blocks: Vec<State>,
}

impl EccData {
    #[inline(always)]
    pub(crate) unsafe fn new( dv_ind: u32) -> Self {
        
        let iter = [
            RsmiGpuBlock::Umc,
            RsmiGpuBlock::Sdma,
            RsmiGpuBlock::Gfx,
            RsmiGpuBlock::Mmhub,
            RsmiGpuBlock::Athub,
            RsmiGpuBlock::PcieBif,
            RsmiGpuBlock::Hdp,
            RsmiGpuBlock::XgmiWafl,
            RsmiGpuBlock::Df,
            RsmiGpuBlock::Smn,
            RsmiGpuBlock::Sem,
            RsmiGpuBlock::Mp0,
            RsmiGpuBlock::Mp1,
            RsmiGpuBlock::Fuse,
        ]
        .iter();

        let mut blocks = vec![];

        for entry in iter {
            let mut ec = rsmi_error_count_t {
                correctable_err: 0,
                uncorrectable_err: 0,
            };

            let mut state = RsmiRasErrState::Disabled;

            let ret = rsmi_dev_ecc_count_get(dv_ind, *entry as u64, &mut ec as *mut rsmi_error_count_t);
            rsmi_dev_ecc_status_get(dv_ind, *entry as u64, (&mut state as *mut RsmiRasErrState).cast());

            let block = if ret.into_rocm_err().is_ok() {
                Some(Block {
                    entry: *entry,
                    counters: ec,
                })
            } else {
                None
            };

            blocks.push(State { state, block })
        }

        Self { blocks }
    }
}
