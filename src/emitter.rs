pub trait Emitter<D> {
    fn emit(&self, data: &D);
}