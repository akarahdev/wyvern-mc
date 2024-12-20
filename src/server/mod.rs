pub mod net;

use std::sync::LazyLock;
use voxidian_protocol::packet::PacketBuf;
use crate::ConnectionHandle;

pub struct Server {
    connections: Vec<ConnectionHandle>,
    events: Vec<fn(PacketBuf)>
}

impl Server {
    pub fn spawn(address: String) {
        let server = Server {
            connections: Vec::new(),
            events: Vec::new()
        };
        server.start();
    }
}