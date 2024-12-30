#[derive(Debug, Clone, Copy)]
pub struct Location {
    x: f64,
    y: f64,
    z: f64,
    pitch: f32,
    yaw: f32,
}

impl Location {
    pub fn new<X: Into<f64>, Y: Into<f64>, Z: Into<f64>, Pitch: Into<f32>, Yaw: Into<f32>>(
        x: X,
        y: Y,
        z: Z,
        pitch: Pitch,
        yaw: Yaw,
    ) -> Location {
        Location {
            x: x.into(),
            y: y.into(),
            z: z.into(),
            pitch: pitch.into(),
            yaw: yaw.into(),
        }
    }
}
