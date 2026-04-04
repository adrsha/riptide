use std::{net::SocketAddr, pin::Pin, sync::Arc};

use libs_core::{
    errors::{RTErrors, RTResult},
    shared::{clusters::RTCluster, state::RTState},
    types::messages::{RTClusterCommand, RTCommand, RTSignal}
};
use tokio::{
    fs,
    io::{AsyncReadExt, BufReader},
    net::TcpListener,
};

use crate::{RTServer, listener::RTListener};

pub fn tcp_listen_impl<'a>(
    server: &'a mut RTServer
) -> Pin<Box<dyn Future<Output = RTResult<()>> + Send + 'a>> {
    Box::pin(async move {
        let listener = get_listener().await?;
        let addr = listener.local_addr()?;

        create_lock_file(addr, &mut server.listener).await?;

        tokio::select! {
            result = async {
                loop {
                    parse_signal(&listener, server).await?;
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

pub async fn create_lock_file(addr: SocketAddr, listener: &mut RTListener) -> RTResult<()> {
    let base_dir = dirs::data_local_dir().ok_or_else(|| {
        RTErrors::NotFound {
            key: String::from("data local directory")
        }
    })?;

    let mut server_id = 0usize;
    while base_dir
        .join(format!("riptide_client_{server_id}.txt"))
        .is_file()
    {
        server_id += 1;
    }

    let lock_path = base_dir.join(format!("riptide_client_{server_id}.txt"));
    println!("Lock file path: {lock_path:?}, server_id = {server_id}");

    fs::write(&lock_path, addr.port().to_string()).await?;
    listener.lock_path = lock_path;

    Ok(())
}

async fn parse_signal(listener: &TcpListener, server: &mut RTServer) -> Result<(), RTErrors> {
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
                    let _id = server.shared.clusters.insert(RTCluster::new());
                },
                RTClusterCommand::CloseCluster => {
                    let old_state = server.shared.state.load();
                    if let Some(cluster_id) = old_state.active_cluster_id {
                        server.shared.clusters.remove(cluster_id);
                        let new_state = RTState {
                            active_frame_id:   old_state.active_frame_id,
                            active_cluster_id: None,
                            active_buffer_id:  old_state.active_buffer_id
                        };
                        server.shared.state.swap(Arc::new(new_state));
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

#[cfg(unix)]
async fn shutdown_signals() {
    use tokio::signal::unix::{SignalKind, signal};

    let mut sig_hup = signal(SignalKind::hangup()).expect("failed to register SIGHUP");
    let mut sig_term = signal(SignalKind::terminate()).expect("failed to register SIGTERM");

    tokio::select! {
        _ = sig_hup.recv()  => {},
        _ = sig_term.recv() => {},
    }
}

#[cfg(not(unix))]
async fn shutdown_signals() { std::future::pending::<()>().await }
