use super::{Task, TypeMap};

pub trait TaskParameter: Sized {
    fn fetch(data: &TypeMap) -> Option<Self>;
}

impl TaskParameter for () {
    fn fetch(data: &TypeMap) -> Option<Self> {
        Some(())
    }
}

impl<T1: TaskParameter> TaskParameter for (T1,) {
    fn fetch(data: &TypeMap) -> Option<Self> {
        Some((T1::fetch(data)?,))
    }
}

impl<T1: TaskParameter, T2: TaskParameter> TaskParameter for (T1, T2) {
    fn fetch(data: &TypeMap) -> Option<Self> {
        Some((T1::fetch(data)?, T2::fetch(data)?))
    }
}

impl<T1: TaskParameter, 
    T2: TaskParameter, 
    T3: TaskParameter> TaskParameter for (T1, T2, T3) {
    fn fetch(data: &TypeMap) -> Option<Self> {
        Some((T1::fetch(data)?, T2::fetch(data)?, T3::fetch(data)?))
    }
}

impl<T1: TaskParameter, 
    T2: TaskParameter, 
    T3: TaskParameter,
    T4: TaskParameter> TaskParameter for (T1, T2, T3, T4) {
    fn fetch(data: &TypeMap) -> Option<Self> {
        Some((T1::fetch(data)?, T2::fetch(data)?, T3::fetch(data)?, T4::fetch(data)?))
    }
}