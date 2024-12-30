use std::vec;

use crate::plugin::Plugin;
use crate::{connection, ServerHandle};
use voxidian_protocol::packet::c2s::config::C2SConfigPackets;
use voxidian_protocol::packet::c2s::handshake::C2SHandshakePackets;
use voxidian_protocol::packet::c2s::login::C2SLoginPackets;
use voxidian_protocol::packet::c2s::play::C2SPlayPackets;
use voxidian_protocol::packet::c2s::status::C2SStatusPackets;
use voxidian_protocol::packet::s2c::config::{CustomPayloadS2CConfigPacket, FinishConfigurationS2CConfigPacket, KnownPack, SelectKnownPacksS2CConfigPacket};
use voxidian_protocol::packet::s2c::login::{LoginFinishedS2CLoginPacket, LoginSuccessProperty};
use voxidian_protocol::packet::s2c::play::{BlockUpdateS2CPlayPacket, GameEvent, GameEventS2CPlayPacket, Gamemode, LoginS2CPlayPacket, PlayerPositionS2CPlayPacket, TeleportFlags, WorldChunkWithLightS2CPlayPacket};
use voxidian_protocol::packet::s2c::status::{PongResponseS2CStatusPacket, StatusResponse, StatusResponsePlayers, StatusResponseVersion};
use voxidian_protocol::packet::Stage;
use voxidian_protocol::registry::{RegEntry, Registry};
use voxidian_protocol::value::{Biome, BlockPos, ChunkSection, ChunkSectionData, ConsumeAllVec, DamageType, DimEffects, DimMonsterSpawnLightLevel, DimType, Identifier, LengthPrefixHashMap, LengthPrefixVec, Nbt, NbtCompound, PaintingVariant, PaletteFormat, PalettedContainer, TextComponent, VarInt, WolfVariant};

pub struct LoginProtocol;

impl Plugin for LoginProtocol {
    fn load(&self, server: ServerHandle) {
        server.low_level(|server| {
            server.handshake_event(|packet, conn| {
                let C2SHandshakePackets::Intention(packet) = packet;
                let conn = conn.protocol_handle();
                let stage = packet.intended_stage.into_stage();
                println!("new stage: {:?}", stage);
                match stage {
                    Stage::Status => {
                        conn.set_stage(Stage::Status);
                        println!("Connection is now status phase");
                    }
                    stage => conn.set_stage(stage),
                };
            }).status_event(|packet, connection| {
                match packet {
                    C2SStatusPackets::PingRequest(packet) => {
                        connection.protocol_handle().send_packet(PongResponseS2CStatusPacket {
                            timestamp: packet.timestamp,
                        }).unwrap();
                    }
                    C2SStatusPackets::StatusRequest(_packet) => {
                        connection.protocol_handle().send_packet(
                            StatusResponse {
                                version: StatusResponseVersion {
                                    name: "1.21.1".to_string(),
                                    protocol: 767,
                                },
                                players: StatusResponsePlayers {
                                    online: 0,
                                    max: 0,
                                    sample: vec![],
                                },
                                desc: TextComponent::of_literal("hi").into(),
                                favicon_png_b64: "".to_string(),
                                enforce_chat_reports: false,
                                prevent_chat_reports: false,
                            }
                                .to_packet(),
                        )
                            .unwrap();
                    }
                }
            }).login_event(|packet, connection| {
                let C2SLoginPackets::Hello(packet) = packet else {
                    return;
                };
                let connection = connection.protocol_handle();
    
                let mut props =
                    LengthPrefixHashMap::<VarInt, String, LoginSuccessProperty>::new();
                props.insert(
                    "textures".into(),
                    LoginSuccessProperty {
                        value: "ewogICJ0aW1lc3RhbXAiIDogMTYxMjIxMTAxNDg1MywKICAicHJvZmlsZUlkIiA6ICI1ZWE0ODg2NTg2OWI0Y2ZhOWRjNTg5YmFlZWQwNzM5MCIsCiAgInByb2ZpbGVOYW1lIiA6ICJfUllOMF8iLAogICJzaWduYXR1cmVSZXF1aXJlZCIgOiB0cnVlLAogICJ0ZXh0dXJlcyIgOiB7CiAgICAiU0tJTiIgOiB7CiAgICAgICJ1cmwiIDogImh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvN2NmNDU1YmI4NjcyN2M1NjFlNjI2ZDIxZjA3MGE1OTdmMDlhOTZkOGFhNmMwZmRjM2JjYjZkMDE2NTZjMDk3OCIKICAgIH0KICB9Cn0=".to_string(),
                        sig: Some("SA3W+MXMEWPOwmktk2K8G9kYSb07loa/UOCqBF7PBlvMzGrPb7clNQS/JP2uXU3BxlunguuLPK2bR+Q86neBBSzndSErB8oyJorogi/1y0LOEFVF98Iy0hGrDDCuuT+236SY2L+u05Y/cpN7M/lE4J2YLitx7RzWfqcdxIJE8nCcJcfso1YKEMHzKlkQkxtZOd5+HDfmAlI9qSaK0LpgEFF5DieYMhRvbC6Vl54AXTfTYMZ1QmixmxdBXMSF1sDWzl57Jx79Q6djB/BahMC9aj83rTcyZJaXJS6PqVOULx7YZFs89abVtzrj+pvt3b2SMZoEbjOMsGulXy336NJBuf7mPN+MXz2bnwGbhxYwDrMdSwUjgm+iH9XWwN3piAovenhRyW4vdpXVYf4993gnQBbOVyDFmf/COLt5mezsSNTmCMkoEXrdvz02JjzxmzXasv25rglPSlZFWmStrEMGTHARLtNvKF+SL5LYiHl8rBJrvQDEOSj0fR3eH9o+MSlT5veNjdtDFt2Llc+0tiSqvuM1e3PnE72ALC6cPDludDQI9+YFbX5uV1miB0C0Fe/+DEGe3oVtufP122yobEB1fegWf02BZtCp4Ss8Zm8JOQepXhOvw7QjJFyRckZRHa0GlkBdMYr5GHNe9cTtPEUEAOwrQ86eqo/jk/IFMChiNvY=".to_string()),
                    },
                );
                connection.send_packet(LoginFinishedS2CLoginPacket {
                    uuid: packet.uuid.clone(),
                    username: packet.username.clone(),
                    props,
                }).unwrap();
            }).login_event(|packet, connection| {
                let C2SLoginPackets::LoginAcknowledged(_packet) = packet else {
                    return;
                };
                let connection = connection.protocol_handle();
                connection.set_stage(Stage::Config);
    
                let mut data = ConsumeAllVec::new();
                data.extend("Wyvern-MC".bytes());
                connection.send_packet(CustomPayloadS2CConfigPacket {
                    channel: Identifier::new("minecraft", "branc"),
                    data,
                }).unwrap();
    
                let mut known_packs = LengthPrefixVec::new();
                known_packs.push(KnownPack {
                    namespace: "minecraft".to_string(),
                    id: "core".to_string(),
                    version: "1.21.4".to_string(),
                });
                known_packs.push(KnownPack {
                    namespace: "minecraft".to_string(),
                    id: "vanilla".to_string(),
                    version: "1.21.4".to_string(),
                });
                connection.send_packet(SelectKnownPacksS2CConfigPacket {
                    known_packs: known_packs,
                }).unwrap();
            }).configuration_event(|packet, _connection| {
                println!("config packet: {:?}", packet);
            }).configuration_event(|packet, _connection| {
                let C2SConfigPackets::ClientInformation(_packet) = packet else {
                    return;
                };
            }).configuration_event(|packet, connection| {
                let C2SConfigPackets::SelectKnownPacks(_packet) = packet else {
                    return;
                };

                let connection = connection.protocol_handle();
    
                let mut dim_type_registry = Registry::new();
                dim_type_registry.insert(
                    Identifier::new("minecraft", "oveworld"), 
                    DimType {
                        fixed_time: None,
                        has_skylight: true,
                        has_ceiling: false,
                        ultrawarm: false,
                        natural: true,
                        coordinate_scale: 1.0,
                        bed_works: true,
                        respawn_anchor_works: false,
                        min_y: 0,
                        max_y: 16,
                        logical_height: 16,
                        height: 16,
                        infiniburn: "#minecraft:infiniburn_overworld".to_string(),
                        effects: DimEffects::Nether,
                        ambient_light: 15.0,
                        piglin_safe: false,
                        has_raids: true,
                        monster_spawn_light_level: DimMonsterSpawnLightLevel::Constant(1),
                        monster_spawn_block_light_limit: 1,
                    }
                );
    
                let mut wolf_variant = Registry::new();
                wolf_variant.insert(
                    Identifier::new("minecraft", "pale"), 
                    WolfVariant {
                        wild_texture: Identifier::new("minecraft", "wild_tex"),
                        tame_texture: Identifier::new("minecraft", "tame_tex"),
                        angry_texture: Identifier::new("minecraft", "angry_tex"),
                        biomes: vec![]
                    }
                );
    
                let mut painting_variant = Registry::new();
                painting_variant.insert(
                    Identifier::new("minecraft", "empty_painting"), 
                    PaintingVariant {
                        asset_id: Identifier::new("minecraft", "empty_painting"),
                        width: 1,
                        height: 1,
                        title: TextComponent::of_literal("Empty Painting"),
                        author: TextComponent::of_literal("Endistic")
                    }
                );
    
                connection.send_packet(DamageType::vanilla_registry().to_registry_data_packet()).unwrap();
                connection.send_packet(Biome::vanilla_registry().to_registry_data_packet()).unwrap();
                connection.send_packet(dim_type_registry.to_registry_data_packet()).unwrap();
                connection.send_packet(wolf_variant.to_registry_data_packet()).unwrap();
                connection.send_packet(painting_variant.to_registry_data_packet()).unwrap();
    
                connection.send_packet(FinishConfigurationS2CConfigPacket).unwrap();
            }).configuration_event(|packet, connection| {
                let C2SConfigPackets::FinishConfiguration(_packet) = packet else {
                    return;
                };
    
                let connection = connection.protocol_handle();
                
                connection.set_stage(Stage::Play);
    
                let mut dims = LengthPrefixVec::new();
                dims.push(Identifier::new("minecraft", "overworld"));
                connection.send_packet(LoginS2CPlayPacket {
                    entity: 1,
                    hardcore: false,
                    dims,
                    max_players: VarInt::from(0),
                    view_dist: VarInt::from(8),
                    sim_dist: VarInt::from(8),
                    reduced_debug: false,
                    respawn_screen: false,
                    limited_crafting: false,
                    dim: unsafe { RegEntry::new_unchecked(0) },
                    dim_name: Identifier::new("minecraft", "overworld"),
                    seed: 0,
                    gamemode: Gamemode::Adventure,
                    old_gamemode: Gamemode::None,
                    is_debug: false,
                    is_flat: false,
                    death_loc: None,
                    portal_cooldown: VarInt::from(0),
                    sea_level: VarInt::from(64),
                    enforce_chat_reports: false,
                }).unwrap();
    
                connection.send_packet(GameEventS2CPlayPacket {
                    event: GameEvent::WaitForChunks,
                    value: 0.0,
                }).unwrap();
    
                connection.send_packet(PlayerPositionS2CPlayPacket {
                    teleport_id: VarInt::from(1),
                    x: 8.0,
                    y: 16.0,
                    z: 8.0,
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
                        rotate_velocity: false
                    }
                }).unwrap();
    
                
            }).play_event(|packet, connection| {
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