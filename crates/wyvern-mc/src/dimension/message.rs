use tokio::sync::oneshot::Sender;
use voxidian_protocol::value::DimType;

use crate::values::{Key, Position};

use super::chunk::ChunkSection;

pub enum DimensionMessage {
    GetChunkSection(Position<i32>, Sender<Option<ChunkSection>>),
    GetDimensionType(Sender<Key<DimType>>),
}
