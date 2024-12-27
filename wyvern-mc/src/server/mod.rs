mod handle;
pub use handle::*;

use crate::ConnectionHandle;
use std::sync::{Arc, Mutex};
use voxidian_protocol::packet::c2s::config::C2SConfigPackets;
use voxidian_protocol::packet::c2s::handshake::C2SHandshakePackets;
use voxidian_protocol::packet::c2s::login::C2SLoginPackets;
use voxidian_protocol::packet::c2s::status::C2SStatusPackets;

type HandshakeEvent = fn(&C2SHandshakePackets, ConnectionHandle);
type StatusEvent = fn(&C2SStatusPackets, ConnectionHandle);
type LoginEvent = fn(&C2SLoginPackets, ConnectionHandle);
type ConfigEvent = fn(&C2SConfigPackets, ConnectionHandle);


#[derive(Default)]
pub struct Server {
    connections: Vec<ConnectionHandle>,

    handshake_events: Vec<HandshakeEvent>,
    status_events: Vec<StatusEvent>,
    login_events: Vec<LoginEvent>,
    config_events: Vec<ConfigEvent>,
}

impl Server {
    pub fn new() -> ServerHandle {
        ServerHandle {
            inner: Arc::new(Mutex::new(Server::default()))
        }
    }
}
