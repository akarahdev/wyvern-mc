use std::{
    fmt::Debug,
    sync::{Arc, Mutex, mpsc::Sender},
};

use voxidian_protocol::{
    packet::{EncodeError, PacketBuf, PacketEncode, PrefixedPacketEncode, Stage},
    value::VarInt,
};

use crate::Server;

use super::{ConnectionData, Connection};

#[derive(Clone)]
pub struct UnsafeConnection {
    pub(crate) inner: Arc<Mutex<ConnectionData>>,
    pub(crate) server: Server,
    pub(crate) packet_sender: Sender<PacketBuf>,
}

impl UnsafeConnection {
    pub fn to_safe(&self) -> Connection {
        Connection {
            inner: self.inner.clone(),
            server: self.server.clone(),
            packet_sender: self.packet_sender.clone(),
        }
    }

    pub fn send_packet<P: PrefixedPacketEncode + PacketEncode + Debug>(
        &self,
        packet: P,
    ) -> Result<(), EncodeError> {
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
