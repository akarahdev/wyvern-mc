use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Key<T> {
    pub(crate) namespace: String,
    pub(crate) path: String,
    _phantom: PhantomData<T>
}

impl<T> Key<T> {
    pub fn new<N: Into<String>, P: Into<String>>(namespace: N, path: P) -> Key<T> {
        Key {
            namespace: namespace.into(),
            path: path.into(),
            _phantom: PhantomData
        }
    }
}