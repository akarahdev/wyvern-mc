pub trait TaskParameter {
    fn fetch() -> Self;
}

impl TaskParameter for () {
    fn fetch() -> Self {
        ()
    }
}

impl<A: TaskParameter, B: TaskParameter> TaskParameter for (A, B) {
    fn fetch() -> Self {
        (A::fetch(), B::fetch())
    }
}