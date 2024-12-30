mod location;
pub use location::*;
mod vector;
pub use vector::*;
mod key;
pub use key::*;

pub trait Position {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
}