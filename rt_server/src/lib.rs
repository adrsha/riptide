pub mod def_fns;
pub mod listener;
pub mod session;

use std::fs;

use rt_core::{shared::RTShared, types::fn_alias::RTAsyncArcFn};

use crate::{def_fns::run_impl, listener::RTListener};

pub struct RTServer {
    pub shared:   RTShared,
    pub listener: RTListener,
    pub run:      RTAsyncArcFn<RTServer, ()>
}

impl Default for RTServer {
    fn default() -> Self {
        let shared = RTShared::new();
        Self {
            shared,
            listener: RTListener::new(),
            run: run_impl
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
