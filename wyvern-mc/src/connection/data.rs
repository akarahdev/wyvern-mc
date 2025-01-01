
use std::sync::{atomic::{AtomicBool, AtomicI32, AtomicU64}, Mutex};

use crate::{dimension::Dimension, values::Location};

pub(crate) struct PlayerData {
    pub(crate) last_position: Mutex<Location>,
    pub(crate) dimension: Mutex<Option<Dimension>>,

    pub(crate) is_sneaking: AtomicBool,
    pub(crate) is_sprinting: AtomicBool,
    pub(crate) last_teleport_transaction_sent: AtomicI32,
    pub(crate) last_teleport_transaction_received: AtomicI32,
    pub(crate) time_alive: AtomicU64
}

impl Default for PlayerData {
    fn default() -> Self {
        Self { 
            last_position: Mutex::new(Location::new(0.0, 0.0, 0.0, 0.0, 0.0)),
            dimension: Mutex::new(None),

            is_sneaking: AtomicBool::new(false),
            is_sprinting: AtomicBool::new(false),
            last_teleport_transaction_sent: AtomicI32::new(1),
            last_teleport_transaction_received: AtomicI32::new(1),
            time_alive: AtomicU64::new(0)
        }
    }
}