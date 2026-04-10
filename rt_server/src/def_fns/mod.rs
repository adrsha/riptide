use std::{pin::Pin, sync::Arc};

use rt_core::errors::RTResult;

use crate::RTServer;

pub fn run_impl(server: Arc<RTServer>) -> Pin<Box<dyn Future<Output = RTResult<()>> + Send>> {
    Box::pin(async move {
        (server.listener.tcp_listen)(server).await
    })
}
