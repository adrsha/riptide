use crate::{
    shared::frames::RTFrameId,
    types::misc::{RTDirection, RTPosition, RTSize}
};

pub type RTClusterId = slotmap::DefaultKey;

pub struct RTCluster {
    root:       RTClusterNode,
    is_visible: bool
}

pub enum RTClusterNode {
    Leaf {
        frame_id: RTFrameId
    },
    Branch {
        direction: RTDirection,
        children:  Vec<RTClusterNode>,
        ratio:     f32
    },
    Float {
        position: RTPosition,
        size:     RTSize
    }
}
