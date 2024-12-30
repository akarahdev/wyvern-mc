use std::sync::{Arc, Mutex};

use crate::values::Key;

use super::DimensionData;

#[derive(Clone)]
pub struct Dimension {
    inner: Arc<Mutex<DimensionData>>
}

impl Dimension {
    pub fn name(&self) -> Key<Dimension> {
        self.inner.lock().unwrap().name.clone()
    }
}