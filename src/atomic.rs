use crate::Bound;

#[derive(PartialEq)]
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

impl <T: PartialOrd + Clone> AtomicInterval<T> {
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
}
