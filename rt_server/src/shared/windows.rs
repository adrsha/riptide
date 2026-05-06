use crate::shared::clusters::RTClusterId;

pub type RTWindowId = slotmap::DefaultKey;

#[derive(Default)]
pub struct RTWindow {
    pub cluster_id : RTClusterId,
}
