use rocm_smi_lib_sys:: RawRsmi;

pub use rocm_smi_lib_sys::bindings::{RsmiErrorCount, RsmiGpuBlock, RsmiRasErrState};

#[derive(Debug)]
pub struct Block {
    pub entry: RsmiGpuBlock,
    pub counters: RsmiErrorCount,
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
    pub(crate) unsafe fn new(raw: &mut RawRsmi, dv_ind: u32) -> Self {
        let iter = [
            RsmiGpuBlock::RsmiGpuBlockUmc,
            RsmiGpuBlock::RsmiGpuBlockSdma,
            RsmiGpuBlock::RsmiGpuBlockGfx,
            RsmiGpuBlock::RsmiGpuBlockMmhub,
            RsmiGpuBlock::RsmiGpuBlockAthub,
            RsmiGpuBlock::RsmiGpuBlockPcieBif,
            RsmiGpuBlock::RsmiGpuBlockHdp,
            RsmiGpuBlock::RsmiGpuBlockXgmiWafl,
            RsmiGpuBlock::RsmiGpuBlockDf,
            RsmiGpuBlock::RsmiGpuBlockSmn,
            RsmiGpuBlock::RsmiGpuBlockSem,
            RsmiGpuBlock::RsmiGpuBlockMp0,
            RsmiGpuBlock::RsmiGpuBlockMp1,
            RsmiGpuBlock::RsmiGpuBlockFuse,
        ]
        .iter();

        let mut blocks = vec![];

        for entry in iter {
            let mut ec = RsmiErrorCount {
                correctable_err: 0,
                uncorrectable_err: 0,
            };

            let mut state = RsmiRasErrState::RsmiRasErrStateDisabled;

            let ret = raw.rsmi_dev_ecc_count_get(dv_ind, *entry, &mut ec as *mut RsmiErrorCount);
            raw.rsmi_dev_ecc_status_get(dv_ind, *entry, &mut state as *mut RsmiRasErrState);

            let block = if ret.try_err().is_ok() {
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
