use std::sync::{Arc, Mutex};

use voxidian_protocol::packet::c2s::{config::C2SConfigPackets, handshake::C2SHandshakePackets, login::C2SLoginPackets, play::C2SPlayPackets, status::C2SStatusPackets};

use crate::ConnectionHandle;

use super::Server;

pub(crate) type HandshakeEvent = fn(&C2SHandshakePackets, ConnectionHandle);
pub(crate) type StatusEvent = fn(&C2SStatusPackets, ConnectionHandle);
pub(crate) type LoginEvent = fn(&C2SLoginPackets, ConnectionHandle);
pub(crate) type ConfigEvent = fn(&C2SConfigPackets, ConnectionHandle);
pub(crate) type PlayEvent = fn(&C2SPlayPackets, ConnectionHandle);

pub struct ProtocolServerHandle {
    pub(crate) inner: Arc<Mutex<Server>>,
}

impl ProtocolServerHandle {
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
}