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
