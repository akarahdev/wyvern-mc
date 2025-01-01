use std::net::SocketAddrV4;
use std::str::FromStr;

use wyvern_mc::{dimension::{BlockState, Dimension}, plugin::Setup, scheduler::{ConnectEvent, Event, MoveEvent, Param, PlayerTickEvent}, values::{BlockPosition, Key, Location}, Player, Server};

pub fn main() {
    let mut server = Server::new();
    server.add_system(on_connect);
    server.add_system(on_tick);
    server.start(SocketAddrV4::from_str("127.0.0.1:25565").unwrap());
}

fn on_tick(
    _event: Event<PlayerTickEvent>,
    player: Param<Player>
) {
    if player.location().y < 0.0 {
        player.teleport(Location::new(0.0, 1.0, 0.0, 0.0, 0.0));
    }
}

fn on_connect(
    _event: Event<ConnectEvent>,
    player: Param<Player>
) {
    let mut dim = Dimension::new(Key::new("wyvern", "world"));
    let wool = BlockState::new(Key::new("minecraft", "yellow_wool"));
    dim.set_block(
        BlockPosition::new(0, 0, 0), 
        wool.clone()
    );

    dim.set_block(
        BlockPosition::new(2, 1, 3), 
        wool.clone()
    );

    dim.set_block(
        BlockPosition::new(3, 2,  7), 
        wool.clone()
    );

    player.set_dimension(dim);
    player.teleport(Location::new(0.0, 1.0, 0.0, 0.0, 0.0));
}