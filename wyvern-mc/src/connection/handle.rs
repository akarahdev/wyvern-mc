use crate::{Connection, ServerHandle};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use voxidian_protocol::packet::PacketBuf;

use super::protocol::ProtocolConnectionHandle;

#[derive(Clone)]
pub struct ConnectionHandle {
    pub(crate) inner: Arc<Mutex<Connection>>,
    pub(crate) server: ServerHandle,
    pub(crate) packet_sender: Sender<PacketBuf>,
}

impl ConnectionHandle {
    pub fn protocol_handle(&self) -> ProtocolConnectionHandle {
        ProtocolConnectionHandle {
            inner: self.inner.clone(),
            server: self.server.clone(),
            packet_sender: self.packet_sender.clone()
        }
    }
}