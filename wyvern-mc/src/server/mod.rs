use std::io::ErrorKind;
use std::net::{TcpListener, ToSocketAddrs};
use voxidian_protocol::packet::PacketBuf;
use crate::{Connection, ConnectionHandle};
use crate::plugin::Plugin;

pub struct Server {
    connections: Vec<ConnectionHandle>,
    events: Vec<fn(PacketBuf)>
}

impl Server {
    pub fn new() -> Server {
        Server {
            connections: Vec::new(),
            events: Vec::new()
        }
    }

    pub fn add_plugin<P: Plugin>(mut self, plugin: P) -> Self {
        plugin.load(&mut self);
        self
    }

    pub fn start<S: ToSocketAddrs>(mut self, address: S) {
        let listener = TcpListener::bind(address).unwrap();
        listener.set_nonblocking(true);
        loop {
            match listener.accept() {
                Ok(conn) => {
                    self.connections.push(Connection::new(conn.0));
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {}
                Err(e) => panic!("{:?}", e)
            }
            self.connections.retain(|x| !x.marked_for_removal());
            for connection in &mut self.connections {
                connection.handle_incoming_data();
            }
        }
    }
}