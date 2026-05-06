use crate::shm::{shapes::RTShape, styles::RTStyle};

pub const MAX_FRAMES: usize = 32;
pub const MAX_STYLES: usize = 256;
pub const MAX_SHAPES: usize = 16384;

#[repr(C)]
pub struct RTPixelShmHeader {
    pub disconnect: u8,
    pub format:     u8,
    pub _pad:       [u8; 2],

    pub dirty:  u32, // monotonic counter. client tracks its own last seen
    pub width:  u32,
    pub height: u32,
    pub stride: u32, // bytes per row
    pub _pad2:  [u8; 4]
}

const _: () = assert!(std::mem::size_of::<RTPixelShmHeader>() == 24);

#[repr(C)]
pub struct RTShmHeader {
    pub visible_frame_count: u64,
    pub style_count:         u16,
    pub _pad:                [u8; 6],
    // alignment of 16, and not 12(4) cause of 8 byte frame count
    pub styles:              [RTStyle; MAX_STYLES]
}

#[repr(C)]
pub struct RTShmFrame {
    pub seq:          u64,
    pub shape_count:  u32,
    pub bezier_count: u16,
    pub pixel_count:  u8,
    pub _pad:         [u8; 49], // pads header to exactly 64 bytes

    pub shapes: [RTShape; MAX_SHAPES]
}

const _: () = assert!(std::mem::offset_of!(RTShmFrame, shapes) == 64);
