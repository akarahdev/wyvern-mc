use crate::server::{ConfigEvent, HandshakeEvent, LoginEvent, StatusEvent};
use crate::{Connection, Plugin, Server};
use std::io::ErrorKind;
use std::net::{TcpListener, ToSocketAddrs};
use std::sync::{Arc, Mutex};

use super::PlayEvent;

#[derive(Clone)]
pub struct ServerHandle {
    pub(crate) inner: Arc<Mutex<Server>>,
}

impl ServerHandle {
    pub fn add_plugin<P: Plugin>(self, plugin: P) -> Self {
        plugin.load(self.clone());
        self
    }

    pub fn handshake_event(self, event: HandshakeEvent) -> Self {
        self.inner.lock().unwrap().handshake_events.push(event);
        self
    }

    pub fn handshake_events(&self) -> Vec<HandshakeEvent> {
        self.inner.lock().unwrap().handshake_events.clone()
    }

    pub fn status_event(self, event: StatusEvent) -> Self {
        self.inner.lock().unwrap().status_events.push(event);
        self
    }

    pub fn status_events(&self) -> Vec<StatusEvent> {
        self.inner.lock().unwrap().status_events.clone()
    }

    pub fn login_event(self, event: LoginEvent) -> Self {
        self.inner.lock().unwrap().login_events.push(event);
        self
    }

    pub fn login_events(&self) -> Vec<LoginEvent> {
        self.inner.lock().unwrap().login_events.clone()
    }

    pub fn configuration_event(self, event: ConfigEvent) -> Self {
        self.inner.lock().unwrap().config_events.push(event);
        self
    }

    pub fn configuration_events(&self) -> Vec<ConfigEvent> {
        self.inner.lock().unwrap().config_events.clone()
    }

    pub fn play_event(self, event: PlayEvent) -> Self {
        self.inner.lock().unwrap().play_events.push(event);
        self
    }

    pub fn play_events(&self) -> Vec<PlayEvent> {
        self.inner.lock().unwrap().play_events.clone()
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
                .retain(|x| !x.marked_for_removal());

            let connections = self.inner.lock().unwrap().connections.clone();

            for connection in connections {
                connection.handle_incoming_data();
            }
        }
    }
}
