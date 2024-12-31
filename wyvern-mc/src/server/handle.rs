use crate::Player;
use crate::{ConnectionData, ServerData, plugin::Plugin};
use std::io::ErrorKind;
use std::net::{TcpListener, ToSocketAddrs};
use std::sync::{Arc, Mutex};

use super::UnsafeServer;

#[derive(Clone)]
pub struct Server {
    pub(crate) inner: Arc<Mutex<ServerData>>,
}

impl Server {
    pub fn add_plugin<P: Plugin>(self, plugin: P) -> Self {
        plugin.load(self.clone());
        self
    }

    pub fn low_level<F: FnOnce(UnsafeServer) -> UnsafeServer>(
        self,
        f: F,
    ) -> Self {
        let handle = UnsafeServer {
            inner: self.inner.clone(),
        };
        f(handle);
        self
    }

    pub fn get_low_level(&self) -> UnsafeServer {
        UnsafeServer {
            inner: self.inner.clone(),
        }
    }

    pub fn start<S: ToSocketAddrs>(self, address: S) {
        let listener = TcpListener::bind(address).unwrap();
        listener
            .set_nonblocking(true)
            .expect("must be able to do non-blocking IO to run server");
        loop {
            match listener.accept() {
                Ok(conn) => {
                    conn.0.set_nonblocking(true).unwrap();
                    self.inner
                        .lock()
                        .unwrap()
                        .connections
                        .push(ConnectionData::new(conn.0, self.clone()));
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {}
                Err(e) => panic!("{:?}", e),
            }
            self.inner
                .lock()
                .unwrap()
                .connections
                .retain(|x| !x.raw_handle().marked_for_removal());

            let connections = self.inner.lock().unwrap().connections.clone();

            for connection in connections {
                connection.raw_handle().handle_incoming_data();
            }
        }
    }

    pub fn connections(&self) -> Vec<Player> {
        self.inner.lock().unwrap().connections.clone()
    }
}
