use voxidian_protocol::{
    packet::{
        c2s::status::C2SStatusPackets,
        s2c::status::{
            PongResponseS2CStatusPacket, StatusResponse, StatusResponsePlayers,
            StatusResponseVersion,
        },
    },
    value::TextComponent,
};

use crate::plugin::Plugin;

pub struct StatusPlugin;

impl Plugin for StatusPlugin {
    fn load(&self, server: crate::Server) {
        server.low_level(|server| {
            server.status_event(|packet, connection| match packet {
                C2SStatusPackets::PingRequest(packet) => {
                    connection
                        .protocol_handle()
                        .send_packet(PongResponseS2CStatusPacket {
                            timestamp: packet.timestamp,
                        })
                        .unwrap();
                }
                C2SStatusPackets::StatusRequest(_packet) => {
                    connection
                        .protocol_handle()
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
                                desc: TextComponent::of_literal("hi").into(),
                                favicon_png_b64: "".to_string(),
                                enforce_chat_reports: false,
                                prevent_chat_reports: false,
                            }
                            .to_packet(),
                        )
                        .unwrap();
                }
            })
        });
    }
}
