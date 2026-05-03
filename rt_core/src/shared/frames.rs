use crate::shared::processes::RTProcessId;

pub type RTFrameId = slotmap::DefaultKey;

#[derive(Default)]
pub struct RTFrame {
    pub process_id: RTProcessId
}
