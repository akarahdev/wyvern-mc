use std::marker::PhantomData;

#[derive(Clone)]
pub struct FunctionTask<Input, F: Send + Sync + 'static> {
    pub(crate) function: F,
    pub(crate) marker: PhantomData<fn() -> Input>
}