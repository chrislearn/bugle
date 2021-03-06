pub trait Receiver<D> {
    fn call(&self, data: &D);
}

impl<F, D> Receiver<D> for F where F: Fn(&D){
    fn call(&self, data: &D) {
        self(data);
    }
}