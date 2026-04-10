use crate::{
    shared::frames::RTFrameId,
    types::misc::{RTDirection, RTPosition, RTSize}
};

pub type RTClusterId = slotmap::DefaultKey;

pub struct RTCluster {
    pub root:       RTClusterNode,
    pub is_visible: bool
}

impl RTCluster {
    pub fn new() -> Self {
        Self {
            root:       RTClusterNode::Branch {
                direction: RTDirection::Horizontal,
                children:  Vec::new(),
                ratio:     0.0
            },
            is_visible: true
        }
    }
}

impl Default for RTCluster {
    fn default() -> Self { Self::new() }
}

pub enum RTClusterNode {
    Leaf {
        frame_id: RTFrameId,
        ratio:    f32
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
