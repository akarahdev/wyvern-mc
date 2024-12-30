use crate::{Connection, Server, plugin::Plugin};
use std::io::ErrorKind;
use std::net::{TcpListener, ToSocketAddrs};
use std::sync::{Arc, Mutex};

use super::ProtocolServerHandle;

#[derive(Clone)]
pub struct ServerHandle {
    pub(crate) inner: Arc<Mutex<Server>>,
}

impl ServerHandle {
    pub fn add_plugin<P: Plugin>(self, plugin: P) -> Self {
        plugin.load(self.clone());
        self
    }

    pub(crate) fn low_level<F: FnOnce(ProtocolServerHandle) -> ProtocolServerHandle>(
        self,
        f: F,
    ) -> Self {
        let handle = ProtocolServerHandle {
            inner: self.inner.clone(),
        };
        f(handle);
        self
    }

    pub(crate) fn get_low_level(&self) -> ProtocolServerHandle {
        ProtocolServerHandle {
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
                        .push(Connection::new(conn.0, self.clone()));
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {}
                Err(e) => panic!("{:?}", e),
            }
            self.inner
                .lock()
                .unwrap()
                .connections
                .retain(|x| !x.protocol_handle().marked_for_removal());

            let connections = self.inner.lock().unwrap().connections.clone();

            for connection in connections {
                connection.protocol_handle().handle_incoming_data();
            }
        }
    }
}
