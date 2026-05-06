// 12 bytes, no implicit padding, 256 of these = 3KB (fits in L1)
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RTStyle {
    pub fill:         u32, // RGBA
    pub stroke:       u32, // RGBA
    pub stroke_width: u16,
    pub opacity:      u8,
    // aligning the struct to 12 bytes => 4
    pub _pad:         u8
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default)]
pub enum RTPixelFormat {
    #[default]
    RGBA8  = 0,
    BGRA8  = 1,
    YUV420 = 2
}
