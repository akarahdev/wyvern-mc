mod handle;
use std::collections::HashMap;

pub use handle::*;

use crate::values::{Key, Vector};

pub struct DimensionRegistry {
    dimensions: HashMap<Key<DimensionHandle>, DimensionHandle>
}


pub struct DimensionData {
    name: Key<DimensionHandle>,
    blocks: HashMap<Vector, () /* todo: block state value */>
}