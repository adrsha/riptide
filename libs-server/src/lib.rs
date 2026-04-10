pub mod def_fns;
pub mod listener;
pub mod session;

use libs_core::{shared::RTShared, types::fn_alias::RTAsyncArcFn};

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
