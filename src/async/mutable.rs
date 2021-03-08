use crate::{Emitter, Receiver};

pub trait Mutable<D> {
    fn new(value: D) -> Self
    where
        D: Sized;
    fn get(&self) -> &D;
    fn set(&mut self, value: D);
    fn on_change<R>(&mut self, receiver: R)
    where
        R: Receiver<D> + Send + 'static;
    fn change_map<F, T, R>(&mut self, func: F) -> R
    where
        F: Fn(&D) -> T + Send + 'static,
        R: Mutable<T>;
}
pub struct Changed<'a, D> {
    old: &'a D,
    new: &'a D,
}
