use crate::plugin::Plugin;
use crate::ServerHandle;
use voxidian_protocol::packet::c2s::handshake::C2SHandshakePackets;
use voxidian_protocol::packet::c2s::status::C2SStatusPackets;
use voxidian_protocol::packet::s2c::status::{StatusResponse, StatusResponsePlayers, StatusResponseVersion};
use voxidian_protocol::packet::Stage;

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
        }).status_event(|packet, connection| match packet {
            C2SStatusPackets::PingRequest(packet) => {}
            C2SStatusPackets::StatusRequest(packet) => {
                connection
                    .send_packet(
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
                            desc: Default::default(),
                            favicon_png_b64: "".to_string(),
                            enforce_chat_reports: false,
                            prevent_chat_reports: false,
                        }
                            .to_packet(),
                    )
                    .unwrap();
            }
        });
    }
}
