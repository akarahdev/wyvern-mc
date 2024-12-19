pub mod net;

use std::cell::{LazyCell, OnceCell};
use std::sync::LazyLock;
use async_executor::{Executor, LocalExecutor};
use async_net::TcpListener;
use voxidian_protocol::packet::PacketBuf;
use crate::ConnectionHandle;

pub struct Server {
    connections: Vec<ConnectionHandle>,
    events: Vec<fn(PacketBuf)>
}

static SERVER_EXECUTOR: LazyLock<Executor> = LazyLock::new(|| { Executor::new() });
impl Server {
    pub fn spawn(address: String) {
        let server = Server {
            connections: Vec::new(),
            events: Vec::new()
        };
        server.start();
    }
}