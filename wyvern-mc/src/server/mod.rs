mod handle;
pub use handle::*;

mod protocol;
pub use protocol::*;

use crate::ConnectionHandle;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct Server {
    connections: Vec<ConnectionHandle>,

    handshake_events: Vec<HandshakeEvent>,
    status_events: Vec<StatusEvent>,
    login_events: Vec<LoginEvent>,
    config_events: Vec<ConfigEvent>,
    play_events: Vec<PlayEvent>,
}

impl Server {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ServerHandle {
        ServerHandle {
            inner: Arc::new(Mutex::new(Server::default())),
        }
    }
}
