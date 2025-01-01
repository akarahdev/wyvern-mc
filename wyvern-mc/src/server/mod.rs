mod handle;
pub use handle::*;
mod protocol;
pub use protocol::*;
mod builder;
pub use builder::*;

use voxidian_protocol::value::BlockState;

use crate::{dimension::Dimension, scheduler::StoredTask, values::Key, Player};
use std::{cell::OnceCell, collections::HashMap, sync::{mpsc::Receiver, Arc, Mutex, OnceLock}};

pub static SERVER_INSTANCE: OnceLock<Server> = OnceLock::new();

#[derive(Default)]
pub struct ServerData {
    pub(crate) connections: Vec<Player>,

    pub(crate) handshake_events: Vec<HandshakeEvent>,
    pub(crate) status_events: Vec<StatusEvent>,
    pub(crate) login_events: Vec<LoginEvent>,
    pub(crate) config_events: Vec<ConfigEvent>,
    pub(crate) play_events: Vec<PlayEvent>,

    pub(crate) task_receiver: OnceCell<Receiver<StoredTask>>,

    pub(crate) dimensions: HashMap<Key<Dimension>, Dimension>
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
        ServerBuilder { server, persistent_tasks: Vec::new() }
    }
}

impl ServerData {
    pub fn task_receiver(&self) -> &Receiver<StoredTask> {
        self.task_receiver.get().unwrap()
    }
}