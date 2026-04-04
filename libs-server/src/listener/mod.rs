pub mod def_fns;

use std::path::PathBuf;

use libs_core::types::fn_alias::RTAsyncMutRefFn;

use crate::{RTServer, listener::def_fns::tcp_listen_impl};

pub struct RTListener {
    pub lock_path:  PathBuf,
    pub tcp_listen: RTAsyncMutRefFn<RTServer, ()>
}

impl RTListener {
    pub fn new() -> Self {
        Self {
            lock_path:  PathBuf::new(),
            tcp_listen: tcp_listen_impl
        }
    }
}

impl Default for RTListener {
    fn default() -> Self { Self::new() }
}
