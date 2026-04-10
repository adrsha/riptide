pub mod buffers;
pub mod clusters;
pub mod frames;
pub mod state;

use parking_lot::Mutex;
use slotmap::SlotMap;

use crate::shared::{
    buffers::{RTBuffer, RTBufferId},
    clusters::{RTCluster, RTClusterId},
    frames::{RTFrame, RTFrameId},
    state::RTState
};

pub struct RTShared {
    pub clusters: Mutex<SlotMap<RTClusterId, RTCluster>>,
    pub frames:   Mutex<SlotMap<RTFrameId, RTFrame>>,
    pub buffers:  Mutex<SlotMap<RTBufferId, RTBuffer>>,
    pub state:    Mutex<RTState>
}

impl RTShared {
    pub fn new() -> Self {
        Self {
            clusters: Mutex::new(SlotMap::new()),
            frames:   Mutex::new(SlotMap::new()),
            buffers:  Mutex::new(SlotMap::new()),
            state:    Mutex::new(RTState::new())
        }
    }
}

impl Default for RTShared {
    fn default() -> Self { Self::new() }
}
