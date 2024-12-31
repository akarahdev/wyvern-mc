use voxidian_protocol::value::BlockPos;

use super::Position;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPosition {
    x: i32,
    y: i32,
    z: i32
}

impl Position for BlockPosition {
    fn x(&self) -> f64 {
        self.x as f64
    }

    fn y(&self) -> f64 {
        self.y as f64
    }

    fn z(&self) -> f64 {
        self.z as f64
    }
}

impl BlockPosition {
    pub fn new(x: i32, y: i32, z: i32) -> BlockPosition {
        BlockPosition { x, y, z }
    }

    pub fn from_position<P: Position>(p: P) -> BlockPosition {
        BlockPosition { x: p.x().floor() as i32, y: p.y().floor() as i32, z: p.z().floor() as i32 }
    }
}