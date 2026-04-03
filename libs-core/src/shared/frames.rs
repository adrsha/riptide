use std::fmt::Display;

use crate::shared::buffers::RTBufferId;

pub type RTFrameId = slotmap::DefaultKey;

pub struct RTFrame {
    buffer_id: RTBufferId
}
