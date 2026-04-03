pub mod buffers;
pub mod clusters;
pub mod frames;
pub mod state;

use arc_swap::ArcSwap;
use slotmap::SlotMap;

use crate::shared::{
    buffers::{RTBuffer, RTBufferId},
    clusters::{RTCluster, RTClusterId},
    frames::{RTFrame, RTFrameId},
    state::RTState
};

pub struct RTShared {
    pub clusters: SlotMap<RTClusterId, RTCluster>,
    pub frames:   SlotMap<RTFrameId, RTFrame>,
    pub buffers:  SlotMap<RTBufferId, RTBuffer>,
    pub state:    ArcSwap<RTState>
}

impl RTShared {
    pub fn new() -> Self {
        Self {
            clusters: SlotMap::new(),
            frames:   SlotMap::new(),
            buffers:  SlotMap::new(),
            state:    ArcSwap::from_pointee(RTState::new())
        }
    }
}

impl Default for RTShared {
    fn default() -> Self { Self::new() }
}
