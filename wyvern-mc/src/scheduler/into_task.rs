use std::marker::PhantomData;

use super::{functions::FunctionTask, parameters::TaskParameter, Task};

pub trait IntoTask<Input> {
    type TaskType: Task;

    fn into_task(self) -> Self::TaskType;
}

impl<F: Fn() + Send + Sync + 'static> IntoTask<()> for F {
    type TaskType = FunctionTask<(), F>;

    fn into_task(self) -> Self::TaskType {
        FunctionTask {
            function: self,
            marker: PhantomData
        }
    }
}

impl<
    F: Fn(T1) + Send + Sync + 'static,
    T1: TaskParameter + 'static> IntoTask<(T1,)> for F {
    type TaskType = FunctionTask<(T1,), F>;

    fn into_task(self) -> Self::TaskType {
        FunctionTask {
            function: self,
            marker: PhantomData
        }
    }
}

impl<
    F: Fn(T1, T2) + Send + Sync + 'static,
    T1: TaskParameter + 'static,
    T2: TaskParameter + 'static> IntoTask<(T1, T2)> for F {
    type TaskType = FunctionTask<(T1, T2), F>;

    fn into_task(self) -> Self::TaskType {
        FunctionTask {
            function: self,
            marker: PhantomData
        }
    }
}

impl<
    F: Fn(T1, T2, T3) + Send + Sync + 'static,
    T1: TaskParameter + 'static,
    T2: TaskParameter + 'static,
    T3: TaskParameter + 'static> IntoTask<(T1, T2, T3)> for F {
    type TaskType = FunctionTask<(T1, T2, T3), F>;

    fn into_task(self) -> Self::TaskType {
        FunctionTask {
            function: self,
            marker: PhantomData
        }
    }
}