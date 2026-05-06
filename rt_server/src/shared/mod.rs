pub mod clusters;
pub mod frames;
pub mod processes;
pub mod state;
pub mod windows;

use parking_lot::Mutex;
use slotmap::SlotMap;

use crate::shared::{
    clusters::{RTCluster, RTClusterId},
    frames::{RTFrame, RTFrameId},
    processes::{RTProcess, RTProcessId},
    state::RTState,
    windows::{RTWindow, RTWindowId}
};

pub struct RTShared {
    pub windows:   Mutex<SlotMap<RTWindowId, RTWindow>>,
    pub clusters:  Mutex<SlotMap<RTClusterId, RTCluster>>,
    pub frames:    Mutex<SlotMap<RTFrameId, RTFrame>>,
    pub processes: Mutex<SlotMap<RTProcessId, RTProcess>>,
    pub state:     Mutex<RTState>
}

impl RTShared {
    pub fn new() -> Self {
        Self {
            windows:   Mutex::new(SlotMap::new()),
            clusters:  Mutex::new(SlotMap::new()),
            frames:    Mutex::new(SlotMap::new()),
            processes: Mutex::new(SlotMap::new()),
            state:     Mutex::new(RTState::new())
        }
    }
}

impl Default for RTShared {
    fn default() -> Self { Self::new() }
}
