mod handle;
pub use handle::*;
mod block_state;
pub use block_state::*;
mod chunk;
pub use chunk::*;

use std::collections::HashMap;

use crate::values::{Key, BlockPosition};

pub struct DimensionRegistry {
    dimensions: HashMap<Key<Dimension>, Dimension>
}

pub struct DimensionData {
    name: Key<Dimension>,
    chunk_sections: HashMap<BlockPosition, ChunkSection>,

    min_y: i32,
    max_y: i32
}
