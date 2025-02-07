use crate::Bound;

#[derive(PartialEq, Debug, Clone)]
pub struct AtomicInterval<T> {
    pub left: Bound<T>,
    pub right: Bound<T>,
}

impl<T: ToString> ToString for AtomicInterval<T> {
    fn to_string(&self) -> String {
        match (&self.left, &self.right) {
            (Bound::Included(l), Bound::Included(r)) => format!("[{}, {}]", l.to_string(), r.to_string()),
            (Bound::Included(l), Bound::Excluded(r)) => format!("[{}, {})", l.to_string(), r.to_string()),
            (Bound::Excluded(l), Bound::Included(r)) => format!("({}, {}]", l.to_string(), r.to_string()),
            (Bound::Excluded(l), Bound::Excluded(r)) => format!("({}, {})", l.to_string(), r.to_string()),
        }
    }
}


impl<T: Clone> AtomicInterval<T> {
    pub fn open(left: T, right: T) -> Self {
        AtomicInterval { left: Bound::Excluded(left), right: Bound::Excluded(right) }
    }

    pub fn closed(left: T, right: T) -> Self {
        AtomicInterval { left: Bound::Included(left), right: Bound::Included(right) }
    }

    pub fn open_closed(left: T, right: T) -> Self {
        AtomicInterval { left: Bound::Excluded(left), right: Bound::Included(right) }
    }

    pub fn closed_open(left: T, right: T) -> Self {
        AtomicInterval { left: Bound::Included(left), right: Bound::Excluded(right) }
    }

    pub fn point(value: T) -> Self {
        AtomicInterval { left: Bound::Included(value.clone()), right: Bound::Included(value) }
    }
}

impl <T: PartialOrd> AtomicInterval<T> {
    pub fn is_superset (&self, other: &AtomicInterval<T>) -> bool {
        match (&self.left, &self.right, &other.left, &other.right) {
            (Bound::Included(l1), Bound::Excluded(r1), _, Bound::Included(r2)) => l1 <= other.left.value() && r1 > r2,
            (Bound::Excluded(l1), Bound::Included(r1), Bound::Included(l2), _) => l1 < l2 && r1 >= other.right.value(),
            (Bound::Excluded(l1), Bound::Excluded(r1), Bound::Included(l2), Bound::Included(r2)) => l1 < l2 && r1 > r2,
            (_, _, _, _) => self.left.value() <= other.left.value() && self.right.value() >= other.right.value(),
        }
    }

    pub fn is_subset (&self, other: &AtomicInterval<T>) -> bool {
        other.is_superset(self)
    }

    pub fn is_overlapping (&self, other: &AtomicInterval<T>) -> bool {

        let cond1_overlapping = match (&self.left, &self.right, &other.left) {
            (Bound::Included(l1), Bound::Included(r1), _) => other.left.value() >= l1 && other.left.value() <= r1,
            (Bound::Included(l1), Bound::Excluded(r1), Bound::Included(l2)) => l2 >= l1 && l2 < r1,
            (Bound::Included(l1), Bound::Excluded(r1), Bound::Excluded(l2)) => l2 >= l1 && l2 <= r1,
            (Bound::Excluded(l1), Bound::Included(r1), Bound::Included(l2)) => l2 > l1 && l2 <= r1,
            (Bound::Excluded(l1), Bound::Included(r1), Bound::Excluded(l2)) => l2 >= l1 && l2 <= r1,
            (Bound::Excluded(l1), Bound::Excluded(r1), Bound::Included(l2)) => l2 > l1 && l2 < r1,
            (Bound::Excluded(l1), Bound::Excluded(r1), Bound::Excluded(l2)) => l2 >= l1 && l2 <= r1,
        };
        let cond2_overlapping = match (&self.left, &self.right, &other.right) {
            (Bound::Included(l1), Bound::Included(r1), _) => other.right.value() >= l1 && other.right.value() <= r1,
            (Bound::Included(l1), Bound::Excluded(r1), Bound::Included(r2)) => r2 > l1 && r2 <= r1,
            (Bound::Included(l1), Bound::Excluded(r1), Bound::Excluded(r2)) => r2 >= l1 && r2 <= r1,
            (Bound::Excluded(l1), Bound::Included(r1), Bound::Included(r2)) => r2 >= l1 && r2 < r1,
            (Bound::Excluded(l1), Bound::Included(r1), Bound::Excluded(r2)) => r2 >= l1 && r2 <= r1,
            (Bound::Excluded(l1), Bound::Excluded(r1), Bound::Included(r2)) => r2 > l1 && r2 < r1,
            (Bound::Excluded(l1), Bound::Excluded(r1), Bound::Excluded(r2)) => r2 >= l1 && r2 <= r1,
        };

        return cond1_overlapping || cond2_overlapping;
    }

    pub fn is_adjacent(&self, other: &AtomicInterval<T>) -> bool {
        let cond1_adjacent = match (&self.left, &other.right) {
            (Bound::Excluded(_), Bound::Excluded(_)) => false,
            (_, _) => self.right.value() == other.left.value(),
        };

        let cond2_adjacent = match (&self.right, &other.left) {
            (Bound::Excluded(_), Bound::Excluded(_)) => false,
            (_, _) => self.left.value() == other.right.value(),
        };

        return cond1_adjacent || cond2_adjacent;
    }

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
    pub fn union(a: &AtomicInterval<T>, b: &AtomicInterval<T>) -> Option<AtomicInterval<T>> {
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
            Some(AtomicInterval { left, right })
        } else {
            None
        }
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        // If they're disjoint, there's no intersection.
        if self.is_disjoint(other) {
            return None;
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
                    Some(AtomicInterval { left: Bound::Included(val.clone()), right: Bound::Included(val) })
                }
                _ => None,
            };
        }

        // Otherwise, we have a valid overlapping range.
        Some(AtomicInterval { left, right })
    }

    pub fn difference(&self, other: &Self) -> Vec<Self> {
        // If disjoint, difference is just self.
        if self.is_disjoint(other) {
            return vec![self.clone()];
        } else if self.is_subset(other) {
            return vec![];
        }

        // If there's no intersection, difference is self.
        let intersection = match self.intersection(other) {
            Some(i) => i,
            None => panic!("No intersection found!"),
        };

        // // If other covers self completely, no remainder.
        // if intersection == *self {
        //     return vec![];
        // }

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
        let interval2 = AtomicInterval::closed(5, 10);
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
        let merged = AtomicInterval::union(&interval1, &interval2).unwrap();
        assert_eq!(merged, AtomicInterval::closed(1, 7));
    }

    #[test]
    fn test_union_adjacent_intervals() {
        let interval1 = AtomicInterval::closed(1, 5);
        let interval2 = AtomicInterval::closed(5, 7);
        let merged = AtomicInterval::union(&interval1, &interval2).unwrap();
        assert_eq!(merged, AtomicInterval::closed(1, 7));
    }

    #[test]
    fn test_union_disjoint_intervals() {
        let interval1 = AtomicInterval::closed(1, 5);
        let interval2 = AtomicInterval::closed(6, 7);
        let merged = AtomicInterval::union(&interval1, &interval2);
        assert_eq!(merged, None);
    }

    #[test]
    fn test_intersection_between_two_overlapping_intervals() {
        let interval1 = AtomicInterval::closed(1, 5);
        let interval2 = AtomicInterval::closed(3, 7);
        let intersection = interval1.intersection(&interval2).unwrap();
        assert_eq!(intersection, AtomicInterval::closed(3, 5));
    }

    #[test]
    fn test_intersection_between_two_disjoint_intervals() {
        let interval1 = AtomicInterval::closed(1, 3);
        let interval2 = AtomicInterval::closed(4, 7);
        let intersection = interval1.intersection(&interval2);
        assert!(intersection.is_none());
    }

    #[test]
    fn test_intersection_between_two_adjacent_intervals() {
        let interval1 = AtomicInterval::closed(1, 5);
        let interval2 = AtomicInterval::open(5, 7);
        let intersection = interval1.intersection(&interval2);
        assert!(intersection.is_none());
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
