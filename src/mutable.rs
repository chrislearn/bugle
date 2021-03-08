use crate::Receiver;

pub trait Mutable<'a, D>
where
    D: Clone + 'static,
{
    fn new(value: D) -> Self
    where
        D: Sized;
    fn get(&self) -> &D;
    fn set(&mut self, value: D);
    fn on_change<R>(&mut self, receiver: R)
    where
        R: Receiver<Changed<D>> + Send + 'a;
    //     fn change_map<F, T, R>(&mut self, func: F) -> R
    //     where
    //         F: Fn(&D) -> T + Send + 'static,
    //         R: Mutable<T>;
}
#[derive(Debug, Clone, Copy)]
pub struct Changed<D> {
    pub old: D,
    pub new: D,
}
