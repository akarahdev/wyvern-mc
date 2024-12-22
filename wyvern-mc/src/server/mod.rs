mod handle;
pub use handle::*;

use crate::ConnectionHandle;
use crate::plugin::Plugin;
use std::net::ToSocketAddrs;
use std::sync::{Arc, Mutex};
use voxidian_protocol::packet::PacketBuf;

pub struct Server {
    connections: Vec<ConnectionHandle>,
    events: Vec<fn(PacketBuf)>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            connections: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn add_plugin<P: Plugin>(mut self, plugin: P) -> Self {
        plugin.load(&mut self);
        self
    }

    pub fn start<S: ToSocketAddrs>(self, address: S) {
        let handle = ServerHandle {
            inner: Arc::new(Mutex::new(self)),
        };
        handle.start(address);
    }
}
