use std::{fs, pin::Pin, sync::Arc};

use libs_core::{errors::RTResult, types::fn_alias::{RTAsyncMutRefFn, RTAsyncRefFn}};
use libs_server::RTServer;

pub struct Riptide {
    pub server: Arc<RTServer>,
    pub run:    RTAsyncRefFn<Riptide, ()>
}

impl Riptide {
    pub fn new() -> Self {
        Self {
            server: Arc::new(RTServer::default()),
            run:    run_impl
        }
    }
}

impl Default for Riptide {
    fn default() -> Self { Self::new() }
}

pub fn run_impl<'a>(
    riptide: &'a Riptide
) -> Pin<Box<dyn Future<Output = RTResult<()>> + Send + 'a>> {
    Box::pin(async move {
        println!("Running riptide");
        (riptide.server.run)(riptide.server.clone()).await
    })
}

impl Drop for Riptide {
    fn drop(&mut self) {
        if let Some(lock_path) = self.server.listener.lock_path.get() {
            let _ = fs::remove_file(lock_path);
        }
        println!("Dropped lock file for current server");
    }
}
