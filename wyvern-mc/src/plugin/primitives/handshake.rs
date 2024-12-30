use voxidian_protocol::packet::{c2s::handshake::C2SHandshakePackets, Stage};

use crate::plugin::Plugin;

pub struct HandshakePlugin;

impl Plugin for HandshakePlugin {
    fn load(&self, server: crate::ServerHandle) {
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
            })
        });
    }
}