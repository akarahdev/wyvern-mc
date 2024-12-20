use std::collections::VecDeque;
use std::fmt::Debug;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use voxidian_protocol::packet::{DecodeError, EncodeError, PacketBuf, PacketEncode, PrefixedPacketDecode, Stage};
use voxidian_protocol::packet::c2s::handshake::C2SHandshakePackets;
use voxidian_protocol::packet::processing::{CompressionMode, PacketProcessing, SecretCipher};

pub struct Connection {
    packet_sender: Sender<PacketBuf>,
    packet_receiver: Receiver<PacketBuf>,
    stream: TcpStream,
    incoming_bytes: VecDeque<u8>,
    packet_processing: PacketProcessing,
    stage: Stage
}

impl Connection {
    pub fn new(stream: TcpStream) -> ConnectionHandle {
        let (sender, recv) = channel();
        ConnectionHandle {
            inner: Arc::new(Mutex::new(Connection {
                packet_sender: sender.clone(),
                packet_receiver: recv,
                stream,
                incoming_bytes: VecDeque::new(),
                packet_processing: PacketProcessing {
                    secret_cipher: SecretCipher::no_cipher(),
                    compression: CompressionMode::None,
                },
                stage: Stage::Handshake,
            })),
            packet_sender: sender,
        }
    }

    pub(crate) fn handle_incoming_data(&mut self) {
        let mut buf = BufReader::new(&mut self.stream);
        match buf.fill_buf() {
            Ok(bytes) => {
                self.incoming_bytes.extend(
                    bytes
                        .iter()
                        .map(|byte| self.packet_processing.secret_cipher.encrypt_u8(*byte))
                        .map(|byte| byte.unwrap())
                );
            }
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock => {}
                _ => panic!("{:?}", err)
            }
        }
        self.handle_packets();
    }

    pub(crate) fn handle_packets(&mut self) {
        match self.stage {
            Stage::Handshake => {
                self.parse_packets(|packet: C2SHandshakePackets| {
                    println!("{:?}", packet);
                });
            }
            _ => {}
        }
    }

    pub(crate) fn parse_packets<T: PrefixedPacketDecode + Debug, F: Fn(T)>(&mut self, f: F) {
        match self.packet_processing.decode_from_raw_queue(self.incoming_bytes.iter().map(|x| *x)) {
            Ok((mut buf, consumed)) => {
                for _ in 0..consumed { self.incoming_bytes.pop_front(); }
                match (T::decode_prefixed(&mut buf)) {
                    Ok(packet) => {
                        f(packet)
                    },
                    Err(e) => panic!("{:?}", e)
                }
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
}

#[derive(Clone)]
pub struct ConnectionHandle {
    inner: Arc<Mutex<Connection>>,
    packet_sender: Sender<PacketBuf>,
}

impl ConnectionHandle {
    pub fn send_packet<P: PacketEncode>(&self, packet: P) -> Result<(), EncodeError> {
        let mut buf = PacketBuf::new();
        buf.encode_write(packet)?;
        let _ = self.packet_sender.send(buf);
        Ok(())
    }

    pub(crate) fn handle_incoming_data(&self) {
        self.inner.lock().unwrap().handle_incoming_data();
    }
}