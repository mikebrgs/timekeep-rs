//! A module containing the `AtomicInterval` struct and its implementations.
//! An atomic interval is a closed or open interval that contains a single value or a range of values.
//! 
//! # Examples
//! ```
//! use timekeep_rs::{AtomicInterval, Bound};
//! 
//! let interval = AtomicInterval::closed(1, 5);
//! assert_eq!(*interval.left(), Bound::Included(1));
//! assert_eq!(*interval.right(), Bound::Included(5));
//! ```
//!
use crate::Bound;

/// A struct representing an atomic interval.
/// An atomic interval is a closed or open interval that contains a single value or a range of values.
/// 
/// # Fields
/// * `left` - The left endpoint of the interval
/// * `right` - The right endpoint of the interval
/// 
/// # Examples
/// ```
/// use timekeep_rs::{AtomicInterval, Bound};
///
/// let interval = AtomicInterval::closed(1, 5);
/// assert_eq!(*interval.left(), Bound::Included(1));
/// assert_eq!(*interval.right(), Bound::Included(5));
/// ```
/// 
#[derive(PartialEq, Debug, Clone)]
pub struct AtomicInterval<T> {
    left: Bound<T>,
    right: Bound<T>,
}


/// Implementation of the `ToString` trait for `AtomicInterval`.
impl<T: ToString> ToString for AtomicInterval<T> {
    /// This allows `AtomicInterval` to be converted to a string.
    /// 
    /// # Returns
    /// A string representation of the `AtomicInterval`
    fn to_string(&self) -> String {
        match (&self.left, &self.right) {
            (Bound::Included(l), Bound::Included(r)) => format!("[{}, {}]", l.to_string(), r.to_string()),
            (Bound::Included(l), Bound::Excluded(r)) => format!("[{}, {})", l.to_string(), r.to_string()),
            (Bound::Excluded(l), Bound::Included(r)) => format!("({}, {}]", l.to_string(), r.to_string()),
            (Bound::Excluded(l), Bound::Excluded(r)) => format!("({}, {})", l.to_string(), r.to_string()),
        }
    }
}


/// A collection of constructors for creating different types of atomic intervals.
impl<T: Clone + PartialOrd> AtomicInterval<T> {
    /// Creates an open interval (a,b) that excludes both endpoints.
    ///
    /// # Arguments
    /// * `left` - The left endpoint of the interval
    /// * `right` - The right endpoint of the interval
    ///
    /// # Returns
    /// A new `AtomicInterval` with excluded endpoints
    pub fn open(left: T, right: T) -> Self {
        if left >= right {
            panic!("The following condition must be valid: `left < right`");
        }
        AtomicInterval { left: Bound::Excluded(left), right: Bound::Excluded(right) }
    }

    /// Creates a closed interval [a,b] that includes both endpoints.
    ///
    /// # Arguments
    /// * `left` - The left endpoint of the interval
    /// * `right` - The right endpoint of the interval
    ///
    /// # Returns
    /// A new `AtomicInterval` with included endpoints
    pub fn closed(left: T, right: T) -> Self {
        if left >= right {
            panic!("The following condition must be valid: `left < right`");
        }
        AtomicInterval { left: Bound::Included(left), right: Bound::Included(right) }
    }

    /// Creates a left-open, right-closed interval (a,b] that excludes the left endpoint and includes the right endpoint.
    ///
    /// # Arguments
    /// * `left` - The left endpoint of the interval
    /// * `right` - The right endpoint of the interval
    ///
    /// # Returns
    /// A new `AtomicInterval` with excluded left endpoint and included right endpoint
    pub fn open_closed(left: T, right: T) -> Self {
        if left >= right {
            panic!("The following condition must be valid: `left < right`");
        }
        AtomicInterval { left: Bound::Excluded(left), right: Bound::Included(right) }
    }

    /// Creates a left-closed, right-open interval [a,b) that includes the left endpoint and excludes the right endpoint.
    ///
    /// # Arguments
    /// * `left` - The left endpoint of the interval
    /// * `right` - The right endpoint of the interval
    ///
    /// # Returns
    /// A new `AtomicInterval` with included left endpoint and excluded right endpoint
    pub fn closed_open(left: T, right: T) -> Self {
        if left >= right {
            panic!("The following condition must be valid: `left < right`");
        }
        AtomicInterval { left: Bound::Included(left), right: Bound::Excluded(right) }
    }

    /// Creates a point interval [a,a] containing a single value.
    ///
    /// # Arguments
    /// * `value` - The value to create a point interval from
    ///
    /// # Returns
    /// A new `AtomicInterval` representing a single point
    pub fn point(value: T) -> Self {
        AtomicInterval { left: Bound::Included(value.clone()), right: Bound::Included(value) }
    }
}


impl<T> AtomicInterval<T> {
    /// Return a reference to the left bound.
    /// 
    /// # Returns
    /// A reference of `Bound` associated to the left bound.
    pub fn left(&self) -> &Bound<T> {
        &self.left
    }

    /// Return a reference to the right bound.
    /// 
    /// # Returns
    /// A reference of `Bound` associated to the right bound.
    pub fn right(&self) -> &Bound<T> {
        &self.right
    }
}

/// A collection of methods for performing set operations on atomic intervals.
impl <T: PartialOrd> AtomicInterval<T> {
    /// Checks if the interval is a superset of another interval.
    /// An interval is a superset of another if it contains all the elements of the other interval.
    /// 
    /// # Arguments
    /// * `other` - The other interval to check if it is a subset of the current interval
    /// 
    /// # Returns
    /// `true` if the current interval is a superset of the other interval, `false` otherwise
    /// 
    /// # Examples
    /// ```
    /// use timekeep_rs::AtomicInterval;
    ///
    /// let interval1 = AtomicInterval::closed(1, 5);
    /// let interval2 = AtomicInterval::closed(2, 4);
    /// assert!(interval1.is_superset(&interval2));
    /// ```
    /// 
    pub fn is_superset (&self, other: &AtomicInterval<T>) -> bool {
        match (&self.left, &self.right, &other.left, &other.right) {
            (Bound::Included(l1), Bound::Excluded(r1), _, Bound::Included(r2)) => l1 <= other.left.value() && r1 > r2,
            (Bound::Excluded(l1), Bound::Included(r1), Bound::Included(l2), _) => l1 < l2 && r1 >= other.right.value(),
            (Bound::Excluded(l1), Bound::Excluded(r1), Bound::Included(l2), Bound::Included(r2)) => l1 < l2 && r1 > r2,
            (_, _, _, _) => self.left.value() <= other.left.value() && self.right.value() >= other.right.value(),
        }
    }

    /// Checks if the interval is a subset of another interval.
    /// An interval is a subset of another if it is contained within the other interval.
    /// 
    /// # Arguments
    /// * `other` - The other interval to check if it is a superset of the current interval
    ///
    /// # Returns
    /// `true` if the current interval is a subset of the other interval, `false` otherwise
    /// 
    /// # Examples
    /// ```
    /// use timekeep_rs::AtomicInterval;
    ///
    /// let interval1 = AtomicInterval::closed(2, 4);
    /// let interval2 = AtomicInterval::closed(1, 5);
    /// assert!(interval1.is_subset(&interval2));
    /// ```
    /// 
    pub fn is_subset (&self, other: &AtomicInterval<T>) -> bool {
        other.is_superset(self)
    }

    /// Checks if the interval is overlapping with another interval.
    /// Two intervals are overlapping if they share at least one common point.
    /// 
    /// # Arguments
    /// * `other` - The other interval to check if it is overlapping with the current interval
    /// 
    /// # Returns
    /// `true` if the current interval is overlapping with the other interval, `false` otherwise
    /// 
    /// # Examples
    /// ```
    /// use timekeep_rs::AtomicInterval;
    /// 
    /// let interval1 = AtomicInterval::closed(1, 5);
    /// let interval2 = AtomicInterval::closed(4, 6);
    /// assert!(interval1.is_overlapping(&interval2));
    /// ```
    /// 
    pub fn is_overlapping (&self, other: &AtomicInterval<T>) -> bool {
        // Check if the intervals are overlapping on left side of other
        let cond1_overlapping = match (&self.left, &self.right, &other.left) {
            (Bound::Included(l1), Bound::Included(r1), _) => other.left.value() >= l1 && other.left.value() <= r1,
            (Bound::Included(l1), Bound::Excluded(r1), Bound::Included(l2)) => l2 >= l1 && l2 < r1,
            (Bound::Included(l1), Bound::Excluded(r1), Bound::Excluded(l2)) => l2 >= l1 && l2 <= r1,
            (Bound::Excluded(l1), Bound::Included(r1), Bound::Included(l2)) => l2 > l1 && l2 <= r1,
            (Bound::Excluded(l1), Bound::Included(r1), Bound::Excluded(l2)) => l2 >= l1 && l2 <= r1,
            (Bound::Excluded(l1), Bound::Excluded(r1), Bound::Included(l2)) => l2 > l1 && l2 < r1,
            (Bound::Excluded(l1), Bound::Excluded(r1), Bound::Excluded(l2)) => l2 >= l1 && l2 <= r1,
        };
        // Check if the intervals are overlapping on right side of other
        let cond2_overlapping = match (&self.left, &self.right, &other.right) {
            (Bound::Included(l1), Bound::Included(r1), _) => other.right.value() >= l1 && other.right.value() <= r1,
            (Bound::Included(l1), Bound::Excluded(r1), Bound::Included(r2)) => r2 > l1 && r2 <= r1,
            (Bound::Included(l1), Bound::Excluded(r1), Bound::Excluded(r2)) => r2 >= l1 && r2 <= r1,
            (Bound::Excluded(l1), Bound::Included(r1), Bound::Included(r2)) => r2 >= l1 && r2 < r1,
            (Bound::Excluded(l1), Bound::Included(r1), Bound::Excluded(r2)) => r2 >= l1 && r2 <= r1,
            (Bound::Excluded(l1), Bound::Excluded(r1), Bound::Included(r2)) => r2 > l1 && r2 < r1,
            (Bound::Excluded(l1), Bound::Excluded(r1), Bound::Excluded(r2)) => r2 >= l1 && r2 <= r1,
        };
        // They overlap if either condition is true
        return cond1_overlapping || cond2_overlapping;
    }

    /// Checks if the interval is adjacent to another interval.
    /// Two intervals are adjacent if they share a common boundary, but do not overlap.
    /// 
    /// # Arguments
    /// * `other` - The other interval to check if it is adjacent to the current interval
    /// 
    /// # Returns
    /// `true` if the current interval is adjacent to the other interval, `false` otherwise
    /// 
    /// # Examples
    /// ```
    /// use timekeep_rs::AtomicInterval;
    /// 
    /// let interval1 = AtomicInterval::closed(1, 5);
    /// let interval2 = AtomicInterval::open_closed(5, 10);
    /// assert!(interval1.is_adjacent(&interval2));
    /// ```
    pub fn is_adjacent(&self, other: &AtomicInterval<T>) -> bool {
        // Check if the intervals are adjacent on left side of other
        let cond1_adjacent = match (&self.left, &other.right) {
            (Bound::Excluded(_), Bound::Excluded(_)) => false,
            (Bound::Included(_), Bound::Included(_)) => false,
            (_, _) => self.left.value() == other.right.value(),
        };
        // Check if the intervals are adjacent on right side of other
        let cond2_adjacent = match (&self.right, &other.left) {
            (Bound::Excluded(_), Bound::Excluded(_)) => false,
            (Bound::Included(_), Bound::Included(_)) => false,
            (_, _) => self.right.value() == other.left.value(),
        };

        return cond1_adjacent || cond2_adjacent;
    }

    /// Checks if the interval is disjoint from another interval.
    /// Two intervals are disjoint if they do not share any common points.
    /// 
    /// # Arguments
    /// * `other` - The other interval to check if it is disjoint from the current interval
    /// 
    /// # Returns
    /// `true` if the current interval is disjoint from the other interval, `false` otherwise
    /// 
    /// # Examples
    /// ```
    /// use timekeep_rs::AtomicInterval;
    /// 
    /// let interval1 = AtomicInterval::closed(1, 5);
    /// let interval2 = AtomicInterval::closed(6, 10);
    /// assert!(interval1.is_disjoint(&interval2));
    /// ```
    /// 
    pub fn is_disjoint(&self, other: &AtomicInterval<T>) -> bool {
        // Check if the intervals are disjoint on one side
        let cond1_disjoint = match (&self.left, &other.right) {
            (Bound::Included(l1), Bound::Included(r2)) => l1 > r2,
            (_, _) => return self.right.value() <= other.left.value(),
        };

        // Check if the intervals are disjoint on the other side
        let cond2_disjoint = match (&self.right, &other.left) {
            (Bound::Included(r1), Bound::Included(l2)) => r1 < l2,
            (_, _) => return self.left.value() >= other.right.value(),
        };

        return cond1_disjoint || cond2_disjoint;
    }
}

impl <T: PartialOrd + Clone> AtomicInterval<T> {
    /// Computes the union of two overlapping or adjacent intervals.
    /// The union of two intervals is the smallest interval that contains both intervals.
    /// 
    /// # Arguments
    /// * `a` - The first interval to union
    /// * `b` - The second interval to union
    /// 
    /// # Returns
    /// A `Vec` containing the union of the two intervals if they are overlapping or adjacent, an empty `Vec` otherwise
    /// 
    /// # Examples
    /// ```
    /// use timekeep_rs::AtomicInterval;
    /// 
    /// let interval1 = AtomicInterval::closed(1, 5);
    /// let interval2 = AtomicInterval::closed(4, 7);
    /// let merged = AtomicInterval::union(&interval1, &interval2);
    /// 
    /// assert_eq!(merged.len(), 1);
    /// assert_eq!(merged.first().unwrap(), &AtomicInterval::closed(1, 7));
    /// ```
    /// 
    pub fn union(a: &AtomicInterval<T>, b: &AtomicInterval<T>) -> Vec<AtomicInterval<T>> {
        if a.is_overlapping(b) || a.is_adjacent(b) {
            let left = if a.left.value() <= b.left.value() {
                a.left.clone()
            } else {
                b.left.clone()
            };
            let right = if a.right.value() >= b.right.value() {
                a.right.clone()
            } else {
                b.right.clone()
            };
            vec![AtomicInterval { left, right }]
        } else {
            vec![]
        }
    }

    /// Computes the intersection of two overlapping intervals.
    /// The intersection of two intervals is the largest interval that is contained within both intervals.
    /// 
    /// # Arguments
    /// * `other` - The other interval to intersect with the current interval
    /// 
    /// # Returns
    /// A `Vec` containing the intersection of the two intervals if they are overlapping, an empty `Vec` otherwise
    /// 
    /// # Examples
    /// ```
    /// use timekeep_rs::AtomicInterval;
    /// 
    /// let interval1 = AtomicInterval::closed(1, 5);
    /// let interval2 = AtomicInterval::closed(3, 7);
    /// let intersection = interval1.intersection(&interval2);
    /// 
    /// assert_eq!(intersection.len(), 1);
    /// assert_eq!(intersection.first().unwrap(), &AtomicInterval::closed(3, 5));
    /// ```
    /// 
    pub fn intersection(&self, other: &Self) -> Vec<Self> {
        // If they're disjoint, there's no intersection.
        if self.is_disjoint(other) {
            return vec![];
        }

        // Determine the left boundary of the intersection.
        let left = if self.left.value() > other.left.value() {
            self.left.clone()
        } else {
            other.left.clone()
        };

        // Determine the right boundary of the intersection.
        let right = if self.right.value() < other.right.value() {
            self.right.clone()
        } else {
            other.right.clone()
        };

        // If they meet at a single point, ensure it's included on both sides.
        if left.value() == right.value() {
            return match (left, right) {
                (Bound::Included(val), Bound::Included(_)) => {
                    vec![ AtomicInterval { left: Bound::Included(val.clone()), right: Bound::Included(val) } ]
                }
                _ => vec![],
            };
        }

        // Otherwise, we have a valid overlapping range.
        vec![ AtomicInterval { left, right } ]
    }

    /// Computes the difference between two intervals.
    /// The difference between two intervals is the set of intervals that are in the first interval but not in the second interval.
    /// 
    /// # Arguments
    /// * `other` - The other interval to compute the difference with the current interval
    /// 
    /// # Returns
    /// A `Vec` of `AtomicInterval` representing the difference between the two intervals
    /// 
    /// # Examples
    /// ```
    /// use timekeep_rs::AtomicInterval;
    /// 
    /// let interval1 = AtomicInterval::closed(1, 5);
    /// let interval2 = AtomicInterval::closed(3, 7);
    /// let difference = interval1.difference(&interval2);
    /// assert_eq!(difference.len(), 1);
    /// assert_eq!(difference[0], AtomicInterval::closed_open(1, 3));
    /// ```
    /// 
    pub fn difference(&self, other: &Self) -> Vec<Self> {
        // If disjoint, difference is just self.
        if self.is_disjoint(other) {
            return vec![self.clone()];
        } else if self.is_subset(other) {
            return vec![];
        }

        // If there's no intersection, difference is self.
        let intersection_vec = self.intersection(other);
        let intersection = intersection_vec.first().expect("No intersection found!");

        let mut result = Vec::new();

        // Left remainder: from self.left up to intersection.left (if any).
        if intersection.left.value() > self.left.value() {
            let left_interval = AtomicInterval {
                left: self.left.clone(),
                right: match &intersection.left {
                    Bound::Included(val) => Bound::Excluded(val.clone()),
                    Bound::Excluded(val) => Bound::Excluded(val.clone()),
                },
            };
            // Only add if valid (left <= right).
            if left_interval.left.value() < left_interval.right.value() {
                result.push(left_interval);
            }
        }

        // Right remainder: from intersection.right up to self.right (if any).
        if intersection.right.value() < self.right.value() {
            let right_interval = AtomicInterval {
                left: match &intersection.right {
                    Bound::Included(val) => Bound::Excluded(val.clone()),
                    Bound::Excluded(val) => Bound::Excluded(val.clone()),
                },
                right: self.right.clone(),
            };
            // Only add if valid (left <= right).
            if right_interval.left.value() < right_interval.right.value() {
                result.push(right_interval);
            }
        }

        result
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_interval() {
        let interval = AtomicInterval::open(1, 5);
        assert_eq!(interval.left, Bound::Excluded(1));
        assert_eq!(interval.right, Bound::Excluded(5));
    }

    #[test]
    fn test_closed_interval() {
        let interval = AtomicInterval::closed(1, 5);
        assert_eq!(interval.left, Bound::Included(1));
        assert_eq!(interval.right, Bound::Included(5));
    }

    #[test]
    fn test_open_closed_interval() {
        let interval = AtomicInterval::open_closed(1, 5);
        assert_eq!(interval.left, Bound::Excluded(1));
        assert_eq!(interval.right, Bound::Included(5));
    }

    #[test]
    fn test_closed_open_interval() {
        let interval = AtomicInterval::closed_open(1, 5);
        assert_eq!(interval.left, Bound::Included(1));
        assert_eq!(interval.right, Bound::Excluded(5));
    }

    #[test]
    fn test_point_interval() {
        let interval = AtomicInterval::point(1);
        assert_eq!(interval.left, Bound::Included(1));
        assert_eq!(interval.right, Bound::Included(1));
    }

    #[test]
    fn test_is_overlapping() {
        let interval1 = AtomicInterval::closed(1, 5);
        let interval2 = AtomicInterval::closed(4, 6);
        assert!(interval1.is_overlapping(&interval2));
    }

    #[test]
    fn test_is_adjacent() {
        let interval1 = AtomicInterval::closed(1, 5);
        let interval2 = AtomicInterval::open_closed(5, 10);
        assert!(interval1.is_adjacent(&interval2));
    }

    #[test]
    fn test_is_disjoint() {
        let interval1 = AtomicInterval::closed(1, 5);
        let interval2 = AtomicInterval::closed(6, 10);
        assert!(interval1.is_disjoint(&interval2));
    }

    #[test]
    fn test_is_subset() {
        let interval1 = AtomicInterval::closed(2, 4);
        let interval2 = AtomicInterval::closed(1, 5);
        assert!(interval1.is_subset(&interval2));
    }

    #[test]
    fn test_is_superset() {
        let interval1 = AtomicInterval::closed(1, 5);
        let interval2 = AtomicInterval::closed(2, 4);
        assert!(interval1.is_superset(&interval2));
    }

    #[test]
    fn test_union_overlapping_intervals() {
        let interval1 = AtomicInterval::closed(1, 5);
        let interval2 = AtomicInterval::closed(4, 7);
        let merged = AtomicInterval::union(&interval1, &interval2);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged.first().unwrap(), &AtomicInterval::closed(1, 7));
    }

    #[test]
    fn test_union_adjacent_intervals() {
        let interval1 = AtomicInterval::closed(1, 5);
        let interval2 = AtomicInterval::closed(5, 7);
        let merged = AtomicInterval::union(&interval1, &interval2);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged.first().unwrap(), &AtomicInterval::closed(1, 7));
    }

    #[test]
    fn test_union_disjoint_intervals() {
        let interval1 = AtomicInterval::closed(1, 5);
        let interval2 = AtomicInterval::closed(6, 7);
        let merged = AtomicInterval::union(&interval1, &interval2);
        assert_eq!(merged.len(), 0);
    }

    #[test]
    fn test_intersection_between_two_overlapping_intervals() {
        let interval1 = AtomicInterval::closed(1, 5);
        let interval2 = AtomicInterval::closed(3, 7);
        let intersection = interval1.intersection(&interval2);
        assert_eq!(intersection.len(), 1);
        assert_eq!(intersection.first().unwrap(), &AtomicInterval::closed(3, 5));
    }

    #[test]
    fn test_intersection_between_two_disjoint_intervals() {
        let interval1 = AtomicInterval::closed(1, 3);
        let interval2 = AtomicInterval::closed(4, 7);
        let intersection = interval1.intersection(&interval2);
        assert_eq!(intersection.len(), 0);
    }

    #[test]
    fn test_intersection_between_two_adjacent_intervals() {
        let interval1 = AtomicInterval::closed(1, 5);
        let interval2 = AtomicInterval::open(5, 7);
        let intersection = interval1.intersection(&interval2);
        assert_eq!(intersection.len(), 0);
    }

    #[test]
    fn test_difference_between_two_overlapping_intervals() {
        let interval1 = AtomicInterval::closed(1, 5);
        let interval2 = AtomicInterval::closed(3, 7);
        let difference = interval1.difference(&interval2);
        assert_eq!(difference.len(), 1);
        assert_eq!(difference[0], AtomicInterval::closed_open(1, 3));
    }

    #[test]
    fn test_difference_between_subset_and_superset_interval() {
        let interval1 = AtomicInterval::closed(1, 5);
        let interval2 = AtomicInterval::closed(2, 4);
        let difference = interval1.difference(&interval2);
        assert_eq!(difference.len(), 2);
        assert_eq!(difference[0], AtomicInterval::closed_open(1, 2));
        assert_eq!(difference[1], AtomicInterval::open_closed(4, 5));
    }

    #[test]
    fn test_difference_between_two_disjoint_intervals() {
        let interval1 = AtomicInterval::closed(1, 3);
        let interval2 = AtomicInterval::closed(4, 7);
        let difference = interval1.difference(&interval2);
        assert_eq!(difference.len(), 1);
        assert_eq!(difference[0], AtomicInterval::closed(1, 3));
    }

    #[test]
    fn test_difference_between_two_adjacent_intervals() {
        let interval1 = AtomicInterval::closed(1, 5);
        let interval2 = AtomicInterval::open(5, 7);
        let difference = interval1.difference(&interval2);
        assert_eq!(difference.len(), 1);
        assert_eq!(difference[0], AtomicInterval::closed(1, 5));
    }
}
