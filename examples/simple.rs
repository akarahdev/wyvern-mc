use std::{net::SocketAddrV4, time::Instant};
use std::str::FromStr;

use voxidian_protocol::packet::c2s::play::C2SPlayPackets;
use wyvern_mc::{dimension::{BlockState, Dimension}, plugin::Setup, values::{BlockPosition, Key, Location}, Server};

pub fn main() {
    Server::new()
        .low_level(|server| {
            server.play_event(|packet, player| {
                let C2SPlayPackets::AcceptTeleportation(packet) = packet else {
                    return;
                };

                if packet.teleport_id.as_i32() != 1 {
                    return;
                }

                
                let mut dim = Dimension::new(Key::new("player", "world"));
                let block = BlockState::new(Key::new("minecraft", "andesite"));
                
                for x in -50..50 {
                    for z in -50..50 {
                        
                        dim.set_block(
                            BlockPosition::new(x, 1, z), 
                            block.clone()
                        );
                        
                    }
                }
                
                

                

                player.set_dimension(dim.clone());
                player.teleport(Location::new(0.0, 20.0, 0.0, 0.0, 0.0));
            })
        })
        .add_plugin(Setup)
        .start(SocketAddrV4::from_str("127.0.0.1:25565").unwrap());
}
