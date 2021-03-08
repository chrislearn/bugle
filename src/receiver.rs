pub trait Receiver<D> {
    fn receive(&mut self, data: &D);
}

// impl< F, D> Receiver< D> for F where F: FnMut(&D){
//     fn receive(&mut self, data: &D) {
//         self(data);
//     }
// }