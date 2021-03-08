use std::sync::Weak;

use crate::{Emitter, Mutable, Receiver};

pub struct SignalVar<D> {
    value: D,
    change_receivers: Vec<Box<dyn Receiver<D>>>,
}

impl<D> SignalVar<D> {
    pub fn new(value: D) -> Self {
        SignalVar {
            value,
            change_receivers: Vec::default(),
        }
    }
}
impl<D> Mutable<D> for SignalVar<D> {
    fn new(value: D) -> Self where D:Sized {
        SignalVar {
            value,
            change_receivers: vec![],
        }
    }
    fn get(&self) -> &D {
        &self.value
    }
    fn set(&mut self, value: D) {
        let old = std::mem::replace(&mut self.value, value);

    }
    fn on_change<R>(&mut self, receiver: R) where R: Receiver<D> + Send + 'static{
        self.change_receivers.push(Box::new(receiver));
    }
    fn change_map<F, T, R>(&mut self, func: F) -> R where F: Fn(&D) -> T + Send + 'static, R: Mutable<T>{
        let target = R::new(func(self.get()));
        self.change_receivers.push(Box::new(|data: &D|{
            target.set(func(self.get()));
        }));
        target
    }
}