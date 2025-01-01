use super::{EventParameter, Param, TypeMap};

pub trait TaskParameter: Sized {
    fn fetch(data: &TypeMap) -> Option<Self>;
}

impl TaskParameter for () {
    fn fetch(data: &TypeMap) -> Option<Self> {
        Some(())
    }
}

impl<A: TaskParameter> TaskParameter for (A,) {
    fn fetch(data: &TypeMap) -> Option<Self> {
        Some((A::fetch(data)?,))
    }
}

impl<A: TaskParameter, B: TaskParameter> TaskParameter for (A, B) {
    fn fetch(data: &TypeMap) -> Option<Self> {
        Some((A::fetch(data)?, B::fetch(data)?))
    }
}