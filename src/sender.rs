pub trait Sender<D> {
    fn send(&self, data: &D);
}