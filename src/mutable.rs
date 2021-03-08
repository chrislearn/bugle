use crate::{Emitter, Receiver};

pub trait Mutable<'a, 'b, D> where D: Clone + 'static{
    fn new(value: D) -> Self
    where
        D: Sized;
    fn get(&self) -> &D;
    fn set(&mut self, value: D);
    fn on_change<R>(&mut self, receiver: R)
    where
        R: Receiver<Changed<'b, D>> + Send + 'a;
    //     fn change_map<F, T, R>(&mut self, func: F) -> R
    //     where
    //         F: Fn(&D) -> T + Send + 'static,
    //         R: Mutable<T>;
}
#[derive(Debug, Clone, Copy)]
pub struct Changed<'b, D> { 
    pub old: &'b D,
    // pub new: &'b D,
}
