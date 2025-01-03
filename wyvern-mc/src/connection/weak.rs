use std::sync::{mpsc::Sender, Weak};

use voxidian_protocol::packet::PacketBuf;

use crate::Server;

use super::{ConnectionData, Player};

#[derive(Clone)]
pub struct WeakRefPlayer {
    pub(crate) inner: Weak<ConnectionData>,
    pub(crate) server: Server,
    pub(crate) packet_sender: Sender<PacketBuf>,
}

impl WeakRefPlayer {
    pub(crate) fn upgrade(&self) -> Option<Player> {
        self.inner.upgrade().map(|inner| Player {
            inner,
            server: self.server.clone(),
            packet_sender: self.packet_sender.clone(),
        })
    }
}