use voxidian_protocol::{packet::{c2s::play::C2SPlayPackets, s2c::play::{BlockUpdateS2CPlayPacket, WorldChunkWithLightS2CPlayPacket}}, registry::RegEntry, value::{BlockPos, ChunkSection, ChunkSectionData, LengthPrefixVec, Nbt, NbtCompound, PaletteFormat, PalettedContainer, VarInt}};

use crate::plugin::Plugin;

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn load(&self, server: crate::ServerHandle) {
        server.low_level(|server| {
            server.play_event(|packet, _connection| {
                println!("Play Packet: {:?}", packet);
            }).play_event(|packet, connection| {
                let C2SPlayPackets::AcceptTeleportation(packet) = packet else {
                    return;
                };
    
                if packet.teleport_id != VarInt::from(1) {
                    return;
                }
    
                let connection = connection.protocol_handle();

                for chunk_x in -2..2 {
                    for chunk_z in -2..2 {
                        connection.send_packet(WorldChunkWithLightS2CPlayPacket {
                            chunk_x,
                            chunk_z,
                            heightmaps: Nbt { name: "".to_string(), root: NbtCompound::new() },
                            data: ChunkSectionData {
                                sections: vec![
                                    ChunkSection { 
                                        block_count: 0, 
                                        block_states: PalettedContainer {
                                            bits_per_entry: 0,
                                            format: PaletteFormat::SingleValued { entry: unsafe { RegEntry::new_unchecked(0) } }
                                        }, 
                                        biomes: PalettedContainer { 
                                            bits_per_entry: 0, 
                                            format: PaletteFormat::SingleValued { entry: unsafe { RegEntry::new_unchecked(0) } }
                                        }
                                    }
                                ]
                            },
                            block_entities: LengthPrefixVec::new(),
                            sky_light_mask: LengthPrefixVec::new(),
                            block_light_mask: LengthPrefixVec::new(),
                            empty_sky_light_mask: LengthPrefixVec::new(),
                            empty_block_light_mask: LengthPrefixVec::new(),
                            sky_light_array: LengthPrefixVec::new(),
                            block_light_array: LengthPrefixVec::new(),
                        }).unwrap();
                    }
                }
    
                connection.send_packet(BlockUpdateS2CPlayPacket {
                    pos: BlockPos::new(8, 5, 8),
                    block: unsafe { RegEntry::new_unchecked(3) },
                }).unwrap();
                connection.send_packet(BlockUpdateS2CPlayPacket {
                    pos: BlockPos::new(8, 5, 11),
                    block: unsafe { RegEntry::new_unchecked(4) },
                }).unwrap();
                connection.send_packet(BlockUpdateS2CPlayPacket {
                    pos: BlockPos::new(10, 5, 14),
                    block: unsafe { RegEntry::new_unchecked(5) },
                }).unwrap();
                connection.send_packet(BlockUpdateS2CPlayPacket {
                    pos: BlockPos::new(10, 5, 17),
                    block: unsafe { RegEntry::new_unchecked(5) },
                }).unwrap();
                connection.send_packet(BlockUpdateS2CPlayPacket {
                    pos: BlockPos::new(7, 5, 18),
                    block: unsafe { RegEntry::new_unchecked(5) },
                }).unwrap();
            })
        });
    }
}