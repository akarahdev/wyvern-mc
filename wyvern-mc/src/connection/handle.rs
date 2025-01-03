use crate::dimension::Dimension;
use crate::inventory::PlayerInventory;
use crate::values::Location;
use crate::{ConnectionData, Server};
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use voxidian_protocol::packet::s2c::play::{PlayerPositionS2CPlayPacket, TeleportFlags};
use voxidian_protocol::packet::PacketBuf;
use voxidian_protocol::value::VarInt;

use super::protocol::RawConnection;
use super::{PlayerData, WeakRefPlayer};

#[derive(Clone)]
pub struct Player {
    pub(crate) inner: Arc<ConnectionData>,
    pub(crate) server: Server,
    pub(crate) packet_sender: Sender<PacketBuf>,
}

impl Player {
    pub(crate) fn data(&self) -> &PlayerData {
        self.inner.player_data.get().unwrap()
    }

    pub(crate) fn make_weak(&self) -> WeakRefPlayer {
        WeakRefPlayer {
            inner: Arc::downgrade(&self.inner),
            server: self.server.clone(),
            packet_sender: self.packet_sender.clone(),
        }
    }

    pub fn raw_handle(&self) -> RawConnection {
        RawConnection {
            inner: self.inner.clone(),
            server: self.server.clone(),
            packet_sender: self.packet_sender.clone(),
        }
    }

    pub fn set_dimension(&self, dim: Dimension) {
        let mut dim_ref = self.data().dimension.lock().unwrap();
        *dim_ref = Some(dim);
    }

    pub fn inventory(&self) -> PlayerInventory {
        self.data().inventory.clone()
    }

    pub fn dimension(&self) -> Dimension {
        self.data().dimension.lock().unwrap().clone().unwrap()
    }

    pub fn location(&self) -> Location {
        self.data().last_position.lock().unwrap().clone()
    }

    pub fn teleport(&self, location: Location) {
        *self.data().last_position.lock().unwrap() = location.clone();
        self.raw_handle().send_packet(PlayerPositionS2CPlayPacket {
            teleport_id: VarInt::from(
                self.data().last_teleport_transaction_sent.fetch_add(1, Ordering::AcqRel)+1
            ),
            x: location.x,
            y: location.y,
            z: location.z,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
            adyaw_deg: location.pitch,
            adpitch_deg: location.yaw,
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
