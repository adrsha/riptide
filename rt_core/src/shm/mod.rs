pub mod def_fns;
pub mod layout;
pub mod seqlock;
pub mod shapes;
pub mod styles;

use std::ffi::CString;

use memmap2::MmapMut;

use crate::{
    errors::RTResult,
    shm::{
        def_fns::{
            impls::{as_bytes, get_frame, get_header, get_mut_frame, get_mut_header},
            utils::prepare_shm
        },
        layout::{MAX_FRAMES, RTShmFrame, RTShmHeader}
    }
};

pub struct RTShm {
    _file:              std::fs::File,
    pub map:            MmapMut,
    pub as_bytes:       for<'a> fn(&'a mut Self) -> &'a mut [u8],
    pub get_header:     for<'a> fn(&'a Self) -> &'a RTShmHeader,
    pub get_mut_header: for<'a> fn(&'a mut Self) -> &'a mut RTShmHeader,
    pub get_frame:      for<'a> fn(&'a Self, usize) -> &'a RTShmFrame,
    pub get_mut_frame:  for<'a> fn(&'a mut Self, usize) -> &'a mut RTShmFrame
}

const SHM_LEN: i64 =
    (std::mem::size_of::<RTShmHeader>() + std::mem::size_of::<RTShmFrame>() * MAX_FRAMES) as i64;

impl RTShm {
    pub fn create() -> RTResult<Self> {
        let cname = CString::new(format!("/riptide_shm")).unwrap();
        match prepare_shm(cname, SHM_LEN) {
            Ok((map, file)) => {
                Ok(Self {
                    _file: file,
                    map,
                    as_bytes,
                    get_header,
                    get_mut_header,
                    get_frame,
                    get_mut_frame
                })
            },
            Err(err) => Err(err)
        }
    }
}
