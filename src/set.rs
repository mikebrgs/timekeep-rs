//! # Interval Set Operations Module
//! 
//! This module provides implementations for interval set operations through the `Interval` type,
//! which manages collections of `AtomicInterval`s and supports various set operations.
//! 
//! ## Core Features
//! 
//! - Create interval sets from atomic intervals
//! - Perform set operations (union, intersection, difference)
//! - Handle both overlapping and disjoint intervals
//! 
//! ## Main Types
//! 
//! - `Interval<T>`: A collection of atomic intervals supporting set operations
//! 
//! ## Operations
//! 
//! The module supports these primary set operations:
//! 
//! - **Union**: Combines intervals, merging overlapping or adjacent ones
//! - **Intersection**: Finds common regions between intervals
//! - **Difference**: Computes regions present in one interval but not another
//! 
//! ## Example
//! 
//! ```rust
//! use timekeep_rs::{Interval, AtomicInterval};
//! 
//! // Create two intervals
//! let a = Interval::from(AtomicInterval::closed(1, 5));
//! let b = Interval::from(AtomicInterval::closed(3, 7));
//! 
//! // Perform set operations
//! let union = a.union(&b);                    // Results in [1, 7]
//! let intersection = a.intersection(&b);       // Results in [3, 5]
//! let difference = a.difference(&b);          // Results in [1, 3)
//! ```
//! 
//! ## Type Parameters
//! 
//! - `T`: Represents the boundary type for intervals
//!   - Must implement `Clone`
//!   - Must implement `PartialOrd` for set operations
use crate::atomic::AtomicInterval;

pub struct Interval<T: Clone> {
    intervals: Vec<AtomicInterval<T>>,
}

/// Returns true if this interval set has no intervals.
///
/// # Examples
///
/// ```
/// // Suppose `interval` is an `Interval` with no intervals.
/// assert!(interval.is_empty());
/// ```
impl<T: Clone> Interval<T> {
    pub fn is_empty(&self) -> bool {
        self.intervals.is_empty()
    }
}

impl<T: Clone> From<AtomicInterval<T>> for Interval<T> {
    /// Creates a new `Interval<T>` from an `AtomicInterval<T>`.
    ///
    /// This implementation allows converting a single atomic interval into an `Interval`
    /// collection by wrapping it in a vector.
    ///
    /// # Examples
    /// ```
    /// use timekeep_rs::AtomicInterval;
    /// use timekeep_rs::Interval;
    /// 
    /// let atomic = AtomicInterval::closed(1, 5);
    /// let interval: Interval<i32> = atomic.into();
    /// ```
    fn from(interval: AtomicInterval<T>) -> Self {
        Interval {
            intervals: vec![interval],
        }
    }
}

/// A trait implementation for `Interval<T>` where `T` implements `PartialOrd` and `Clone`.
/// Provides set operations for intervals.
impl <T: PartialOrd + Clone> Interval<T> {
    /// Computes the union of two intervals.
    ///
    /// The union of two intervals is the smallest interval that contains both intervals.
    /// This operation merges overlapping or adjacent intervals.
    ///
    /// # Arguments
    ///
    /// * `other` - Another interval to compute the union with
    ///
    /// # Returns
    ///
    /// A new `Interval` representing the union of both intervals
    /// # Examples
    ///
    /// ```
    /// use timekeep_rs::AtomicInterval;
    /// use timekeep_rs::Interval;
    ///
    /// // Create two intervals
    /// let interval1 = Interval::from(AtomicInterval::closed(1, 5));
    /// let interval2 = Interval::from(AtomicInterval::closed(3, 7));
    ///
    /// // Compute union (results in [1, 7])
    /// let union = interval1.union(&interval2);
    /// ```
    pub fn union(&self, other: &Self) -> Self {
        let mut intervals = self.intervals.clone();
        intervals.extend(other.intervals.iter().cloned());

        // Sort intervals by the value of their left boundary.
        intervals.sort_by(|a, b| a.left.value().partial_cmp(b.left.value()).unwrap());

        let mut merged = Vec::new();

        for interval in intervals {
            if let Some(last) = merged.last_mut() {
                if let Some(m) = AtomicInterval::union(last, &interval) {
                    *last = m;
                    continue;
                }
            }
            merged.push(interval);
        }

        Interval { intervals: merged }
    }

    /// Computes the intersection of two intervals.
    ///
    /// The intersection of two intervals is the interval containing all points that are in both intervals.
    ///
    /// # Arguments
    ///
    /// * `other` - Another interval to compute the intersection with
    ///
    /// # Returns
    ///
    /// * `Some(Interval)` if the intervals intersect
    /// * `None` if the intervals are disjoint
    /// 
    /// # Examples
    ///
    /// ```
    /// use timekeep_rs::AtomicInterval;
    /// use timekeep_rs::Interval;
    ///
    /// // Create two intervals
    /// let interval1 = Interval::from(AtomicInterval::closed(1, 5));
    /// let interval2 = Interval::from(AtomicInterval::closed(3, 7));
    ///
    /// // Compute intersection (results in [3, 5])
    /// let intersection = interval1.intersection(&interval2).unwrap();
    /// ```
    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let mut intervals = Vec::new();

        for interval in &self.intervals {
            for other_interval in &other.intervals {
                if let Some(intersection) = interval.intersection(other_interval) {
                    intervals.push(intersection);
                }
            }
        }

        if intervals.is_empty() {
            None
        } else {
            Some(Interval { intervals })
        }
    }

    /// Computes the difference between two intervals.
    ///
    /// The difference A - B contains all points that are in A but not in B.
    ///
    /// # Arguments
    ///
    /// * `other` - Another interval to subtract from this interval
    ///
    /// # Returns
    ///
    /// A new `Interval` representing the difference between the intervals
    /// 
    /// # Examples
    ///
    /// ```
    /// use timekeep_rs::AtomicInterval;
    /// use timekeep_rs::Interval;
    ///
    /// // Create two intervals
    /// let interval1 = Interval::from(AtomicInterval::closed(1, 5));
    /// let interval2 = Interval::from(AtomicInterval::closed(3, 7));
    ///
    /// // Compute difference (results in [1, 3])
    /// let difference = interval1.difference(&interval2);
    /// ```
    pub fn difference(&self, other: &Self) -> Self {
        let mut result = Vec::new();

        for interval in &self.intervals {
            let mut remaining = vec![interval.clone()];
            for other_interval in &other.intervals {
                let mut new_remaining = Vec::new();
                for part in remaining {
                    new_remaining.extend(part.difference(other_interval));
                }
                remaining = new_remaining;
            }
            result.extend(remaining);
        }

        Interval { intervals: result }
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_from_atomic_interval() {
        let atomic_interval = AtomicInterval::closed(1, 5);
        let interval: Interval<i32> = Interval::from(atomic_interval.clone());
        assert_eq!(interval.intervals.len(), 1);
        assert_eq!(interval.intervals[0], atomic_interval);
    }

    #[test]
    fn test_union_between_two_overlapping_intervals() {
        let interval1 = AtomicInterval::closed(1, 3);
        let interval2 = AtomicInterval::closed(4, 7);
        let interval3 = AtomicInterval::closed(2, 4);
        let interval4 = AtomicInterval::closed(7, 8);
        let union = Interval::from(interval1).union(&Interval::from(interval2));
        let union = union.union(&Interval::from(interval3));
        let union = union.union(&Interval::from(interval4));
        assert_eq!(union.intervals.len(), 1);
        assert_eq!(union.intervals[0], AtomicInterval::closed(1, 8));
    }

    #[test]
    fn test_union_between_two_disjoint_intervals() {
        let interval1 = AtomicInterval::closed(1, 3);
        let interval2 = AtomicInterval::closed(4, 7);
        let interval3 = AtomicInterval::closed(5, 8);
        let union = Interval::from(interval1).union(&Interval::from(interval2));
        let union = union.union(&Interval::from(interval3));
        assert_eq!(union.intervals.len(), 2);
        assert_eq!(union.intervals[0], AtomicInterval::closed(1, 3));
        assert_eq!(union.intervals[1], AtomicInterval::closed(4, 8));
    }

    #[test]
    fn test_intersection_between_two_overlapping_intervals() {
        let interval1 = AtomicInterval::closed(1, 5);
        let interval2 = AtomicInterval::closed(3, 7);
        let interval1 = Interval::from(interval1);
        let interval2 = Interval::from(interval2);
        let intersection = interval1.intersection(&interval2).unwrap();
        assert_eq!(intersection.intervals.len(), 1);
        assert_eq!(intersection.intervals[0], AtomicInterval::closed(3, 5));
    }

    #[test]
    fn test_intersection_between_two_disjoint_intervals() {
        let interval1 = AtomicInterval::closed(1, 3);
        let interval2 = AtomicInterval::closed(4, 7);
        let interval1 = Interval::from(interval1);
        let interval2 = Interval::from(interval2);
        let intersection = interval1.intersection(&interval2);
        assert!(intersection.is_none());
    }

    #[test]
    fn test_difference_between_two_overlapping_intervals() {
        let interval1 = AtomicInterval::closed(1, 5);
        let interval2 = AtomicInterval::closed(3, 7);
        let interval1 = Interval::from(interval1);
        let interval2 = Interval::from(interval2);
        let difference = interval1.difference(&interval2);
        assert_eq!(difference.intervals.len(), 1);
        assert_eq!(difference.intervals[0], AtomicInterval::closed_open(1, 3));
    }

    #[test]
    fn test_difference_between_two_disjoint_intervals() {
        let interval1 = AtomicInterval::closed(1, 3);
        let interval2 = AtomicInterval::closed(4, 7);
        let interval1 = Interval::from(interval1);
        let interval2 = Interval::from(interval2);
        let difference = interval1.difference(&interval2);
        assert_eq!(difference.intervals.len(), 1);
        assert_eq!(difference.intervals[0], AtomicInterval::closed(1, 3));
    }


}
