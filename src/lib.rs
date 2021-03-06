#![recursion_limit="128"]
#![warn(unreachable_pub)]

pub mod signal;
pub mod receiver;

// mod signal_fn;

// pub use signal_fn::SignalFn;
pub use signal::Signal;
pub use receiver::Receiver;