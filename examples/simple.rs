use std::net::SocketAddrV4;
use std::str::FromStr;

use wyvern_mc::{Server, plugin::Setup};

pub fn main() {
    Server::new()
        .add_plugin(Setup)
        .start(SocketAddrV4::from_str("127.0.0.1:25565").unwrap());
}
