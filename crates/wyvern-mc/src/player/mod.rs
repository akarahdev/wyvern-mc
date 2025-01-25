use std::{collections::VecDeque, net::IpAddr};

use data::PlayerData;
use net::ConnectionStoppedSignal;
use tokio::{
    net::TcpStream,
    sync::mpsc::{self, Receiver, Sender},
};
use voxidian_protocol::{
    packet::{PacketBuf, PacketEncode, PrefixedPacketEncode, Stage, processing::PacketProcessing},
    value::VarInt,
};

use crate::{dimension::Dimension, server::Server};

pub mod chunkload;
pub mod data;
pub mod net;
pub mod stages;

use crate as wyvern_mc;

#[crate::actor(Player, PlayerMessage)]
pub struct ConnectionData {
    pub(crate) stream: TcpStream,
    #[allow(dead_code)]
    pub(crate) addr: IpAddr,
    pub(crate) received_bytes: VecDeque<u8>,
    pub(crate) bytes_to_send: Vec<u8>,
    pub(crate) packet_processing: PacketProcessing,
    pub(crate) signal: mpsc::Sender<ConnectionStoppedSignal>,
    pub(crate) connected_server: Server,
    pub(crate) stage: Stage,
    pub(crate) associated_data: PlayerData,
    pub(crate) sender: Sender<PlayerMessage>,
}

#[crate::message(Player, PlayerMessage)]
impl ConnectionData {
    #[SetStage]
    pub async fn set_stage(&mut self, stage: Stage) {
        self.stage = stage;
    }

    #[GetStage]
    pub async fn get_stage(&mut self) -> Stage {
        self.stage.clone()
    }

    #[SendPacketBuf]
    pub async fn send_packet_buf(&mut self, buf: PacketBuf) {
        self.bytes_to_send.extend(buf.iter());
    }

    #[GetServer]
    pub async fn get_server(&self) -> Server {
        self.connected_server.clone()
    }

    #[GetDimension]
    pub async fn get_dimension(&self) -> Dimension {
        self.associated_data.dimension.clone().unwrap()
    }
}

impl Player {
    pub async fn write_packet<P: PrefixedPacketEncode + std::fmt::Debug>(&self, packet: P) {
        let mut buf = PacketBuf::new();
        packet.encode_prefixed(&mut buf).unwrap();

        let mut len_buf = PacketBuf::new();
        VarInt::from(buf.iter().count())
            .encode(&mut len_buf)
            .unwrap();

        self.send_packet_buf(len_buf).await;
        self.send_packet_buf(buf).await;
    }
}

impl ConnectionData {
    pub async fn write_packet<P: PrefixedPacketEncode + std::fmt::Debug>(&mut self, packet: P) {
        let mut buf = PacketBuf::new();
        packet.encode_prefixed(&mut buf).unwrap();

        let mut len_buf = PacketBuf::new();
        VarInt::from(buf.iter().count())
            .encode(&mut len_buf)
            .unwrap();

        self.send_packet_buf(len_buf).await;
        self.send_packet_buf(buf).await;
    }
}

#[derive(Debug)]
pub struct ConnectionWithSignal {
    pub(crate) player: Player,
    pub(crate) _signal: Receiver<ConnectionStoppedSignal>,
}

impl ConnectionWithSignal {
    pub fn lower(&self) -> Player {
        self.player.clone()
    }
}
