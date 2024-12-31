use voxidian_protocol::packet::{Stage, c2s::handshake::C2SHandshakePackets};

use crate::plugin::Plugin;

pub struct HandshakePlugin;

impl Plugin for HandshakePlugin {
    fn load(&self, server: crate::Server) {
        server.low_level(|server| {
            server.handshake_event(|packet, conn| {
                let C2SHandshakePackets::Intention(packet) = packet;
                let conn = conn.raw_handle();
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
