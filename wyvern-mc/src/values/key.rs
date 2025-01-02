use std::marker::PhantomData;

#[derive(Debug, Clone, Hash, Eq)]
pub struct Key<T> {
    pub(crate) namespace: String,
    pub(crate) path: String,
    _phantom: PhantomData<T>
}

impl<T> PartialEq for Key<T> {
    fn eq(&self, other: &Self) -> bool {
        self.namespace == other.namespace && self.path == other.path && self._phantom == other._phantom
    }
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