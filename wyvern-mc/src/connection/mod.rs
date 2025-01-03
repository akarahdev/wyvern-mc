mod handle;
mod net;
use data::PlayerData;
pub use handle::*;
pub mod protocol;
mod data;

mod weak;
pub use weak::*;

use crate::Server;
use std::collections::VecDeque;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::sync::{Arc, Mutex, OnceLock};
use voxidian_protocol::packet::processing::{CompressionMode, PacketProcessing, SecretCipher};
use voxidian_protocol::packet::{PacketBuf, Stage};

pub struct ConnectionData {
    pub(crate) packet_sender: Sender<PacketBuf>,
    pub(crate) packet_receiver: Mutex<Receiver<PacketBuf>>,
    pub(crate) stream: Mutex<TcpStream>,
    pub(crate) incoming_bytes: Mutex<VecDeque<u8>>,
    pub(crate) packet_processing: Mutex<PacketProcessing>,
    pub(crate) stage: Mutex<Stage>,
    pub(crate) mark_for_removal: AtomicBool,
    pub(crate) player_data: OnceLock<PlayerData>
}

impl ConnectionData {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(stream: TcpStream, handle: Server) -> Player {
        let (sender, recv) = channel();
        let player = Player {
            inner: Arc::new(ConnectionData {
                packet_sender: sender.clone(),
                packet_receiver: Mutex::new(recv),
                stream: Mutex::new(stream),
                incoming_bytes: Mutex::new(VecDeque::new()),
                packet_processing: Mutex::new(PacketProcessing {
                    secret_cipher: SecretCipher::no_cipher(),
                    compression: CompressionMode::None,
                }),
                stage: Mutex::new(Stage::Handshake),
                mark_for_removal: AtomicBool::new(false),
                player_data: OnceLock::new(),
            }),
            packet_sender: sender,
            server: handle,
        };
        let _ = player.inner.player_data.set(PlayerData::new(player.clone()));
        player
    }

    pub(crate) fn handle_incoming_data(&self) {
        let mut buf = [0u8; 512];

        let mut stream = self.stream.lock().unwrap();
        let mut incoming_bytes = self.incoming_bytes.lock().unwrap();
        let mut packet_processing  = self.packet_processing.lock().unwrap();

        match stream.read(&mut buf) {
            Ok(bytes) => {
                for item in buf.iter().take(bytes) {
                    let byte = packet_processing
                        .secret_cipher
                        .decrypt_u8(*item)
                        .unwrap();
                    incoming_bytes.push_back(byte);
                }
            }
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock => {}
                ErrorKind::Interrupted => {}
                _ => panic!("{:?}", err),
            },
        }
    }

    pub(crate) fn handle_outgoing_data(&self) {
        loop {
            let Ok(buf) = self.packet_receiver.lock().unwrap().try_recv() else {
                return;
            };
            match self.stream.lock().unwrap().write_all(buf.as_slice()) {
                Ok(()) => {}
                Err(_e) => {
                    self.packet_sender.send(buf).unwrap();
                    break;
                }
            }
        }
    }
}
