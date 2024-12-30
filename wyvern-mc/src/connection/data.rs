use crate::values::Location;

pub(crate) struct PlayerData {
    pub(crate) last_position: Location
}

impl Default for PlayerData {
    fn default() -> Self {
        Self { 
            last_position: Location::new(0.0, 0.0, 0.0, 0.0, 0.0)
        }
    }
}