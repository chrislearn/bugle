use std::sync::{Arc, Weak};

use super::{Emitter, Receiver};

pub struct Signal<D> {
    receivers: Vec<Weak<Box<dyn Receiver<D>>>>,
}

impl<D> Signal<D> {
    pub fn new() -> Self {
        Signal {
            receivers: Vec::default(),
        }
    }
    pub fn push<R>(&mut self, receiver: R) where R: Receiver<D> + Send + 'static{
        self.receivers.push(Arc::downgrade(&Arc::new(Box::new(receiver))));
    }
}
impl<D> Emitter<D> for Signal<D> {
    fn emit(&self, data: &D) {
        for receiver in &self.receivers {
            if let Some(receiver) = receiver.upgrade() {
                receiver.receive(data);
            }
        }
    }
}