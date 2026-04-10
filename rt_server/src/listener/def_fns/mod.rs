use std::{fs, path::PathBuf};

use rt_core::errors::{RTErrors, RTResult};

use crate::listener::RTListener;

pub mod tcp;
pub mod unix;

async fn create_lock_dir(listener: &RTListener) -> RTResult<PathBuf> {
    let base_dir = dirs::data_local_dir().ok_or(RTErrors::NotFound {
        key: String::from("data local directory")
    })?;

    let mut server_id = 0usize;
    while base_dir.join(format!("riptide_{server_id}")).is_dir() {
        server_id += 1;
    }

    let lock_path = base_dir.join(format!("riptide_{server_id}"));
    println!("Lock Folder path: {lock_path:?}, server_id = {server_id}");

    fs::create_dir_all(&lock_path)?;

    if let Err(t) = listener.lock_path.set(lock_path.clone()) {
        Err(RTErrors::AlreadyExists {
            key: t.to_string_lossy().to_string()
        })
    }
    else {
        Ok(lock_path)
    }
}

#[cfg(unix)]
async fn shutdown_signals() {
    use tokio::signal::unix::{SignalKind, signal};

    let mut sig_hup = signal(SignalKind::hangup()).expect("failed SIGHUP");
    let mut sig_term = signal(SignalKind::terminate()).expect("failed SIGTERM");

    tokio::select! {
        _ = sig_hup.recv()  => {},
        _ = sig_term.recv() => {},
    }
}

#[cfg(not(unix))]
async fn shutdown_signals() { std::future::pending::<()>().await }
