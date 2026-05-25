use crate::shm::{RTShm, RTShmMut};

fn get_ptr<'a, T>(shm: &'a RTShm<T>) -> *const T { shm.map.as_ptr() as *const T }
pub fn get_impl<'a, T>(shm: &'a RTShm<T>) -> &'a T { unsafe { &*get_ptr::<T>(shm) } }

pub fn as_bytes_impl<'a, T>(shm: &'a mut RTShmMut<T>) -> &'a mut [u8] { &mut shm.map }
fn get_mut_ptr<T>(shm: &mut RTShmMut<T>) -> *mut T { shm.map.as_ptr() as *mut T }
pub fn get_mut_impl<'a, T>(shm: &'a mut RTShmMut<T>) -> &'a mut T {
    unsafe { &mut *get_mut_ptr(shm) }
}
