use serde::{Deserialize, Serialize};

/// TCP connection state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TcpConnectionState {
    pub id: String,
    pub endpoint: String,
}

/// Event payload sent to frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPayload {
    pub id: String,
    pub event: EventType,
}

/// Event types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum EventType {
    /// Connection established
    Connect { endpoint: String },
    /// Connection closed
    Disconnect { endpoint: String, reason: DisconnectReason },
    /// Message received
    Message { endpoint: String, data: Vec<u8> },
    /// Error occurred
    Error { endpoint: String, message: String },
}

/// Disconnect reason
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisconnectReason {
    Normal,
    Reset,
    Timeout,
    Error(String),
}
