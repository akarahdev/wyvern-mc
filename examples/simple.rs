use std::{net::SocketAddrV4, sync::LazyLock};
use std::str::FromStr;

use wyvern_mc::{dimension::{BlockState, Dimension}, scheduler::{ConnectEvent, Event, Param, PlayerTickEvent, ServerStartEvent}, values::{BlockPosition, Key, Location}, Player, Server};

static DIMENSION: LazyLock<Dimension> = LazyLock::new(|| Dimension::new(Key::new("wyvern", "global")));

pub fn main() {
    let mut server = Server::new();
    server.add_system(on_start);
    server.add_system(on_connect);
    server.add_system(on_tick);
    server.start(SocketAddrV4::from_str("127.0.0.1:25565").unwrap());
}

fn on_start(
    _event: Event<ServerStartEvent>
) {

    println!("Server started!");

    let wool = BlockState::new(Key::new("minecraft", "yellow_wool"));
    DIMENSION.set_block(
        BlockPosition::new(0, 0, 0), 
        wool.clone()
    );
    DIMENSION.set_block(
        BlockPosition::new(2, 1, 3), 
        wool.clone()
    );
    DIMENSION.set_block(
        BlockPosition::new(3, 2,  7), 
        wool.clone()
    );
    DIMENSION.set_block(
        BlockPosition::new(6, 3,  9), 
        wool.clone()
    );
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
    player.set_dimension(DIMENSION.clone());
    player.teleport(Location::new(0.0, 1.0, 0.0, 0.0, 0.0));
}