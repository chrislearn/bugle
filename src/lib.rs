#![recursion_limit="128"]
#![warn(unreachable_pub)]

pub mod signal;
pub mod receiver;
pub mod emitter;
pub mod mutable;
mod signal_var;

pub use signal::Signal;
pub use receiver::Receiver;
pub use emitter::Emitter;
pub use mutable::Mutable;
pub use signal_var::SignalVar;