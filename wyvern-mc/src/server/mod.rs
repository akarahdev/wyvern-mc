mod handle;
pub use handle::*;

mod protocol;
pub use protocol::*;

use crate::Connection;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct ServerData {
    connections: Vec<Connection>,

    handshake_events: Vec<HandshakeEvent>,
    status_events: Vec<StatusEvent>,
    login_events: Vec<LoginEvent>,
    config_events: Vec<ConfigEvent>,
    play_events: Vec<PlayEvent>,
}

impl Server {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Server {
        Server {
            inner: Arc::new(Mutex::new(ServerData::default())),
        }
    }
}
