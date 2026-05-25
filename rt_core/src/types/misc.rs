#[derive(Debug, Clone, Copy)]
pub struct RtColor(pub u32);

impl RtColor {
    #[inline]
    pub fn r(self) -> u8 { ((self.0 >> 24) & 0xFF) as u8 }

    #[inline]
    pub fn g(self) -> u8 { ((self.0 >> 16) & 0xFF) as u8 }

    #[inline]
    pub fn b(self) -> u8 { ((self.0 >> 8) & 0xFF) as u8 }

    #[inline]
    pub fn a(self) -> u8 { (self.0 & 0xFF) as u8 }

    #[inline]
    pub fn to_f32(self) -> (f32, f32, f32, f32) {
        let inv = 1.0 / 255.0;
        (
            self.r() as f32 * inv,
            self.g() as f32 * inv,
            self.b() as f32 * inv,
            self.a() as f32 * inv
        )
    }
}

pub enum RTDirection {
    Horizontal,
    Vertical
}

pub struct RTPosition {
    pub x: usize,
    pub y: usize
}

impl RTPosition {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y
        }
    }
}

pub struct RTSize {
    pub width:  usize,
    pub height: usize
}

impl RTSize {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height
        }
    }
}
