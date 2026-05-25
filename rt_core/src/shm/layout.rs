use crate::shm::{shapes::RTShape, styles::RTStyle};

pub const MAX_FRAMES: usize = 256;
pub const MAX_STYLES: usize = 256;
pub const MAX_SHAPES: usize = 16384;

#[repr(C)]
pub struct RTShmMain {
    pub frame_keys:  [u64; MAX_FRAMES], // 0 = empty, else SHM name key
    pub style_count: u16,
    pub _pad:        [u8; 2],
    pub styles:      [RTStyle; MAX_STYLES]
}

#[repr(C)]
pub struct RTShmFrame {
    pub seq:          u64,
    pub shape_count:  u32,
    pub disconnect:   u8,
    pub _pad:         [u8; 51], // pads header to exactly 64 bytes

    pub shapes: [RTShape; MAX_SHAPES]
}

const _: () = assert!(std::mem::offset_of!(RTShmFrame, shapes) == 64);

pub struct RTShmFrameView<'a> {
    pub shapes: &'a [RTShape],
    pub shape_count: u32,
}
