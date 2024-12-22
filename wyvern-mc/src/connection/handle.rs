use crate::{Connection, ServerHandle};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use voxidian_protocol::packet::{EncodeError, PacketBuf, PacketEncode, Stage};

#[derive(Clone)]
pub struct ConnectionHandle {
    pub(crate) inner: Arc<Mutex<Connection>>,
    pub(crate) server: ServerHandle,
    pub(crate) packet_sender: Sender<PacketBuf>,
}

impl ConnectionHandle {
    pub fn send_packet<P: PacketEncode>(&self, packet: P) -> Result<(), EncodeError> {
        let mut buf = PacketBuf::new();
        buf.encode_write(packet)?;
        let _ = self.packet_sender.send(buf);
        Ok(())
    }

    pub fn get_stage(&self) -> Stage {
        self.inner.lock().unwrap().stage
    }

    pub fn set_stage(&self, stage: Stage) {
        self.inner.lock().unwrap().stage = stage;
    }
}
