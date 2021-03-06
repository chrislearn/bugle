use super::{Signal, Receiver};

pub struct SignalFn<F, D> {
    func: F,
    receivers: Vec<Box<dyn Receiver<D>>>,
}

impl<F, D> SignalFn<D> where F: Fn(&D), D: Receiver {
    pub fn new(func: F) -> Self {
        SignalFn {
            func,
            signal: Vec::default(),
        }
    }
    pub fn push<R>(&mut self, receiver: R) where R: Receiver<D> + Send + 'static{
        self.receivers.push(Box::new(receiver));
    }
}
impl<D> Receiver<D> for SignalFn<D> {
    fn receive(&self, data: &D) {
        self.func(data);
        self.send(data);
    }
}
impl<D> Sender<D> for SignalFn<D> {
    fn send(&self, data: &D) {
        for receiver in &self.receivers {
            receiver.receive(data)
        }
    }
}