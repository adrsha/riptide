use std::{pin::Pin, sync::Arc};

use rt_core::{errors::RTResult, shm::def_fns::utils::claim_frame_slot};

use crate::{RTServer, shared::processes::RTProcess};

pub fn run_impl(server: Arc<RTServer>) -> Pin<Box<dyn Future<Output = RTResult<()>> + Send>> {
    Box::pin(async move {
        // TODO:  Events watcher
        let server_cl = server.clone();
        {
            let mut shm = server.shm.lock();
            claim_frame_slot((shm.get_mut)(&mut shm), 2);
        }

        {
            let mut processes = server.shared.processes.lock();
            let new_process = RTProcess::new(2);
            processes.insert(new_process);
        }

        tokio::task::spawn(async move {
            let server_cl = server_cl.clone();
            (server_cl.listener.tcp_listen)(server_cl.clone()).await;
        })
        .await;

        Ok(())
    })
}
