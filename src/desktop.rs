use std::collections::HashMap;
use std::net::ToSocketAddrs;
use std::sync::Arc;
use std::time::Duration;

use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Emitter, Runtime};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::{Mutex, RwLock},
    task::JoinHandle,
};

use crate::error::{Error, Result};
use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> Result<Tcpclient<R>> {
    Ok(Tcpclient {
        app: app.clone(),
        connections: Arc::new(RwLock::new(HashMap::new())),
    })
}

/// TCP connection storage
struct TcpConnection {
    task: JoinHandle<()>,
    write_half: Mutex<tokio::net::tcp::OwnedWriteHalf>,
    endpoint: String,
}

/// Access to the tcpclient APIs.
pub struct Tcpclient<R: Runtime> {
    app: AppHandle<R>,
    connections: Arc<RwLock<HashMap<String, TcpConnection>>>,
}

const EVENT_NAME: &str = "plugin://tcpclient";

/// Check if the error indicates connection reset
fn is_connection_reset_error(e: &std::io::Error) -> bool {
    e.kind() == std::io::ErrorKind::ConnectionReset
        || e.kind() == std::io::ErrorKind::ConnectionAborted
        || e.raw_os_error() == Some(10054) // WSAECONNRESET on Windows
        || e.raw_os_error() == Some(53) // ECONNRESET on Unix
}

/// Check if the error indicates connection timed out
fn is_timeout_error(e: &std::io::Error) -> bool {
    e.kind() == std::io::ErrorKind::TimedOut || e.kind() == std::io::ErrorKind::WouldBlock
}

impl<R: Runtime> Tcpclient<R> {
    /// Connect to a TCP server
    pub async fn connect(&self, id: String, endpoint: String) -> Result<()> {
        // Remove existing connection if exists
        if let Some(conn) = self.connections.read().await.get(&id) {
            conn.task.abort();
            self.connections.write().await.remove(&id);
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        // Establish connection
        let stream = TcpStream::connect(&endpoint).await?;
        let (mut read_half, write_half) = stream.into_split();

        // Emit connect event
        let _ = self.app.emit(
            EVENT_NAME,
            EventPayload {
                id: id.clone(),
                event: EventType::Connect {
                    endpoint: endpoint.clone(),
                },
            },
        );

        // Clone for the spawn task
        let app = self.app.clone();
        let connections_clone = self.connections.clone();
        let tcp_id = id.clone();
        let endpoint_clone = endpoint.clone();
        let task = tokio::task::spawn(async move {
            let mut buf = [0u8; 65535];
            loop {
                match read_half.read(&mut buf).await {
                    Ok(0) => {
                        // Normal disconnect (FIN)
                        let _ = app.emit(
                            EVENT_NAME,
                            EventPayload {
                                id: tcp_id.clone(),
                                event: EventType::Disconnect {
                                    endpoint: endpoint_clone.clone(),
                                    reason: DisconnectReason::Normal,
                                },
                            },
                        );
                        connections_clone.write().await.remove(&tcp_id);
                        break;
                    }
                    Ok(len) => {
                        // Data received
                        let _ = app.emit(
                            EVENT_NAME,
                            EventPayload {
                                id: tcp_id.clone(),
                                event: EventType::Message {
                                    endpoint: endpoint_clone.clone(),
                                    data: buf[..len].to_vec(),
                                },
                            },
                        );
                    }
                    Err(e) => {
                        // Error occurred - handles Windows disconnect issue
                        let reason = if is_connection_reset_error(&e) {
                            DisconnectReason::Reset
                        } else if is_timeout_error(&e) {
                            DisconnectReason::Timeout
                        } else {
                            DisconnectReason::Error(e.to_string())
                        };

                        let _ = app.emit(
                            EVENT_NAME,
                            EventPayload {
                                id: tcp_id.clone(),
                                event: EventType::Disconnect {
                                    endpoint: endpoint_clone.clone(),
                                    reason,
                                },
                            },
                        );
                        connections_clone.write().await.remove(&tcp_id);
                        break;
                    }
                }
            }
        });

        // Insert the new connection
        self.connections.write().await.insert(
            id,
            TcpConnection {
                task,
                write_half: Mutex::new(write_half),
                endpoint,
            },
        );

        Ok(())
    }

    /// Connect to a TCP server with a specific local address
    pub async fn connect_with_bind(
        &self,
        id: String,
        local_addr: String,
        endpoint: String,
    ) -> Result<()> {
        // Remove existing connection if exists
        if let Some(conn) = self.connections.read().await.get(&id) {
            conn.task.abort();
            self.connections.write().await.remove(&id);
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        // Resolve addresses
        let local = local_addr
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| Error::ConnectionError("Invalid local address".into()))?;

        let remote = endpoint
            .to_socket_addrs()?
            .next()
            .ok_or_else(|| Error::ConnectionError("Invalid endpoint".into()))?;

        // Create socket and bind to local address
        use socket2::{Domain, Socket, Type};
        let socket = Socket::new(Domain::for_address(remote), Type::STREAM, None)?;
        socket.bind(&local.into())?;

        // Connect (blocking mode for connect, then non-blocking for tokio)
        socket.set_nonblocking(false)?;
        socket.connect(&remote.into())?;
        socket.set_nonblocking(true)?;

        let stream = TcpStream::from_std(socket.into())?;
        let (mut read_half, write_half) = stream.into_split();

        // Emit connect event
        let _ = self.app.emit(
            EVENT_NAME,
            EventPayload {
                id: id.clone(),
                event: EventType::Connect {
                    endpoint: endpoint.clone(),
                },
            },
        );

        // Clone for the spawn task
        let app = self.app.clone();
        let connections_clone = self.connections.clone();
        let tcp_id = id.clone();
        let endpoint_clone = endpoint.clone();
        let task = tokio::task::spawn(async move {
            let mut buf = [0u8; 65535];
            loop {
                match read_half.read(&mut buf).await {
                    Ok(0) => {
                        let _ = app.emit(
                            EVENT_NAME,
                            EventPayload {
                                id: tcp_id.clone(),
                                event: EventType::Disconnect {
                                    endpoint: endpoint_clone.clone(),
                                    reason: DisconnectReason::Normal,
                                },
                            },
                        );
                        connections_clone.write().await.remove(&tcp_id);
                        break;
                    }
                    Ok(len) => {
                        let _ = app.emit(
                            EVENT_NAME,
                            EventPayload {
                                id: tcp_id.clone(),
                                event: EventType::Message {
                                    endpoint: endpoint_clone.clone(),
                                    data: buf[..len].to_vec(),
                                },
                            },
                        );
                    }
                    Err(e) => {
                        let reason = if is_connection_reset_error(&e) {
                            DisconnectReason::Reset
                        } else if is_timeout_error(&e) {
                            DisconnectReason::Timeout
                        } else {
                            DisconnectReason::Error(e.to_string())
                        };

                        let _ = app.emit(
                            EVENT_NAME,
                            EventPayload {
                                id: tcp_id.clone(),
                                event: EventType::Disconnect {
                                    endpoint: endpoint_clone.clone(),
                                    reason,
                                },
                            },
                        );
                        connections_clone.write().await.remove(&tcp_id);
                        break;
                    }
                }
            }
        });

        // Insert the new connection
        self.connections.write().await.insert(
            id,
            TcpConnection {
                task,
                write_half: Mutex::new(write_half),
                endpoint,
            },
        );

        Ok(())
    }

    /// Disconnect from server
    pub async fn disconnect(&self, id: String) -> Result<()> {
        let mut connections = self.connections.write().await;

        if let Some(conn) = connections.remove(&id) {
            conn.task.abort();
            Ok(())
        } else {
            Err(Error::ConnectionNotFound(id))
        }
    }

    /// Send data to server
    pub async fn send(&self, id: String, data: Vec<u8>) -> Result<()> {
        let connections = self.connections.read().await;

        if let Some(conn) = connections.get(&id) {
            let mut write_half = conn.write_half.lock().await;
            write_half.write_all(&data).await?;
            Ok(())
        } else {
            Err(Error::ConnectionNotFound(id))
        }
    }

    /// Check if connection exists
    pub async fn is_connected(&self, id: String) -> bool {
        self.connections.read().await.contains_key(&id)
    }

    /// Get all connection IDs
    pub async fn get_connections(&self) -> Vec<String> {
        self.connections.read().await.keys().cloned().collect()
    }
}
