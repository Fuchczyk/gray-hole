use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ClientMessage {
    Ready,
    ReadyToListen,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    YouListen,
    YouConnect { address: SocketAddr },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConnectionMessage {
    DataLoad { data: Vec<u8> },
}
