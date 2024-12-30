use crate::values::{Location, Position};
use crate::{Connection, ServerHandle};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use voxidian_protocol::packet::s2c::play::{PlayerPositionS2CPlayPacket, TeleportFlags};
use voxidian_protocol::packet::PacketBuf;
use voxidian_protocol::value::VarInt;

use super::protocol::ProtocolConnectionHandle;

#[derive(Clone)]
pub struct ConnectionHandle {
    pub(crate) inner: Arc<Mutex<Connection>>,
    pub(crate) server: ServerHandle,
    pub(crate) packet_sender: Sender<PacketBuf>,
}

impl ConnectionHandle {
    pub fn protocol_handle(&self) -> ProtocolConnectionHandle {
        ProtocolConnectionHandle {
            inner: self.inner.clone(),
            server: self.server.clone(),
            packet_sender: self.packet_sender.clone(),
        }
    }

    pub fn location(&self) -> Location {
        self.inner.lock().unwrap().player_data.last_position
    }

    pub fn teleport(&self, location: Location) {
        self.protocol_handle().send_packet(PlayerPositionS2CPlayPacket {
            teleport_id: VarInt::from(10),
            x: location.x(),
            y: location.y(),
            z: location.z(),
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            adyaw_deg: 0.0,
            adpitch_deg: 0.0,
            flags: TeleportFlags {
                relative_x: false,
                relative_y: false,
                relative_z: false,
                relative_pitch: false,
                relative_yaw: false,
                relative_vx: false,
                relative_vy: false,
                relative_vz: false,
                rotate_velocity: false,
            },
        }).unwrap();
    }
}
