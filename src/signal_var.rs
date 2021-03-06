use std:sync::Weak;


pub struct SignalVar<A> {
    value: A,
    receivers: Vec<Box<dyn Receiver<D>>>,
}

impl<D> SignalVar<D> {
    pub fn new(value: A) -> Self {
        SignalVar {
            value
            receivers: Vec::default(),
        }
    }
    pub fn push<R>(&mut self, receiver: R) where R: Receiver<D> + Send + 'static{
        self.receivers.push(Box::new(receiver));
    }
}
impl<D> Sender<D> for SignalVar<D> {
    fn send(&self, data: &D) {
        for receiver in &self.receivers {
            receiver.receive(data)
        }
    }
}