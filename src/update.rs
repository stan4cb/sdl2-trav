pub trait Updateble<T> {
    fn update(&mut self) {}
    fn update_with(&mut self, _: T) {}
}
