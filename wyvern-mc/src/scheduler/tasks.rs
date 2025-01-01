use super::{functions::FunctionTask, parameters::TaskParameter, TypeMap};

pub type StoredTask = Box<dyn Task>;

pub trait Task: Send + Sync + 'static {
    fn run(&mut self, data: &TypeMap) -> Option<()>;
}

impl<F: Fn() + Send + Sync + 'static> Task for FunctionTask<(), F> {
    fn run(&mut self, _data: &TypeMap) -> Option<()> {
        (self.function)();
        Some(())
    }
}

impl<F: Fn(T1) + Send + Sync + 'static, 
    T1: TaskParameter + 'static> 
    Task for FunctionTask<(T1,), F> {
    fn run(&mut self, data: &TypeMap) -> Option<()> {
        (self.function)(T1::fetch(data)?);
        Some(())
    }
}

impl<F: Fn(T1, T2) + Send + Sync + 'static, 
    T1: TaskParameter + 'static,
    T2: TaskParameter + 'static> 
    Task for FunctionTask<(T1, T2), F> {
        fn run(&mut self, data: &TypeMap) -> Option<()> {
            (self.function)(T1::fetch(data)?, T2::fetch(data)?);
            Some(())
        }
}