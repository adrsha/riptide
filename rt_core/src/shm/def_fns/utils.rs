use std::{ffi::CString, fs::File, os::fd::FromRawFd};

use libc::{O_CREAT, O_RDWR, O_TRUNC, ftruncate, shm_open};
use memmap2::MmapMut;

use crate::{
    errors::{RTErrors, RTResult},
    shm::{RTShm, layout::{RTPixelShmHeader, RTShmHeader}, shapes::RTPixelBuffer, styles::RTPixelFormat}
};

pub fn get_header_ptr<'a>(shm: &'a RTShm) -> *mut RTShmHeader {
    shm.map.as_ptr() as *mut RTShmHeader
}

pub fn prepare_shm(cname: CString, shm_len: i64) -> RTResult<(MmapMut, File)> {
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
    Ok((map, file))
}

pub fn alloc_pixel_buffer(
    key: u32,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    format: RTPixelFormat
) -> RTResult<(RTPixelBuffer, File)> {
    let name = format!("/riptide_pb_{}", key);
    let cname = CString::new(name.as_str()).unwrap();

    let pixel_data_len = w as i64 * h as i64 * 4; // worst case RGBA8
    let total_len = std::mem::size_of::<RTPixelShmHeader>() as i64 + pixel_data_len;

    match prepare_shm(cname, total_len) {
        Ok((mut map, file)) => {
            let pb_header = unsafe { &mut *(map.as_mut_ptr() as *mut RTPixelShmHeader) };
            pb_header.disconnect = 0;
            pb_header.format = format as u8;
            pb_header._pad = [0; 2];
            pb_header.dirty = 0;
            pb_header.width = w;
            pb_header.height = h;
            pb_header.stride = w * 4;
            pb_header._pad2 = [0; 4];

            drop(map);

            let mut shm_name = [0u8; 32];
            let name_bytes = name.as_bytes();
            assert!(
                name_bytes.len() < 32,
                "alloc_pixel_buffer: shm name too long"
            );
            shm_name[.. name_bytes.len()].copy_from_slice(name_bytes);

            let pb = RTPixelBuffer {
                shm_name,
                x,
                y,
                w: w as i32,
                h: h as i32,
                _pad: [0; 16]
            };

            Ok((pb, file))
        },
        Err(err) => Err(err)
    }
}
