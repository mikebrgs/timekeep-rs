//! # timekeep-rs
//!
//! A library for working with intervals and sets of intervals.
//!
//! This library provides data structures and methods for creating,
//! manipulating, and performing set operations on intervals.
//!
//! ## Modules
//!
//! - [`atomic`]: Defines the [`AtomicInterval`] struct and its associated methods.
//! - [`bound`]: Defines the [`Bound`] enum, representing the boundaries of an interval.
//! - [`set`]: Defines the [`Interval`] struct, representing a set of intervals, and its associated methods.
//!
//! ## Usage
//!
//! Add `timekeep-rs` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! timekeep-rs = "0.1.0"
//! ```
//!
//! Then, you can use the library in your Rust code:
//!
//! ```rust
//! use timekeep_rs::{AtomicInterval, IntervalSet};
//!
//! let atomic_interval = AtomicInterval::closed(1, 5);
//! let interval = IntervalSet::from(atomic_interval);
//!
//! println!("Interval: {}", interval.to_string());
//! ```

pub mod set;
pub mod atomic;
pub mod bound;

pub use atomic::AtomicInterval;
pub use bound::Bound;
pub use set::IntervalSet;