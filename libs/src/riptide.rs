use std::{fs, pin::Pin};

use libs_client::RTClient;
use libs_core::{errors::RTResult, shared::RTShared, types::fn_alias::RTAsyncMutRefFn};
use libs_server::RTServer;

pub struct Riptide {
    pub client: RTClient,
    pub server: RTServer,
    pub shared: RTShared,
    pub run:    RTAsyncMutRefFn<Riptide, ()>
}

impl Riptide {
    pub fn new() -> Self {
        Self {
            client: RTClient::default(),
            server: RTServer::default(),
            shared: RTShared::default(),
            run:    run_impl
        }
    }
}

impl Default for Riptide {
    fn default() -> Self { Self::new() }
}

pub fn run_impl<'a>(
    riptide: &'a mut Riptide
) -> Pin<Box<dyn Future<Output = RTResult<()>> + Send + 'a>> {
    Box::pin(async move {
        println!("Running riptide");
        (riptide.server.run)(&mut riptide.server).await
    })
}

impl Drop for Riptide {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.server.listener.lock_path);
        println!("Dropped lock file for current server");
    }
}
