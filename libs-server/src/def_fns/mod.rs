use std::pin::Pin;

use libs_core::errors::RTResult;

use crate::RTServer;

pub fn run_impl<'a>(
    server: &'a mut RTServer
) -> Pin<Box<dyn Future<Output = RTResult<()>> + Send + 'a>> {
    Box::pin(async move {
        (server.listener.tcp_listen)(server).await
    })
}
