mod handle;
mod net;
use data::PlayerData;
pub use handle::*;
pub mod protocol;
mod data;

use crate::Server;
use std::collections::VecDeque;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use voxidian_protocol::packet::processing::{CompressionMode, PacketProcessing, SecretCipher};
use voxidian_protocol::packet::{PacketBuf, Stage};

pub struct ConnectionData {
    packet_sender: Sender<PacketBuf>,
    packet_receiver: Receiver<PacketBuf>,
    stream: TcpStream,
    incoming_bytes: VecDeque<u8>,
    packet_processing: PacketProcessing,
    stage: Stage,
    mark_for_removal: bool,
    player_data: PlayerData
}

impl ConnectionData {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(stream: TcpStream, handle: Server) -> Player {
        let (sender, recv) = channel();
        Player {
            inner: Arc::new(Mutex::new(ConnectionData {
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
                player_data: PlayerData::default(),
            })),
            packet_sender: sender,
            server: handle,
        }
    }

    pub(crate) fn handle_incoming_data(&mut self) {
        let mut buf = [0u8; 256];

        match self.stream.read(&mut buf) {
            Ok(bytes) => {
                for item in buf.iter().take(bytes) {
                    let byte = self
                        .packet_processing
                        .secret_cipher
                        .decrypt_u8(*item)
                        .unwrap();
                    self.incoming_bytes.push_back(byte);
                }
            }
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock => {}
                ErrorKind::Interrupted => {}
                _ => panic!("{:?}", err),
            },
        }
    }

    pub(crate) fn handle_outgoing_data(&mut self) {
        let Ok(buf) = self.packet_receiver.try_recv() else {
            return;
        };
        match self.stream.write_all(buf.as_slice()) {
            Ok(()) => {}
            Err(_e) => {
                self.packet_sender.send(buf).unwrap();
            }
        }
    }
}
