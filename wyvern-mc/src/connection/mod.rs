mod handle;
mod net;

pub use handle::*;

use crate::ServerHandle;
use std::collections::VecDeque;
use std::io::{BufRead, BufReader, ErrorKind};
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex};
use voxidian_protocol::packet::processing::{CompressionMode, PacketProcessing, SecretCipher};
use voxidian_protocol::packet::{PacketBuf, Stage};

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
            server: handle,
        }
    }

    pub(crate) fn handle_incoming_data(&mut self) {
        let mut buf = BufReader::new(&mut self.stream);
        match buf.fill_buf() {
            Ok(bytes) => {
                for byte in bytes {
                    let byte = self
                        .packet_processing
                        .secret_cipher
                        .decrypt_u8(*byte)
                        .unwrap();
                    self.incoming_bytes.push_back(byte);
                }
            }
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock => {}
                _ => panic!("{:?}", err),
            },
        }
    }
}
