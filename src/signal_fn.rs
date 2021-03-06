use super::{Signal, Receiver};

pub struct SignalFn<F, D> {
    func: F,
    signal: Signal<D>,
}

impl<F, D> SignalFn<D> where F: Fn(&D), D: Receiver {
    pub fn new(func: F) -> Self {
        SignalFn {
            func,
            signal: Signal::default(),
        }
    }
    pub fn push<R>(&mut self, receiver: R) {
        self.signal.push(receiver);
    }
}