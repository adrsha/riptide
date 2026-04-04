use std::{env, fs};

use libs_core::{
    errors::RTResult,
    types::messages::{RTClusterCommand, RTCommand, RTSignal}
};
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() -> RTResult<()> {
    let args: Vec<String> = env::args().collect();
    let mut i = 1;

    let mut server_id: usize = 0;
    let signal_type: RTCommand = RTCommand::Cluster(RTClusterCommand::OpenCluster);

    while i < args.len() {
        match args[i].as_str() {
            "-s" | "--server-id" => {
                server_id = args.get(i + 1).and_then(|s| s.parse().ok()).unwrap_or(0);
                i += 2;
            },
            _ => {
                i += 1;
            }
        }
    }

    let signal = RTSignal {
        server_id,
        signal_type
    };

    if let Some(mut dir) = dirs::data_local_dir() {
        dir.push(format!("riptide_client_{}.txt", signal.server_id));
        if dir.exists() {
            let file_content = fs::read(dir);
            if let Ok(port) = file_content {
                send_open_request(
                    String::from_utf8(port).expect("Cannot convert file data to string"),
                    signal
                )
                .await?;
            }
        }
        else {
            eprint!("No Daemon found.");
        }
    }
    Ok(())
}

async fn send_open_request(port: String, request: RTSignal) -> RTResult<()> {
    println!("Using port: {}", port);
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port)).await?;

    let payload = bitcode::encode(&request);
    stream.write_all(payload.as_slice()).await?;

    Ok(())
}
