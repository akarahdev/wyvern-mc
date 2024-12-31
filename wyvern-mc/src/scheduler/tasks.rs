use super::functions::FunctionTask;

pub type StoredTask = Box<dyn Task>;

pub trait Task: Send + Sync + 'static {
    fn run(&mut self);
}

impl<F: FnMut() + Send + Sync + 'static> Task for FunctionTask<(), F> {
    fn run(&mut self) {
        (self.function)()
    }
}