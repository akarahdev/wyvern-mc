mod handle;
mod net;

pub use handle::*;

use crate::ServerHandle;
use std::collections::VecDeque;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{channel, Receiver, Sender};
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
        let mut buf = [0u8; 256];

        match self.stream.read(&mut buf) {
            Ok(bytes) => {
                for idx in 0..bytes {
                    let byte = self
                        .packet_processing
                        .secret_cipher
                        .decrypt_u8(buf[idx])
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
        loop {
            let Ok(buf) = self.packet_receiver.try_recv() else {
                return;
            };
            match self.stream.write_all(buf.as_slice()) {
                Ok(()) => {
                    return;
                }
                Err(e) => {
                    self.packet_sender.send(buf).unwrap();
                    return;
                }
            }
        }
    }
}
