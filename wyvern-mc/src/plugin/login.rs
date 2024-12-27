use crate::plugin::Plugin;
use crate::ServerHandle;
use voxidian_protocol::packet::c2s::config::C2SConfigPackets;
use voxidian_protocol::packet::c2s::handshake::C2SHandshakePackets;
use voxidian_protocol::packet::c2s::login::C2SLoginPackets;
use voxidian_protocol::packet::c2s::status::C2SStatusPackets;
use voxidian_protocol::packet::s2c::config::{CustomPayloadS2CConfigPacket, FinishConfigurationS2CConfigPacket, KnownPack, SelectKnownPacksS2CConfigPacket};
use voxidian_protocol::packet::s2c::login::{LoginFinishedS2CLoginPacket, LoginSuccessProperty};
use voxidian_protocol::packet::s2c::play::{PlayerPositionS2CPlayPacket, TeleportFlags};
use voxidian_protocol::packet::s2c::status::{PongResponseS2CStatusPacket, StatusResponse, StatusResponsePlayers, StatusResponseVersion};
use voxidian_protocol::packet::Stage;
use voxidian_protocol::value::{ConsumeAllVec, Identifier, LengthPrefixHashMap, LengthPrefixVec, TextComponent, VarInt};

pub struct LoginProtocol;

impl Plugin for LoginProtocol {
    fn load(&self, server: ServerHandle) {
        server.handshake_event(|packet, conn| {
            let C2SHandshakePackets::Intention(packet) = packet;
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
                    connection.send_packet(PongResponseS2CStatusPacket {
                        timestamp: packet.timestamp,
                    }).unwrap();
                }
                C2SStatusPackets::StatusRequest(_packet) => {
                    connection.send_packet(
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

            let mut props =
                LengthPrefixHashMap::<VarInt, String, LoginSuccessProperty>::new();
            props.insert(
                "textures".into(),
                LoginSuccessProperty {
                    value: "ewogICJ0aW1lc3RhbXAiIDogMTYxMjIxMTAxNDg1MywKICAicHJvZmlsZUlkIiA6sICI1ZWE0ODg2NTg2OWI0Y2ZhOWRjNTg5YmFlZWQwNzM5MCIsCiAgInByb2ZpbGVOYW1lIiA6ICJfUllOMF8iLAogICJzaWduYXR1cmVSZXF1aXJlZCIgOiB0cnVlLAogICJ0ZXh0dXJlcyIgOiB7CiAgICAiU0tJTiIgOiB7CiAgICAgICJ1cmwiIDogImh0dHA6Ly90ZXh0dXJlcy5taW5lY3JhZnQubmV0L3RleHR1cmUvN2NmNDU1YmI4NjcyN2M1NjFlNjI2ZDIxZjA3MGE1OTdmMDlhOTZkOGFhNmMwZmRjM2JjYjZkMDE2NTZjMDk3OCIKICAgIH0KICB9Cn0".to_string(),
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

            connection.send_packet(FinishConfigurationS2CConfigPacket).unwrap();
        }).configuration_event(|packet, connection| {
            let C2SConfigPackets::FinishConfiguration(_packet) = packet else {
                return;
            };

            connection.set_stage(Stage::Play);

            connection.send_packet(PlayerPositionS2CPlayPacket {
                teleport_id: VarInt::from(1),
                x: 0.0,
                y: 64.0,
                z: 0.0,
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
        });
    }
}