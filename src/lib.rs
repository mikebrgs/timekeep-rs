mod set;
mod atomic;
mod bound;

// pub use interval::{Bound, AtomicInterval};
pub use atomic::AtomicInterval;
pub use bound::Bound;
pub use set::Interval;