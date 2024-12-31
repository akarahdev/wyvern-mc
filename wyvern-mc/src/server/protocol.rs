use std::sync::{Arc, Mutex};

use voxidian_protocol::packet::c2s::{
    config::C2SConfigPackets, handshake::C2SHandshakePackets, login::C2SLoginPackets,
    play::C2SPlayPackets, status::C2SStatusPackets,
};

use crate::Player;

use super::ServerData;

pub(crate) type HandshakeEvent = fn(&C2SHandshakePackets, Player);
pub(crate) type StatusEvent = fn(&C2SStatusPackets, Player);
pub(crate) type LoginEvent = fn(&C2SLoginPackets, Player);
pub(crate) type ConfigEvent = fn(&C2SConfigPackets, Player);
pub(crate) type PlayEvent = fn(&C2SPlayPackets, Player);

pub struct UnsafeServer {
    pub(crate) inner: Arc<Mutex<ServerData>>,
}

impl UnsafeServer {
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
