use std::io::{Error, Read};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use voxidian_protocol::packet::{DecodeError, EncodeError, PacketBuf, PacketEncode};

pub struct Connection {
    packet_sender: Sender<PacketBuf>,
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> (ConnectionHandle, Receiver<PacketBuf>) {
        let (sender, recv) = channel();
        (ConnectionHandle {
            inner: Arc::new(Mutex::new(Connection {
                packet_sender: sender.clone(),
                stream
            })),
            packet_sender: sender
        }, recv)
    }
}

#[derive(Clone)]
pub struct ConnectionHandle {
    inner: Arc<Mutex<Connection>>,
    packet_sender: Sender<PacketBuf>
}

impl ConnectionHandle {
    pub fn send_packet<P: PacketEncode>(&self, packet: P) -> Result<(), EncodeError> {
        let mut buf = PacketBuf::new();
        buf.encode_write(packet)?;
        let _ = self.packet_sender.send(buf);
        Ok(())
    }
}