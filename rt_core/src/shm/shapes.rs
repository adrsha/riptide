#[repr(C)]
#[derive(Clone, Copy)]
pub struct RTShape {
    pub style_id: u16,
    pub tag:      u8,
    pub _pad:     u8,
    pub data:     RTShapeData
}

#[repr(u8)]
pub enum RTShapeTag {
    Triangle = 0,
    Rect     = 1,
    Circle   = 2
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union RTShapeData {
    pub triangle: RTTriangle,
    pub rect:     RTRect,
    pub circle:   RTCircle,
    pub raw:      [u8; 28]
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RTTriangle {
    pub points: [i32; 6], // [x0,y0, x1,y1, x2,y2]
    pub _pad:   [u8; 4]
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RTRect {
    pub x:    i32,
    pub y:    i32,
    pub w:    i32,
    pub h:    i32,
    pub _pad: [u8; 12]
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RTCircle {
    pub cx:   i32,
    pub cy:   i32,
    pub r:    i32,
    pub _pad: [u8; 16]
}

const _: () = assert!(std::mem::size_of::<RTShape>() == 32);
