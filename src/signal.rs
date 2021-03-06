use super::receiver::Receiver;

pub struct Signal<D> {
    receivers: Vec<Box<dyn Receiver<D>>>,
}

impl<D> Signal<D> {
    pub fn push(&mut self, receiver: Box<dyn Receiver<D>>) {
        self.receivers.push(receiver);
    }
    pub fn emit(&self, data: &D) {
        for receiver in &self.receivers {
            receiver.call(data)
        }
    }
}
