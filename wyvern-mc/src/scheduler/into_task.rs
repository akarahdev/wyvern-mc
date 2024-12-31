use std::marker::PhantomData;

use super::{functions::FunctionTask, Task};

pub trait IntoTask {
    type TaskType: Task;

    fn into_task(self) -> Self::TaskType;
}

impl<F: FnMut() + Send + Sync + 'static> IntoTask for F {
    type TaskType = FunctionTask<(), F>;

    fn into_task(self) -> Self::TaskType {
        FunctionTask {
            function: self,
            marker: PhantomData
        }
    }
}