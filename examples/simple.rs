use std::net::SocketAddrV4;
use std::str::FromStr;

use voxidian_protocol::packet::c2s::play::C2SPlayPackets;
use wyvern_mc::{dimension::{BlockState, Dimension}, plugin::Setup, values::{Key, Location}, Server};

pub fn main() {

    Server::new()
        .low_level(|server| {
            server.play_event(|packet, connection| {
                let C2SPlayPackets::AcceptTeleportation(packet) = packet else {
                    return;
                };

                if packet.teleport_id.as_i32() != 1 {
                    return;
                }

                let mut dim = Dimension::new(Key::new("player", "world"));
                for x in -10..10 {
                    for z in -10..10 {
                        dim.set_block(
                            Location::new(x as f64, 1.0, z as f64, 0.0, 0.0), 
                            BlockState::new(Key::new("minecraft", "andesite"))
                        );
                    }
                }

                connection.set_dimension(dim.clone());
                connection.dimension().set_block(
                    Location::new(0.0, 5.0, 6.0, 0.0, 0.0), 
                    BlockState::new(Key::new("minecraft", "oak_planks"))
                );
                connection.teleport(Location::new(0.0, 20.0, 0.0, 0.0, 0.0));
            })
        })
        .add_plugin(Setup)
        
        .start(SocketAddrV4::from_str("127.0.0.1:25565").unwrap());
}
