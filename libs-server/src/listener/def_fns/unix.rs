use std::{path::PathBuf, pin::Pin, sync::Arc};

use libs_core::{
    errors::{RTErrors, RTResult},
    types::input::{RTInputEvent, RTModifiers, RTMouseButton, RTTag}
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{UnixListener, UnixStream, unix::OwnedWriteHalf}
};

use crate::{
    RTServer,
    listener::def_fns::{create_lock_dir, shutdown_signals}
};

pub fn unix_listen_impl(
    server: Arc<RTServer>
) -> Pin<Box<dyn Future<Output = RTResult<()>> + Send>> {
    Box::pin(async move {
        let mut created;
        let socket_path: &mut PathBuf = match server.listener.lock_path.get() {
            Some(existing) => {
                created = existing.clone();
                &mut created
            },
            None => {
                created = create_lock_dir(&server.listener).await?;
                &mut created
            }
        };
        socket_path.push("unix.sock");

        let listener = UnixListener::bind(&socket_path).map_err(|e| {
            RTErrors::Io {
                source: e
            }
        })?;

        tokio::select! {
            result = async {
                loop {
                    let (mut stream, _) = listener
                        .accept()
                        .await
                        .map_err(|e| RTErrors::Io { source: e })?;

                    tokio::spawn(async move {
                        if let Err(e) = read_message(&mut stream).await {
                            eprintln!("read_message error: {e}");
                        }
                    });
                }
                #[allow(unreachable_code)]
                Ok::<(), RTErrors>(())
            } => result,

            _ = tokio::signal::ctrl_c() => {
                println!("Shutdown signal received (SIGINT)");
                Ok(())
            },

            _ = shutdown_signals() => {
                println!("Shutdown signal received (SIGHUP/SIGTERM)");
                Ok(())
            },
        }
    })
}

pub async fn read_message(stream: &mut UnixStream) -> RTResult<RTInputEvent> {
    let tag = RTTag::try_from(stream.read_u8().await?)?;

    match tag {
        RTTag::Key => {
            Ok(RTInputEvent::Key {
                keycode:   stream.read_u32_le().await?,
                modifiers: RTModifiers(stream.read_u8().await?),
                pressed:   stream.read_u8().await? != 0
            })
        },

        RTTag::MouseMove => {
            Ok(RTInputEvent::MouseMove {
                x: stream.read_f32_le().await?,
                y: stream.read_f32_le().await?
            })
        },

        RTTag::Click => {
            Ok(RTInputEvent::Click {
                x:      stream.read_f32_le().await?,
                y:      stream.read_f32_le().await?,
                button: RTMouseButton::try_from(stream.read_u8().await?)?
            })
        },

        RTTag::Scroll => {
            Ok(RTInputEvent::Scroll {
                x:       stream.read_f32_le().await?,
                y:       stream.read_f32_le().await?,
                delta_x: stream.read_f32_le().await?,
                delta_y: stream.read_f32_le().await?
            })
        },

        RTTag::Custom => {
            let name_len = stream.read_u8().await? as usize;
            let mut name_buf = vec![0u8; name_len];
            stream.read_exact(&mut name_buf).await?;
            let name = String::from_utf8(name_buf).map_err(|e| {
                RTErrors::InvalidValue {
                    field:  String::from("name"),
                    reason: e.to_string()
                }
            })?;

            let payload_len = stream.read_u16_le().await? as usize;
            let mut payload = vec![0u8; payload_len];
            stream.read_exact(&mut payload).await?;

            Ok(RTInputEvent::Custom {
                name,
                payload
            })
        }
    }
}

pub async fn write_message(w: &mut OwnedWriteHalf, event: &RTInputEvent) -> RTResult<()> {
    let mut buf = Vec::with_capacity(32);

    match event {
        RTInputEvent::Key {
            keycode,
            modifiers,
            pressed
        } => {
            buf.push(RTTag::Key as u8);
            buf.extend_from_slice(&keycode.to_le_bytes());
            buf.push(modifiers.to_bits());
            buf.push(*pressed as u8);
        },

        RTInputEvent::MouseMove {
            x,
            y
        } => {
            buf.push(RTTag::MouseMove as u8);
            buf.extend_from_slice(&x.to_le_bytes());
            buf.extend_from_slice(&y.to_le_bytes());
        },

        RTInputEvent::Click {
            x,
            y,
            button
        } => {
            buf.push(RTTag::Click as u8);
            buf.extend_from_slice(&x.to_le_bytes());
            buf.extend_from_slice(&y.to_le_bytes());
            buf.push(*button as u8);
        },

        RTInputEvent::Scroll {
            x,
            y,
            delta_x,
            delta_y
        } => {
            buf.push(RTTag::Scroll as u8);
            buf.extend_from_slice(&x.to_le_bytes());
            buf.extend_from_slice(&y.to_le_bytes());
            buf.extend_from_slice(&delta_x.to_le_bytes());
            buf.extend_from_slice(&delta_y.to_le_bytes());
        },

        RTInputEvent::Custom {
            name,
            payload
        } => {
            let name_bytes = name.as_bytes();
            let name_len = u8::try_from(name_bytes.len()).map_err(|_| {
                RTErrors::Deserialize {
                    context: String::from("name length"),
                    reason:  String::from("exceeds u8::MAX")
                }
            })?;
            let payload_len = u16::try_from(payload.len()).map_err(|_| {
                RTErrors::Deserialize {
                    context: String::from("payload length"),
                    reason:  String::from("exceeds u16::MAX")
                }
            })?;

            buf.push(RTTag::Custom as u8);
            buf.push(name_len);
            buf.extend_from_slice(name_bytes);
            buf.extend_from_slice(&payload_len.to_le_bytes());
            buf.extend_from_slice(payload);
        }
    }

    w.write_all(&buf).await?;
    Ok(())
}
