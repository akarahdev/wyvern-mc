use super::{ChunkPosition, ChunkSectionPosition};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl BlockPosition {
    pub fn new(x: i32, y: i32, z: i32) -> BlockPosition {
        BlockPosition { x, y, z }
    }

    pub fn to_chunk_pos(&self) -> ChunkPosition {
        ChunkPosition {
            x: self.x / 16,
            z: self.z / 16
        }
    }

    pub fn to_chunk_section_pos(&self) -> ChunkSectionPosition {
        ChunkSectionPosition {
            x: self.x / 16,
            y: self.y / 16,
            z: self.z / 16
        }
    }
}