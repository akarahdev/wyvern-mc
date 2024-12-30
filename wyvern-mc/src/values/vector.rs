use super::Position;

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Position for Vector {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn z(&self) -> f64 {
        self.z
    }
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }
}
