pub mod def_fns;
pub mod traits;
use rt_core::{errors::RTResult};

use crate::def_fns::run_impl;

pub struct RTClient {
    pub run: fn(&mut RTClient) -> RTResult<()>
}

impl Default for RTClient {
    fn default() -> Self {
        Self {
            run: run_impl
        }
    }
}
