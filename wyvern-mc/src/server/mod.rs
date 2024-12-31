mod handle;
pub use handle::*;
mod protocol;
pub use protocol::*;
mod builder;
pub use builder::*;

use voxidian_protocol::value::BlockState;

use crate::Player;
use std::sync::{Arc, Mutex, OnceLock};

pub static SERVER_INSTANCE: OnceLock<Server> = OnceLock::new();

#[derive(Default)]
pub struct ServerData {
    connections: Vec<Player>,

    handshake_events: Vec<HandshakeEvent>,
    status_events: Vec<StatusEvent>,
    login_events: Vec<LoginEvent>,
    config_events: Vec<ConfigEvent>,
    play_events: Vec<PlayEvent>,
}

impl Server {
    pub fn get() -> Server {
        SERVER_INSTANCE.get().unwrap().clone()
    }


    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ServerBuilder {
        // cache block state registry ahead of time
        BlockState::from_id(0).unwrap();
        
        let server = Server {
            inner: Arc::new(Mutex::new(ServerData::default())),
        };
        let _ = SERVER_INSTANCE.set(server.clone());
        ServerBuilder { server }
    }
}