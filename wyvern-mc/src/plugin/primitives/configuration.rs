use voxidian_protocol::{
    packet::{
        Stage,
        c2s::config::C2SConfigPackets,
        s2c::{
            config::FinishConfigurationS2CConfigPacket,
            play::{
                GameEvent, GameEventS2CPlayPacket, Gamemode, LoginS2CPlayPacket,
                PlayerPositionS2CPlayPacket, TeleportFlags,
            },
        },
    },
    registry::{RegEntry, Registry},
    value::{
        Biome, DamageType, DimEffects, DimMonsterSpawnLightLevel, DimType, Identifier,
        LengthPrefixVec, PaintingVariant, TextComponent, VarInt, WolfVariant,
    },
};

use crate::{plugin::Plugin, ServerBuilder};

pub struct ConfigurationPlugin;

impl Plugin for ConfigurationPlugin {
    fn load(&self, server: &mut ServerBuilder) {
        server.low_level(|server| {
            server.configuration_event(|packet, _connection| {
                println!("config packet: {:?}", packet);
            });
            server.configuration_event(|packet, _connection| {
                let C2SConfigPackets::ClientInformation(_packet) = packet else {
                    return;
                };
            });
            server.configuration_event(|packet, connection| {
                let C2SConfigPackets::SelectKnownPacks(_packet) = packet else {
                    return;
                };

                let connection = connection.raw_handle();

                let mut dim_type_registry = Registry::new();
                dim_type_registry.insert(Identifier::new("minecraft", "oveworld"), DimType {
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
                    effects: DimEffects::Overworld,
                    ambient_light: 15.0,
                    piglin_safe: false,
                    has_raids: true,
                    monster_spawn_light_level: DimMonsterSpawnLightLevel::Constant(1),
                    monster_spawn_block_light_limit: 1,
                });
            

                let mut wolf_variant = Registry::new();
                wolf_variant.insert(Identifier::new("minecraft", "pale"), WolfVariant {
                    wild_texture: Identifier::new("minecraft", "wild_tex"),
                    tame_texture: Identifier::new("minecraft", "tame_tex"),
                    angry_texture: Identifier::new("minecraft", "angry_tex"),
                    biomes: vec![],
                });

                let mut painting_variant = Registry::new();
                painting_variant.insert(
                    Identifier::new("minecraft", "empty_painting"),
                    PaintingVariant {
                        asset_id: Identifier::new("minecraft", "empty_painting"),
                        width: 1,
                        height: 1,
                        title: TextComponent::of_literal("Empty Painting"),
                        author: TextComponent::of_literal("Endistic"),
                    },
                );

                connection
                    .send_packet(DamageType::vanilla_registry().to_registry_data_packet())
                    .unwrap();
                connection
                    .send_packet(Biome::vanilla_registry().to_registry_data_packet())
                    .unwrap();
                connection
                    .send_packet(dim_type_registry.to_registry_data_packet())
                    .unwrap();
                connection
                    .send_packet(wolf_variant.to_registry_data_packet())
                    .unwrap();
                connection
                    .send_packet(painting_variant.to_registry_data_packet())
                    .unwrap();

                connection
                    .send_packet(FinishConfigurationS2CConfigPacket)
                    .unwrap();
            });
        server.configuration_event(|packet, connection| {
                let C2SConfigPackets::FinishConfiguration(_packet) = packet else {
                    return;
                };

                let connection = connection.raw_handle();

                connection.set_stage(Stage::Play);

                let mut dims = LengthPrefixVec::new();
                dims.push(Identifier::new("minecraft", "overworld"));
                connection
                    .send_packet(LoginS2CPlayPacket {
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
                        gamemode: Gamemode::Creative,
                        old_gamemode: Gamemode::None,
                        is_debug: false,
                        is_flat: false,
                        death_loc: None,
                        portal_cooldown: VarInt::from(0),
                        sea_level: VarInt::from(64),
                        enforce_chat_reports: false,
                    })
                    .unwrap();

                connection
                    .send_packet(GameEventS2CPlayPacket {
                        event: GameEvent::WaitForChunks,
                        value: 0.0,
                    })
                    .unwrap();

                connection
                    .send_packet(PlayerPositionS2CPlayPacket {
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
                            rotate_velocity: false,
                        },
                    })
                    .unwrap();
            });
        });
    }
}
