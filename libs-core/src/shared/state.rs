use crate::shared::{buffers::RTBufferId, clusters::RTClusterId, frames::RTFrameId};

pub struct RTState {
    pub active_frame_id:   Option<RTFrameId>,
    pub active_cluster_id: Option<RTClusterId>,
    pub active_buffer_id:  Option<RTBufferId>
}

impl RTState {
    pub fn new() -> Self {
        Self {
            active_frame_id:   None,
            active_cluster_id: None,
            active_buffer_id:  None
        }
    }
}

impl Default for RTState {
    fn default() -> Self { Self::new() }
}
