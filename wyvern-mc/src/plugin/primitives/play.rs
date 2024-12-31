use voxidian_protocol::{
    packet::c2s::play::C2SPlayPackets,
    value::VarInt,
};

use crate::{plugin::Plugin, values::ChunkPosition, ServerBuilder};

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn load(&self, server: &mut ServerBuilder) {
        server.low_level(|server| {
            server.play_event(|packet, _connection| {
                    println!("Play Packet: {:?}", packet);
                });
            server.play_event(|packet, connection| {
                    let C2SPlayPackets::AcceptTeleportation(packet) = packet else {
                        return;
                    };

                    if packet.teleport_id != VarInt::from(1) {
                        return;
                    }

                    let dim = connection.dimension();
                    for chunk_x in -2..2 {
                        for chunk_z in -2..2 {
                            let p = dim.get_chunk_as_packets(ChunkPosition::new(chunk_x, chunk_z));
                            connection.raw_handle().send_packet(p).unwrap();
                        }
                    }
                });
        });
    }
}
