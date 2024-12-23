use crate::plugin::Plugin;
use crate::ServerHandle;
use voxidian_protocol::packet::c2s::handshake::C2SHandshakePackets;
use voxidian_protocol::packet::c2s::login::C2SLoginPackets;
use voxidian_protocol::packet::c2s::status::C2SStatusPackets;
use voxidian_protocol::packet::s2c::login::{LoginFinishedS2CLoginPacket, LoginSuccessProperty};
use voxidian_protocol::packet::s2c::status::{PongResponseS2CStatusPacket, StatusResponse, StatusResponsePlayers, StatusResponseVersion};
use voxidian_protocol::packet::Stage;
use voxidian_protocol::value::{LengthPrefixHashMap, TextComponent, VarInt};

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
                C2SStatusPackets::StatusRequest(packet) => {
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
            match packet {
                C2SLoginPackets::Hello(packet) => {
                    let mut props =
                        LengthPrefixHashMap::<VarInt, String, LoginSuccessProperty>::new();
                    connection.send_packet(LoginFinishedS2CLoginPacket {
                        uuid: packet.uuid.clone(),
                        username: packet.username.clone(),
                        props,
                    }).unwrap();
                }
                C2SLoginPackets::CookieResponse(packet) => {}
                C2SLoginPackets::CustomQueryAnswer(packet) => {}
                C2SLoginPackets::LoginAcknowledged(packet) => {}
                C2SLoginPackets::Key(packet) => {}
            }
        });
    }
}
