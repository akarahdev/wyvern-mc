use std::io::ErrorKind;
use std::net::{TcpListener, ToSocketAddrs};
use std::sync::{Arc, Mutex};
use crate::{Connection, Server};

#[derive(Clone)]
pub struct ServerHandle {
    pub(crate) inner: Arc<Mutex<Server>>
}

impl ServerHandle {
    pub fn start<S: ToSocketAddrs>(self, address: S) {
        let listener = TcpListener::bind(address).unwrap();
        listener.set_nonblocking(true).expect("must be able to do non-blocking IO to run server");
        loop {
            match listener.accept() {
                Ok(conn) => {
                    self.inner.lock().unwrap().connections.push(Connection::new(conn.0, self.clone()));
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {}
                Err(e) => panic!("{:?}", e)
            }
            self.inner.lock().unwrap().connections.retain(|x| !x.marked_for_removal());

            let connections = self.inner.lock().unwrap().connections.clone();

            for connection in connections {
                connection.handle_incoming_data();
            }
        }
    }
}