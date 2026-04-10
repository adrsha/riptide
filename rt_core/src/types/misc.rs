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
