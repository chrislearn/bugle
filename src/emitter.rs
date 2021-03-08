pub trait Emitter<D> {
    fn emit(&mut self, data: &D);
}