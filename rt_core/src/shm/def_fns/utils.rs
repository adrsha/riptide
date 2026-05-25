use std::{
    ffi::CString,
    fs::File,
    os::fd::FromRawFd,
    sync::atomic::{AtomicU64, Ordering}
};

use libc::{O_CREAT, O_RDONLY, O_RDWR, O_TRUNC, ftruncate, shm_open};
use memmap2::{Mmap, MmapMut};

use crate::{
    errors::{RTErrors, RTResult},
    shm::layout::{MAX_FRAMES, RTShmMain}
};

pub fn prepare_shm_create(key: u64, shm_len: i64) -> RTResult<(MmapMut, File)> {
    let cname = CString::new(format!("/riptide_shm_{}", key)).unwrap();
    let fd = unsafe { shm_open(cname.as_ptr(), O_RDWR | O_CREAT | O_TRUNC, 0o600) };

    if fd < 0 {
        return Err(RTErrors::InvalidValue {
            field:  String::from("fd"),
            reason: format!("{} < 0", fd)
        });
    }

    // set the size of shm
    let res = unsafe { ftruncate(fd, shm_len) };

    if res < 0 {
        return Err(RTErrors::InvalidValue {
            field:  String::from("ftruncate result"),
            reason: format!("{} < 0", res)
        });
    }

    let file = unsafe { File::from_raw_fd(fd) };

    // map it into this process's virt memory
    let mut map = unsafe { MmapMut::map_mut(&file).unwrap() };
    preload_page(&mut map);

    Ok((map, file))
}

pub fn preload_page(map: &mut MmapMut) {
    // touch every page to deal with page faults preemptively
    let ptr = map.as_mut_ptr();
    let len = map.len();
    let page_size = 4096usize;
    let mut offset = 0usize;
    while offset < len {
        unsafe {
            ptr.add(offset).write_volatile(0);
        }
        offset += page_size;
    }
}

pub fn prepare_shm_read(key: u64) -> RTResult<(Mmap, File)> {
    let cname = CString::new(format!("/riptide_shm_{}", key)).unwrap();
    let fd = unsafe { shm_open(cname.as_ptr(), O_RDONLY, 0o600) };

    if fd < 0 {
        return Err(RTErrors::InvalidValue {
            field:  String::from("fd"),
            reason: format!("{} < 0 for key: {}", fd, key)
        });
    }

    let file = unsafe { File::from_raw_fd(fd) };

    // map it into this process's virt memory
    let map = unsafe { Mmap::map(&file).expect("Cannot map to memory") };
    Ok((map, file))
}

pub fn claim_frame_slot(shm: &mut RTShmMain, key: u64) -> Option<usize> {
    if key == 0 {
        return None;
    }
    for i in 0 .. MAX_FRAMES {
        let slot = unsafe { &*(&shm.frame_keys[i] as *const u64 as *const AtomicU64) };
        if slot
            .compare_exchange(0, key, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
        {
            return Some(i);
        }
    }
    None
}

pub fn release_frame_slot(shm: &mut RTShmMain, index: usize) {
    assert!(
        index < MAX_FRAMES,
        "release_frame_slot: index out of bounds"
    );
    let slot = unsafe { &*(&shm.frame_keys[index] as *const u64 as *const AtomicU64) };
    slot.store(0, Ordering::Release);
}
