mod handle;
mod net;

pub use handle::*;

use std::collections::VecDeque;
use std::fmt::Debug;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::net::TcpStream;
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Receiver, Sender};
use voxidian_protocol::packet::{DecodeError, EncodeError, PacketBuf, PacketEncode, PrefixedPacketDecode, Stage};
use voxidian_protocol::packet::c2s::handshake::C2SHandshakePackets;
use voxidian_protocol::packet::c2s::status::C2SStatusPackets;
use voxidian_protocol::packet::processing::{CompressionMode, PacketProcessing, SecretCipher};
use voxidian_protocol::packet::s2c::status::{StatusResponse, StatusResponsePlayers, StatusResponseS2CStatusPacket, StatusResponseVersion};
use crate::ServerHandle;

pub struct Connection {
    packet_sender: Sender<PacketBuf>,
    packet_receiver: Receiver<PacketBuf>,
    stream: TcpStream,
    incoming_bytes: VecDeque<u8>,
    packet_processing: PacketProcessing,
    stage: Stage,
    mark_for_removal: bool,
}

impl Connection {
    pub fn new(stream: TcpStream, handle: ServerHandle) -> ConnectionHandle {
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
                mark_for_removal: false,
            })),
            packet_sender: sender,
            server: handle
        }
    }

    pub(crate) fn handle_incoming_data(&mut self) {
        let mut buf = BufReader::new(&mut self.stream);
        match buf.fill_buf() {
            Ok(bytes) => {
                for byte in bytes {
                    let byte = self.packet_processing.secret_cipher.decrypt_u8(*byte).unwrap();
                    self.incoming_bytes.push_back(byte);
                }
            }
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock => {}
                _ => panic!("{:?}", err)
            }
        }
    }
}

