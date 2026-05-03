use crate::shared::{clusters::RTClusterId, frames::RTFrameId, processes::RTProcessId};

pub struct RTState {
    pub active_frame_id:   Option<RTFrameId>,
    pub active_cluster_id: Option<RTClusterId>,
    pub active_process_id: Option<RTProcessId>
}

impl RTState {
    pub fn new() -> Self {
        Self {
            active_frame_id:   None,
            active_cluster_id: None,
            active_process_id: None
        }
    }
}

impl Default for RTState {
    fn default() -> Self { Self::new() }
}
