use crate::errors::{RTErrors, RTResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RTModifiers(pub u8);

impl RTModifiers {
    pub const ALT: u8 = 0b0100;
    pub const CTRL: u8 = 0b0010;
    const KNOWN_BITS: u8 = Self::SHIFT | Self::CTRL | Self::ALT | Self::META;
    pub const META: u8 = 0b1000;
    pub const SHIFT: u8 = 0b0001;

    pub fn to_bits(self) -> u8 { self.0 }

    pub fn has_unknown_bits(self) -> u8 { self.0 & !Self::KNOWN_BITS }

    pub fn is_shift(self) -> bool { self.0 & Self::SHIFT != 0 }

    pub fn is_ctrl(self) -> bool { self.0 & Self::CTRL != 0 }

    pub fn is_alt(self) -> bool { self.0 & Self::ALT != 0 }

    pub fn is_meta(self) -> bool { self.0 & Self::META != 0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RTMouseButton {
    Left   = 0,
    Right  = 1,
    Middle = 2
}

impl TryFrom<u8> for RTMouseButton {
    type Error = RTErrors;

    fn try_from(v: u8) -> RTResult<Self> {
        match v {
            0 => Ok(Self::Left),
            1 => Ok(Self::Right),
            2 => Ok(Self::Middle),
            _ => {
                Err(RTErrors::InvalidValue {
                    field:  String::from("RTMouseButton"),
                    reason: format!("invalid discriminant: {v}")
                })
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum RTInputEvent {
    Key {
        keycode:   u32,
        modifiers: RTModifiers,
        pressed:   bool
    },
    MouseMove {
        x: f32,
        y: f32
    },
    Click {
        x:      f32,
        y:      f32,
        button: RTMouseButton
    },
    Scroll {
        x:       f32,
        y:       f32,
        delta_x: f32,
        delta_y: f32
    },
    Custom {
        name:    String,
        payload: Vec<u8>
    }
}

#[repr(u8)]
pub enum RTTag {
    Key       = 0,
    MouseMove = 1,
    Click     = 2,
    Scroll    = 3,
    Custom    = 4
}

impl TryFrom<u8> for RTTag {
    type Error = RTErrors;

    fn try_from(v: u8) -> RTResult<Self> {
        match v {
            0 => Ok(Self::Key),
            1 => Ok(Self::MouseMove),
            2 => Ok(Self::Click),
            3 => Ok(Self::Scroll),
            4 => Ok(Self::Custom),
            _ => {
                Err(RTErrors::InvalidValue {
                    field:  String::from("RTTag"),
                    reason: format!("invalid discriminant: {v}")
                })
            },
        }
    }
}
