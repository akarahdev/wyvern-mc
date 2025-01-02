use std::{sync::atomic::Ordering, vec};

use voxidian_protocol::{packet::{c2s::play::{BlockFace, C2SPlayPackets, PlayerStatus}, s2c::play::{BlockChangedAckS2CPlayPacket, BlockUpdateS2CPlayPacket, KeepAliveS2CPlayPacket}, PacketEncodeDecode}, registry::RegEntry, value::{BlockPos, VarInt}};

use crate::{dimension::BlockState, plugin::Plugin, scheduler::{ConnectEvent, Event, MoveEvent, Param, PlayerTickEvent, Scheduler, SneakEvent, SprintEvent, TypeMap}, values::{BlockPosition, Key, Location, Vector}};

pub struct EventDispatcher;

impl Plugin for EventDispatcher {
    fn load(&self, server: &mut crate::ServerBuilder) {
        server.low_level(|server| {
            server.play_event(|packet, player| {
                match packet {
                    C2SPlayPackets::ClientTickEnd(packet) => {
                        let time = player.inner.player_data.time_alive.fetch_add(1, Ordering::AcqRel);
                        if time % 100 == 0 {
                            player.raw_handle().send_packet(KeepAliveS2CPlayPacket(1)).unwrap();
                        }

                        let mut data = TypeMap::new();
                        data.insert(Param::new(player.clone()));
                        data.insert(Event::new(PlayerTickEvent));
                        Scheduler::run_systems_with_map(&data);
                    }
                    C2SPlayPackets::PlayerInput(packet) => {
                        let old_is_sneaking = player.inner.player_data.is_sneaking.swap(packet.flags.sneak, Ordering::AcqRel);
                        if !old_is_sneaking {
                            let mut data = TypeMap::new();
                            data.insert(Param::new(player.clone()));
                            data.insert(Event::new(SneakEvent));
                            Scheduler::run_systems_with_map(&data);
                        }

                        let old_is_sprinting = player.inner.player_data.is_sprinting.swap(packet.flags.sprint, Ordering::AcqRel);
                        if !old_is_sprinting {
                            let mut data = TypeMap::new();
                            data.insert(Param::new(player.clone()));
                            data.insert(Event::new(SprintEvent));
                            Scheduler::run_systems_with_map(&data);
                        }
                    }
                    C2SPlayPackets::AcceptTeleportation(packet) => {
                        player
                            .inner
                            .player_data
                            .last_teleport_transaction_received
                            .store(packet.teleport_id.as_i32(), Ordering::Relaxed);

                        
                        if packet.teleport_id.as_i32() == 1 {
                            let mut data = TypeMap::new();
                            data.insert(Event::new(ConnectEvent));
                            data.insert(Param::new(player.clone()));
                            Scheduler::run_systems_with_map(&data);
                        }
                    }
                    C2SPlayPackets::MovePlayerPosRot(packet) => {
                        let loc = Location::new(packet.x, packet.y, packet.z, packet.pitch, packet.yaw);
                        if player.inner.player_data.last_teleport_transaction_sent.load(Ordering::Relaxed)
                            == player.inner.player_data.last_teleport_transaction_received.load(Ordering::Relaxed) {
                                *player.inner.player_data.last_position.lock().unwrap() = loc;

                                let mut data = TypeMap::new();
                                data.insert(Param::new(player.clone()));
                                data.insert(Param::new(loc.clone()));
                                data.insert(Event::new(MoveEvent));
                                Scheduler::run_systems_with_map(&data);
                            }
                    }
                    C2SPlayPackets::PlayerAction(packet) => {
                        match packet.status {
                            PlayerStatus::StartedDigging => {
                                let block_pos = BlockPosition::new(
                                    packet.location.x, 
                                    packet.location.y, 
                                    packet.location.z
                                );
                                println!("Breaking @ {:?}", block_pos);
                                player.dimension().set_block(block_pos, BlockState::new(Key::new("minecraft", "air")));
                                player.raw_handle().send_packet(BlockChangedAckS2CPlayPacket(packet.sequence)).unwrap();
                            },
                            PlayerStatus::CancelledDigging => todo!(),
                            PlayerStatus::FinishedDigging => todo!(),
                            PlayerStatus::DropItemStack => todo!(),
                            PlayerStatus::DropItem => todo!(),
                            PlayerStatus::FinishUsingItem => todo!(),
                            PlayerStatus::SwapItems => todo!(),
                        }
                    }
                    C2SPlayPackets::UseItemOn(packet) => {
                        let vector = match packet.face {
                            BlockFace::Down => BlockPosition::new(0, -1, 0),
                            BlockFace::Up => BlockPosition::new(0, 1, 0),
                            BlockFace::North => BlockPosition::new(0, 0, -1),
                            BlockFace::South => BlockPosition::new(0, 0, 1),
                            BlockFace::West => BlockPosition::new(-1, 0, 0),
                            BlockFace::East => BlockPosition::new(1, 0, 0)
                        };
                        let block_pos = BlockPosition::new(
                            packet.target.x + vector.x, 
                            packet.target.y + vector.y, 
                            packet.target.z + vector.z
                        );
                        println!("Placing @ {:?}", block_pos);
                        player.dimension().set_block(block_pos, BlockState::new(Key::new("minecraft", "oak_planks")));
                    }
                    _ => {}                    
                }
            });
        });
    }
}