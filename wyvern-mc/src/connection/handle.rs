use crate::{Connection, ServerHandle};
use std::fmt::Debug;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use voxidian_protocol::packet::{EncodeError, PacketBuf, PacketEncode, PrefixedPacketEncode, Stage};
use voxidian_protocol::value::VarInt;

#[derive(Clone)]
pub struct ConnectionHandle {
    pub(crate) inner: Arc<Mutex<Connection>>,
    pub(crate) server: ServerHandle,
    pub(crate) packet_sender: Sender<PacketBuf>,
}

impl ConnectionHandle {
    pub fn send_packet<P: PrefixedPacketEncode + PacketEncode + Debug>(&self, packet: P) -> Result<(), EncodeError> {
        println!("OUT: {:?}", packet);
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
