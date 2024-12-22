extern crate wyvern_mc;

use std::net::SocketAddrV4;
use std::str::FromStr;
use wyvern_mc::login::LoginProtocol;
use wyvern_mc::Server;

pub fn main() {
    Server::new()
        .add_plugin(LoginProtocol)
        .start(SocketAddrV4::from_str("127.0.0.1:25565").unwrap());
}