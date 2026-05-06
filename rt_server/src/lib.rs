pub mod def_fns;
pub mod listener;
pub mod session;
pub mod shared;

use std::fs;

use rt_core::{shm::RTShm, types::fn_alias::RTAsyncArcFn};

use crate::{def_fns::run_impl, listener::RTListener, shared::RTShared};

pub struct RTServer {
    pub shared:   RTShared,
    pub shm:      RTShm,
    pub listener: RTListener,
    pub run:      RTAsyncArcFn<RTServer, ()>
}

impl Default for RTServer {
    fn default() -> Self {
        let shared = RTShared::new();
        match RTShm::create() {
            Ok(shm_new) => {
                Self {
                    shared,
                    shm: shm_new,
                    listener: RTListener::new(),
                    run: run_impl
                }
            },
            Err(err) => panic!("{}", err)
        }
    }
}

impl Drop for RTServer {
    fn drop(&mut self) {
        if let Some(lock_path) = self.listener.lock_path.get() {
            let _ = fs::remove_dir_all(lock_path);
        }
        println!("Dropped lock file for current server");
    }
}
