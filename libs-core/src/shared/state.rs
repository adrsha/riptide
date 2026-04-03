use crate::shared::{buffers::RTBufferId, clusters::RTClusterId, frames::RTFrameId};

pub struct RTState {
    pub active_frame_id:   RTFrameId,
    pub active_cluster_id: RTClusterId,
    pub active_buffer_id:  RTBufferId
}

impl RTState {
    pub fn new() -> Self {
        Self {
            active_frame_id:   RTFrameId::default(),
            active_cluster_id: RTClusterId::default(),
            active_buffer_id:  RTBufferId::default()
        }
    }
}

impl Default for RTState {
    fn default() -> Self { Self::new() }
}
