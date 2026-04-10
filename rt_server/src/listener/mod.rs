pub mod def_fns;

use std::{path::PathBuf, sync::OnceLock};

use rt_core::types::fn_alias::RTAsyncArcFn;

use crate::{
    RTServer,
    listener::def_fns::{tcp::tcp_listen_impl, unix::unix_listen_impl}
};

pub struct RTListener {
    pub lock_path:   OnceLock<PathBuf>,
    pub tcp_listen:  RTAsyncArcFn<RTServer, ()>,
    pub unix_listen: RTAsyncArcFn<RTServer, ()>
}

impl RTListener {
    pub fn new() -> Self {
        Self {
            lock_path:   OnceLock::new(),
            tcp_listen:  tcp_listen_impl,
            unix_listen: unix_listen_impl
        }
    }
}

impl Default for RTListener {
    fn default() -> Self { Self::new() }
}
