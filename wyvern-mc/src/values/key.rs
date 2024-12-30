use std::marker::PhantomData;

pub struct Key<T> {
    namespace: String,
    path: String,
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