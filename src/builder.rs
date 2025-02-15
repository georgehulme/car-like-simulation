pub trait Builder<T> {
    fn create(self) -> T;
}
