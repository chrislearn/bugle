pub trait Receiver<D> {
    fn receive(&self, data: &D);
}

impl<F, D> Receiver<D> for F where F: Fn(&D){
    fn receive(&self, data: &D) {
        self(data);
    }
}