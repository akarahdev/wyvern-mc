use std::net::SocketAddrV4;
use std::str::FromStr;

use voxidian_protocol::{packet::c2s::play::C2SPlayPackets, value::BlockPos};
use wyvern_mc::{dimension::{BlockState, Dimension}, plugin::Setup, scheduler::{Event, MoveEvent, Param, Scheduler}, values::{BlockPosition, Key, Location}, Player, Server};

pub fn main() {
    let mut server = Server::new();
    server.add_system(on_loop);
    server.add_system(on_move);
    server.low_level(|server| {
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
        });
    });
    server.add_plugin(Setup);
    server.start(SocketAddrV4::from_str("127.0.0.1:25565").unwrap());
}

fn on_loop() {
    println!("Looping!");
}

fn on_move(
    _event: Event<MoveEvent>,
    new_location: Param<Location>
) {
    println!("A player moved to {:?}", new_location.center());
}