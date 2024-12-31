mod handle;

pub use handle::*;
mod block_state;
pub use block_state::*;
mod chunk;
pub use chunk::*;
use nohash_hasher::{BuildNoHashHasher, NoHashHasher};

use std::collections::HashMap;

use crate::values::{ChunkSectionPosition, Key};

pub struct DimensionData {
    name: Key<Dimension>,
    chunk_sections: HashMap<u64, ChunkSection, BuildNoHashHasher<u64>>,

    min_y: i32,
    max_y: i32
}
