use super::Position;

#[derive(Debug, Clone, Copy)]
pub struct Location {
    x: f64,
    y: f64,
    z: f64,
    pitch: f32,
    yaw: f32,
}

impl Position for Location {
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

impl Location {
    pub fn new(x: f64, y: f64, z: f64, pitch: f32, yaw: f32) -> Location {
        Location {
            x,
            y,
            z,
            pitch,
            yaw,
        }
    }

    pub fn shift_by<P: Position>(&self, other: P) -> Location {
        Location {
            x: self.x + other.x(),
            y: self.y + other.y(),
            z: self.z + other.z(),
            pitch: self.pitch,
            yaw: self.yaw
        }
    }

    pub fn center(&self) -> Location {
        Location {
            x: self.x.floor() + 0.5,
            y: self.y.floor() + 0.5,
            z: self.z.floor() + 0.5,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}
