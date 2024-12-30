mod handle;
use std::collections::HashMap;

pub use handle::*;

use crate::values::{Key, Vector};

pub struct DimensionRegistry {
    dimensions: HashMap<Key<Dimension>, Dimension>
}


pub struct DimensionData {
    name: Key<Dimension>,
    blocks: HashMap<Vector, () /* todo: block state value */>
}