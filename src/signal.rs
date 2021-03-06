use super::{Sender, Receiver};

pub struct Signal<D> {
    receivers: Vec<Box<dyn Receiver<D>>>,
}

impl<D> Signal<D> {
    pub fn new() -> Self {
        Signal {
            receivers: Vec::default(),
        }
    }
    pub fn push<R>(&mut self, receiver: R) where R: Receiver<D> + Send + 'static{
        self.receivers.push(Box::new(receiver));
    }
}
impl<D> Sender<D> for Signal<D> {
    fn send(&self, data: &D) {
        for receiver in &self.receivers {
            receiver.receive(data)
        }
    }
}