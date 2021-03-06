
pub struct SignalVar<A> {
    value: A,
    change: Signal<A>,
}

impl SignalVar<A> {
    pub fn new(value: A) -> Muteable<A> {
        Muteable {
            value,
            change: Signal::<A>::new(),
        }
    }
    pub fn change(&self, f: Fn(&A, &A) -> &Self {
        self.change.push(f);
    }
    pub fn set(&mut self, value: A) {
        let old = memo.replace(self.value);
        self.value = value;
        self.change.emit((&old, &self.value));
    }
}