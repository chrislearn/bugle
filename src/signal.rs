use std::sync::{Arc, Weak, Mutex};

use super::{Emitter, Receiver};

pub struct Signal<'a, D> {
    receivers: Vec<Weak<Mutex<Box<dyn Receiver<D> + 'a>>>>,
}

impl<'a, D> Signal<'a, D> {
    pub fn new() -> Self {
        Signal {
            receivers: Vec::default(),
        }
    }
    pub fn push<R>(&mut self, receiver: R) where R: Receiver<D> + Send + 'a {
        self.receivers.push(Arc::downgrade(&Arc::new(Mutex::new(Box::new(receiver)))));
    }
}
impl<'a, D> Emitter<D> for Signal<'a, D> {
    fn emit(&mut self, data: &D) {
        for receiver in &mut self.receivers {
            if let Some(receiver) = receiver.upgrade() {
                // let mut receiver = receiver.lock().unwrap();
                // receiver.receive(data);
                if let Ok(mut receiver) = receiver.lock() {
                    receiver.receive(data);
                }
            }
        }
    }
}