
use crate::{dimension::Dimension, values::Location};

pub(crate) struct PlayerData {
    pub(crate) last_position: Location,
    pub(crate) dimension: Option<Dimension>
}

impl Default for PlayerData {
    fn default() -> Self {
        Self { 
            last_position: Location::new(0.0, 0.0, 0.0, 0.0, 0.0),
            dimension: None
        }
    }
}