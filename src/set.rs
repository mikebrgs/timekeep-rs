//! # Interval Set Operations Module
//!
//! This module provides implementations for interval set operations through the [`IntervalSet`] type,
//! which manages collections of [`AtomicInterval`]s and supports various set operations.
//!
//! ## Main Types
//!
//! - [`IntervalSet<T>`]: A collection of atomic intervals supporting set operations
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
//! use timekeep_rs::{IntervalSet, AtomicInterval};
//!
//! // Create two intervals
//! let a = IntervalSet::from(AtomicInterval::closed(1, 5));
//! let b = IntervalSet::from(AtomicInterval::closed(3, 7));
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
//!   - Must implement [`Clone`]
//!   - Must implement [`PartialOrd`] for set operations
use crate::atomic::AtomicInterval;

#[derive(Debug, Clone, PartialEq)]
pub struct IntervalSet<T> {
    /// A vector of AtomicIntervals that make up the IntervalSet
    pub intervals: Vec<AtomicInterval<T>>,
}

impl<T: ToString> ToString for IntervalSet<T> {
    /// Converts the interval set to a string representation.
    ///
    /// # Examples
    ///
    /// ```
    /// use timekeep_rs::AtomicInterval;
    /// use timekeep_rs::IntervalSet;
    ///
    /// let interval = IntervalSet::from(AtomicInterval::closed(1, 5));
    /// assert_eq!(interval.to_string(), "[[1, 5]]");
    /// ```
    fn to_string(&self) -> String {
        let mut result = String::from("[");
        for interval in &self.intervals {
            result.push_str(&interval.to_string());
        }
        result.push_str("]");
        result
    }
    
}

impl<T: Clone> IntervalSet<T> {
    /// Returns `true` if this interval set has no intervals or is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use timekeep_rs::IntervalSet;
    ///
    /// // Suppose `interval` is an `Interval` with no intervals.
    /// let interval = IntervalSet::<i32> { intervals: vec![] };
    ///
    /// assert!(interval.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.intervals.is_empty()
    }

    /// Returns an empty `IntervalSet`.
    ///
    /// # Examples
    ///
    /// ```
    /// use timekeep_rs::IntervalSet;
    ///
    /// // Suppose `interval` is an `Interval` with no intervals.
    /// let interval = IntervalSet::<i32>::new();
    ///
    /// assert!(interval.is_empty());
    /// ```
    pub fn new() -> IntervalSet<T> {
        return IntervalSet { intervals: vec![] }
    }

}

impl<T: Clone> From<AtomicInterval<T>> for IntervalSet<T> {
    /// Creates a new `IntervalSet<T>` from an `AtomicInterval<T>`.
    ///
    /// This implementation allows converting a single atomic interval into an `IntervalSet<T>`
    /// collection by wrapping it in a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use timekeep_rs::AtomicInterval;
    /// use timekeep_rs::IntervalSet;
    ///
    /// let atomic = AtomicInterval::closed(1, 5);
    /// let interval: IntervalSet<i32> = atomic.into();
    /// ```
    fn from(interval: AtomicInterval<T>) -> Self {
        IntervalSet {
            intervals: vec![interval],
        }
    }
}

/// A trait implementation for `IntervalSet<T>` where `T` implements `PartialOrd` and `Clone`.
/// Provides set operations for interval sets.
impl<T: PartialOrd + Clone> IntervalSet<T> {
    /// Computes the union of two interval sets.
    ///
    /// The union of two interval sets is a new interval set that contains all the intervals
    /// from both input sets, merging any overlapping or adjacent intervals.
    ///
    /// # Arguments
    ///
    /// * `other` - Another interval set to compute the union with
    ///
    /// # Returns
    ///
    /// A new `IntervalSet<T>` representing the union of both interval sets
    ///
    /// # Examples
    ///
    /// ```
    /// use timekeep_rs::AtomicInterval;
    /// use timekeep_rs::IntervalSet;
    ///
    /// // Create two interval sets
    /// let interval1 = IntervalSet::from(AtomicInterval::closed(1, 5));
    /// let interval2 = IntervalSet::from(AtomicInterval::closed(3, 7));
    ///
    /// // Compute union (results in [1, 7])
    /// let union = interval1.union(&interval2);
    /// ```
    pub fn union(&self, other: &Self) -> Self {
        let mut intervals = self.intervals.clone();
        intervals.extend(other.intervals.iter().cloned());

        // Sort intervals by the value of their left boundary.
        intervals.sort_by(
            |a, b| a.left().value().partial_cmp(b.left().value()).unwrap()
        );

        let mut merged: Vec<AtomicInterval<T>> = Vec::new();

        for interval in intervals {
            if let Some(last) = merged.last_mut() {
                let union_vec = AtomicInterval::union(last, &interval);
    
                if union_vec.len() == 1 {
                    // Successfully merged, update last interval
                    *last = union_vec.into_iter().next().unwrap();
                    continue;
                } else if union_vec.len() > 1 {
                    // If union() returned multiple intervals, replace last and insert the new one
                    *last = union_vec[0].clone();
                    merged.extend(union_vec.into_iter().skip(1));
                    continue;
                }
            }
            merged.push(interval);
        }

        IntervalSet { intervals: merged }
    }

    /// Computes the intersection of two interval sets.
    ///
    /// The intersection of two interval sets is a new interval set that contains all the intervals
    /// that are common to both input sets.
    ///
    /// # Arguments
    ///
    /// * `other` - Another interval set to compute the intersection with
    ///
    /// # Returns
    ///
    /// * `Some(IntervalSet<T>)` if the interval sets intersect
    /// * `None` if the interval sets are disjoint
    ///
    /// # Examples
    ///
    /// ```
    /// use timekeep_rs::AtomicInterval;
    /// use timekeep_rs::IntervalSet;
    ///
    /// // Create two interval sets
    /// let interval1 = IntervalSet::from(AtomicInterval::closed(1, 5));
    /// let interval2 = IntervalSet::from(AtomicInterval::closed(3, 7));
    ///
    /// // Compute intersection (results in [3, 5])
    /// let intersection = interval1.intersection(&interval2);
    /// ```
    pub fn intersection(&self, other: &Self) -> Self {
        let mut intervals = Vec::new();

        for interval in &self.intervals {
            for other_interval in &other.intervals {
                interval.intersection(other_interval).iter().for_each(
                    |x| intervals.push(x.clone())
                );
                // if intersection_vec.len() > 1 {
                //     panic!("Unexpected behavior from intersection.")
                // } else if intersection_vec.len() == 1 {
                //     intervals.push(intersection);

                // }
                // if let Some(intersection) = interval.intersection(other_interval) {
                //     intervals.push(intersection);
                // }
            }
        }

        if intervals.is_empty() {
            IntervalSet::new()
        } else {
            IntervalSet { intervals }
        }
    }

    /// Computes the difference between two interval sets.
    ///
    /// The difference A - B contains all points that are in A but not in B.
    ///
    /// # Arguments
    ///
    /// * `other` - Another interval set to subtract from this interval set
    ///
    /// # Returns
    ///
    /// A new `IntervalSet<T>` representing the difference between the interval sets
    ///
    /// # Examples
    ///
    /// ```
    /// use timekeep_rs::AtomicInterval;
    /// use timekeep_rs::IntervalSet;
    ///
    /// // Create two interval sets
    /// let interval1 = IntervalSet::from(AtomicInterval::closed(1, 5));
    /// let interval2 = IntervalSet::from(AtomicInterval::closed(3, 7));
    ///
    /// // Compute difference (results in [1, 3))
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

        IntervalSet { intervals: result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interval_from_atomic_interval() {
        let atomic_interval = AtomicInterval::closed(1, 5);
        let interval_set: IntervalSet<i32> = IntervalSet::from(atomic_interval.clone());
        assert_eq!(interval_set.intervals.len(), 1);
        assert_eq!(interval_set.intervals[0], atomic_interval);
    }

    #[test]
    fn test_union_between_two_overlapping_intervals() {
        let interval1 = AtomicInterval::closed(1, 3);
        let interval2 = AtomicInterval::closed(4, 7);
        let interval3 = AtomicInterval::closed(2, 4);
        let interval4 = AtomicInterval::closed(7, 8);
        let union = IntervalSet::from(interval1).union(&IntervalSet::from(interval2));
        let union = union.union(&IntervalSet::from(interval3));
        let union = union.union(&IntervalSet::from(interval4));
        assert_eq!(union.intervals.len(), 1);
        assert_eq!(union.intervals[0], AtomicInterval::closed(1, 8));
    }

    #[test]
    fn test_union_between_two_disjoint_intervals() {
        let interval1 = AtomicInterval::closed(1, 3);
        let interval2 = AtomicInterval::closed(4, 7);
        let interval3 = AtomicInterval::closed(5, 8);
        let union = IntervalSet::from(interval1).union(&IntervalSet::from(interval2));
        let union = union.union(&IntervalSet::from(interval3));
        assert_eq!(union.intervals.len(), 2);
        assert_eq!(union.intervals[0], AtomicInterval::closed(1, 3));
        assert_eq!(union.intervals[1], AtomicInterval::closed(4, 8));
    }

    #[test]
    fn test_intersection_between_two_overlapping_intervals() {
        let interval1 = AtomicInterval::closed(1, 5);
        let interval2 = AtomicInterval::closed(3, 7);
        let interval1 = IntervalSet::from(interval1);
        let interval2 = IntervalSet::from(interval2);
        let intersection = interval1.intersection(&interval2);
        assert_eq!(intersection.intervals.len(), 1);
        assert_eq!(intersection.intervals[0], AtomicInterval::closed(3, 5));
    }

    #[test]
    fn test_intersection_between_two_disjoint_intervals() {
        let interval1 = AtomicInterval::closed(1, 3);
        let interval2 = AtomicInterval::closed(4, 7);
        let interval1 = IntervalSet::from(interval1);
        let interval2 = IntervalSet::from(interval2);
        let intersection = interval1.intersection(&interval2);
        assert!(intersection.is_empty());
    }

    #[test]
    fn test_difference_between_two_overlapping_intervals() {
        let interval1 = AtomicInterval::closed(1, 5);
        let interval2 = AtomicInterval::closed(3, 7);
        let interval1 = IntervalSet::from(interval1);
        let interval2 = IntervalSet::from(interval2);
        let difference = interval1.difference(&interval2);
        assert_eq!(difference.intervals.len(), 1);
        assert_eq!(difference.intervals[0], AtomicInterval::closed_open(1, 3));
    }

    #[test]
    fn test_difference_between_two_disjoint_intervals() {
        let interval1 = AtomicInterval::closed(1, 3);
        let interval2 = AtomicInterval::closed(4, 7);
        let interval1 = IntervalSet::from(interval1);
        let interval2 = IntervalSet::from(interval2);
        let difference = interval1.difference(&interval2);
        assert_eq!(difference.intervals.len(), 1);
        assert_eq!(difference.intervals[0], AtomicInterval::closed(1, 3));
    }
}