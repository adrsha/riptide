use crate::shared::buffers::RTBufferId;

pub type RTFrameId = slotmap::DefaultKey;

pub struct RTFrame {
    pub buffer_id: RTBufferId
}
