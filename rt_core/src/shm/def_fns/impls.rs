use crate::shm::{
    RTShm,
    def_fns::utils::get_header_ptr,
    layout::{RTShmFrame, RTShmHeader}
};

pub fn as_bytes<'a>(shm: &'a mut RTShm) -> &'a mut [u8] { &mut shm.map }

pub fn get_header<'a>(shm: &'a RTShm) -> &'a RTShmHeader { unsafe { &*get_header_ptr(shm) } }

pub fn get_mut_header<'a>(shm: &'a mut RTShm) -> &'a mut RTShmHeader {
    unsafe { &mut *get_header_ptr(shm) }
}

fn get_frame_ptr<'a>(shm: &'a RTShm, index: usize) -> *mut RTShmFrame {
    let offset = size_of::<RTShmHeader>() + index * size_of::<RTShmFrame>();
    let shm_ptr = shm.map.as_ptr() as *const u8;
    (unsafe { shm_ptr.add(offset) }) as *mut RTShmFrame
}

pub fn get_frame<'a>(shm: &'a RTShm, index: usize) -> &'a RTShmFrame {
    unsafe { &*get_frame_ptr(shm, index) }
}

pub fn get_mut_frame<'a>(shm: &'a mut RTShm, index: usize) -> &'a mut RTShmFrame {
    unsafe { &mut *get_frame_ptr(shm, index) }
}
