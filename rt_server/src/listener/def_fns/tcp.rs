use std::{path::PathBuf, pin::Pin, sync::Arc};

use rt_core::{
    errors::{RTErrors, RTResult},
    shared::clusters::RTCluster,
    types::messages::{RTClusterCommand, RTCommand, RTSignal}
};
use tokio::{
    fs,
    io::{AsyncReadExt, BufReader},
    net::TcpListener
};

use crate::{
    RTServer,
    listener::def_fns::{create_lock_dir, shutdown_signals}
};

pub fn tcp_listen_impl<'a>(
    server: Arc<RTServer>
) -> Pin<Box<dyn Future<Output = RTResult<()>> + Send + 'a>> {
    Box::pin(async move {
        let lock_path: &mut PathBuf = match server.listener.lock_path.get() {
            Some(lock_path) => &mut lock_path.clone(),
            None => &mut create_lock_dir(&server.listener).await?
        };
        lock_path.push("tcp.txt");

        let listener = get_listener().await?;
        let addr = listener.local_addr()?;

        fs::write(lock_path, addr.port().to_string()).await?;

        tokio::select! {
            result = async {
                loop {
                    parse_signal(&listener, server.clone()).await?;
                }
                #[allow(unreachable_code)]
                Ok::<(), RTErrors>(())
            } => { result? },

            _ = tokio::signal::ctrl_c() => {
                println!("Shutdown signal received");
            },

            _ = shutdown_signals() => {
                println!("Shutdown signal received (SIGHUP/SIGTERM)");
            },

        }

        Ok(())
    })
}

pub async fn get_listener() -> RTResult<TcpListener> {
    TcpListener::bind("127.0.0.1:0").await.map_err(|e| {
        RTErrors::Io {
            source: e
        }
    })
}

async fn parse_signal(listener: &TcpListener, server: Arc<RTServer>) -> Result<(), RTErrors> {
    let (socket, peer_addr) = listener.accept().await?;

    println!("Connection from {peer_addr}");

    let mut buf = Vec::new();
    let mut reader = BufReader::new(socket);

    reader.read_to_end(&mut buf).await?;

    let signal = bitcode::decode::<RTSignal>(&buf).map_err(|e| {
        RTErrors::Deserialize {
            context: String::from("Signals"),
            reason:  e.to_string()
        }
    })?;

    match signal.signal_type {
        RTCommand::Cluster(cluster_cmd) => {
            match cluster_cmd {
                RTClusterCommand::OpenCluster => {
                    let mut clusters_guard = server.shared.clusters.lock();
                    let _id = clusters_guard.insert(RTCluster::new());
                },
                RTClusterCommand::CloseCluster => {
                    let active_cluster_id = {
                        let mut state_guard = server.shared.state.lock();
                        let past_active_cluster_id = state_guard.active_cluster_id;
                        state_guard.active_cluster_id = None;
                        state_guard.active_frame_id = None;
                        past_active_cluster_id
                    };

                    if let Some(cluster_id) = active_cluster_id {
                        {
                            let mut clusters_guard = server.shared.clusters.lock();
                            clusters_guard.remove(cluster_id);
                        }
                    }
                    else {
                        eprintln!("No cluster selected");
                    }
                },
                _ => {}
            }
        },
    }

    Ok(())
}
