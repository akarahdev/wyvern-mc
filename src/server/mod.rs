pub mod net;

use std::cell::{LazyCell, OnceCell};
use std::sync::LazyLock;
use async_executor::{Executor, LocalExecutor};
use async_net::TcpListener;
use voxidian_protocol::packet::PacketBuf;
use crate::ConnectionHandle;

pub struct Server {
    tcp_listener: TcpListener,
    connections: Vec<ConnectionHandle>,
    events: Vec<fn(PacketBuf)>
}

static SERVER_EXECUTOR: LazyLock<Executor> = LazyLock::new(|| { Executor::new() });
impl Server {
    pub fn spawn(address: String) {
        SERVER_EXECUTOR.spawn(async move {
            let listener = TcpListener::bind(address).await.unwrap();
            let server = Server {
                tcp_listener: listener,
                connections: Vec::new(),
                events: Vec::new()
            };
            server.start();
        }).detach();
    }
}