#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPosition {
    pub x: i32,
    pub z: i32
}

impl ChunkPosition {
    pub fn new(x: i32, z: i32) -> ChunkPosition {
        ChunkPosition { x, z }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkSectionPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32
}

impl ChunkSectionPosition {
    pub fn new(x: i32, y: i32, z: i32) -> ChunkSectionPosition {
        ChunkSectionPosition { x, y, z }
    }

    pub fn map_numeric_hash(&self) -> u64 {
        ((self.x as u64 & 0x3FFFFFF) << 38) | ((self.z as u64 & 0x3FFFFFF) << 12) | (self.y as u64 & 0xFFF)
    }
}