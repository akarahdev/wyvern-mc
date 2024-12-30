use std::sync::{Arc, Mutex};

use crate::values::Key;

use super::DimensionData;

#[derive(Clone)]
pub struct DimensionHandle {
    inner: Arc<Mutex<DimensionData>>
}

impl DimensionHandle {
    pub fn name(&self) -> Key<DimensionHandle> {
        self.inner.lock().unwrap().name.clone()
    }
}