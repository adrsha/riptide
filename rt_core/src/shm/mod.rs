pub mod def_fns;
pub mod layout;
pub mod shapes;
pub mod styles;

use memmap2::{Mmap, MmapMut};

use crate::{
    errors::RTResult,
    shm::{
        def_fns::{
            impls::{as_bytes_impl, get_impl, get_mut_impl},
            utils::{prepare_shm_create, prepare_shm_read}
        },
        layout::MAX_FRAMES
    }
};

pub struct RTShmMut<T> {
    _file:        std::fs::File,
    map:      MmapMut,
    pub as_bytes: for<'a> fn(&'a mut Self) -> &'a mut [u8],
    pub get_mut:  for<'a> fn(&'a mut Self) -> &'a mut T
}

impl<T> RTShmMut<T> {
    pub fn create(key: u64) -> RTResult<Self> {
        match prepare_shm_create(key, (std::mem::size_of::<T>() * MAX_FRAMES) as i64) {
            Ok((map, file)) => {
                Ok(Self {
                    _file: file,
                    map,
                    as_bytes: as_bytes_impl,
                    get_mut: get_mut_impl
                })
            },
            Err(err) => Err(err)
        }
    }
}

pub struct RTShm<T> {
    _file:        std::fs::File,
    map:      Mmap,
    pub get:      for<'a> fn(&'a Self) -> &'a T,
}

impl<T> RTShm<T> {
    pub fn read(key: u64) -> RTResult<Self> {
        match prepare_shm_read(key) {
            Ok((map, file)) => {
                Ok(Self {
                    _file: file,
                    map,
                    get: get_impl,
                })
            },
            Err(err) => Err(err)
        }
    }
}

