use std::{fmt::Debug, sync::{mpsc::Sender, Arc, Mutex}};

use voxidian_protocol::{packet::{EncodeError, PacketBuf, PacketEncode, PrefixedPacketEncode, Stage}, value::VarInt};

use crate::ServerHandle;

use super::{Connection, ConnectionHandle};

#[derive(Clone)]
pub struct ProtocolConnectionHandle {
    pub(crate) inner: Arc<Mutex<Connection>>,
    pub(crate) server: ServerHandle,
    pub(crate) packet_sender: Sender<PacketBuf>,
}

impl ProtocolConnectionHandle {
    pub fn to_normal(&self) -> ConnectionHandle {
        ConnectionHandle {
            inner: self.inner.clone(),
            server: self.server.clone(),
            packet_sender: self.packet_sender.clone()
        }
    }

    pub fn send_packet<P: PrefixedPacketEncode + PacketEncode + Debug>(&self, packet: P) -> Result<(), EncodeError> {
        let mut tmp_buf = PacketBuf::new();
        packet.encode_prefixed(&mut tmp_buf)?;
        let mut buf = PacketBuf::new();
        VarInt::from(tmp_buf.iter().count()).encode(&mut buf)?;
        buf.write_u8s(tmp_buf.as_slice());
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
