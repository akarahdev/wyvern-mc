use super::{functions::FunctionTask, parameters::TaskParameter};

pub type StoredTask = Box<dyn Task>;

pub trait Task: Send + Sync + 'static {
    fn run(&mut self);
}

impl<F: FnMut() + Send + Sync + 'static> Task for FunctionTask<(), F> {
    fn run(&mut self) {
        (self.function)()
    }
}

impl<F: FnMut(T1) + Send + Sync + 'static, 
    T1: TaskParameter + 'static> 
    Task for FunctionTask<(T1,), F> {
    fn run(&mut self) {
        (self.function)(T1::fetch())
    }
}

impl<F: FnMut(T1, T2) + Send + Sync + 'static, 
    T1: TaskParameter + 'static,
    T2: TaskParameter + 'static> 
    Task for FunctionTask<(T1, T2), F> {
        fn run(&mut self) {
            (self.function)(T1::fetch(), T2::fetch())
        }
}