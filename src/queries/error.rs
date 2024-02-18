use rocm_smi_lib_sys::bindings::{rsmi_dev_ecc_count_get, rsmi_dev_ecc_status_get};

pub use rocm_smi_lib_sys::bindings::{RsmiErrorCountT, RsmiGpuBlockT, RsmiRasErrStateT};

#[derive(Debug)]
pub struct Block {
    pub entry: RsmiGpuBlockT,
    pub counters: RsmiErrorCountT,
}

#[derive(Debug)]
pub struct State {
    pub state: RsmiRasErrStateT,
    pub block: Option<Block>,
}

#[derive(Debug)]
pub struct EccData {
    pub blocks: Vec<State>,
}

impl EccData {
    #[inline(always)]
    pub(crate) unsafe fn new(dv_ind: u32) -> Self {
        let iter = [
            RsmiGpuBlockT::RsmiGpuBlockUmc,
            RsmiGpuBlockT::RsmiGpuBlockSdma,
            RsmiGpuBlockT::RsmiGpuBlockGfx,
            RsmiGpuBlockT::RsmiGpuBlockMmhub,
            RsmiGpuBlockT::RsmiGpuBlockAthub,
            RsmiGpuBlockT::RsmiGpuBlockPcieBif,
            RsmiGpuBlockT::RsmiGpuBlockHdp,
            RsmiGpuBlockT::RsmiGpuBlockXgmiWafl,
            RsmiGpuBlockT::RsmiGpuBlockDf,
            RsmiGpuBlockT::RsmiGpuBlockSmn,
            RsmiGpuBlockT::RsmiGpuBlockSem,
            RsmiGpuBlockT::RsmiGpuBlockMp0,
            RsmiGpuBlockT::RsmiGpuBlockMp1,
            RsmiGpuBlockT::RsmiGpuBlockFuse,
        ]
        .iter();

        let mut blocks = vec![];

        for entry in iter {
            let mut ec = RsmiErrorCountT {
                correctable_err: 0,
                uncorrectable_err: 0,
            };

            let mut state = RsmiRasErrStateT::RsmiRasErrStateDisabled;

            let ret = rsmi_dev_ecc_count_get(dv_ind, *entry, &mut ec as *mut RsmiErrorCountT);
            rsmi_dev_ecc_status_get(dv_ind, *entry, &mut state as *mut RsmiRasErrStateT);

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
