use std::sync::{Arc, Weak, Mutex};

use crate::{Emitter, Mutable, Receiver};
use crate::mutable::Changed;

pub struct SignalVar<'a, D> {
    value: D,
    change_receivers: Vec<Weak<Mutex<Box<dyn Receiver<Changed<D>> + 'a>>>>,
}

// pub struct SignalVarMap<'a, D, F> {
//     value: &'a, D,
//     func: F,
//     change_receivers: Vec<Box<dyn Receiver<D>>>,
// }


impl<'a, D> SignalVar<'a, D> {
    pub fn new(value: D) -> Self {
        SignalVar {
            value,
            change_receivers: Vec::default(),
        }
    }
}
impl<'a, D> Mutable<'a, D> for SignalVar<'a, D> where D: Clone + 'static {
    fn new(value: D) -> Self where D:Sized {
        SignalVar {
            value,
            change_receivers: Vec::default(),
        }
    }
    fn get(&self) -> &D {
        &self.value
    }
    fn set(&mut self, value: D) {
        let old = std::mem::replace(&mut self.value, value);
        let changed = Changed {
            old: old,
            new: self.value.clone(),
        };
        for receiver in &mut self.change_receivers {
            if let Some(receiver) = receiver.upgrade() {
                if let Ok(mut receiver) = receiver.lock() {
                    receiver.receive(&changed);
                }
            }
        }
    }
    fn on_change<R>(&mut self, receiver: R) where R: Receiver<Changed<D>> + Send + 'a{
        self.change_receivers.push(Arc::downgrade(&Arc::new(Mutex::new(Box::new(receiver)))));
    }
    // fn change_map<F, T, R>(&mut self, func: F) -> R where F: Fn(&D) -> T + Send + 'static, R: Mutable<T>{
    //     let target = R::new(func(self.get()));
    //     self.change_receivers.push(Box::new(|data: &D|{
    //         target.set(func(self.get()));
    //     }));
    //     target
    // }
}