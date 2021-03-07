#![recursion_limit="128"]
#![warn(unreachable_pub)]

pub mod signal;
pub mod receiver;
pub mod emitter;

// mod signal_fn;

// pub use signal_fn::SignalFn;
pub use signal::Signal;
pub use receiver::Receiver;
pub use emitter::Emitter;