#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
/// Represents a boundary of an interval.
/// Can be either inclusive (closed) or exclusive (open).
pub enum Bound<T> {
    /// Represents an inclusive boundary, meaning the value itself is included in the interval.
    Included(T),
    /// Represents an exclusive boundary, meaning the value itself is excluded from the interval.
    Excluded(T),
}

impl<T> Bound<T> {
    /// Creates a new `Bound` that is inclusive.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be included in the boundary.
    ///
    /// # Examples
    ///
    /// ```
    /// use timekeep_rs::Bound;
    ///
    /// let inclusive_bound = Bound::included(5);
    /// assert_eq!(inclusive_bound, Bound::Included(5));
    /// ```
    pub fn included(value: T) -> Self {
        Bound::Included(value)
    }

    /// Creates a new `Bound` that is exclusive.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to be excluded from the boundary.
    ///
    /// # Examples
    ///
    /// ```
    /// use timekeep_rs::Bound;
    ///
    /// let exclusive_bound = Bound::excluded(10);
    /// assert_eq!(exclusive_bound, Bound::Excluded(10));
    /// ```
    pub fn excluded(value: T) -> Self {
        Bound::Excluded(value)
    }

    /// Returns a reference to the value contained within the `Bound`.
    ///
    /// # Examples
    ///
    /// ```
    /// use timekeep_rs::Bound;
    ///
    /// let inclusive_bound = Bound::included(15);
    /// assert_eq!(inclusive_bound.value(), &15);
    ///
    /// let exclusive_bound = Bound::excluded(20);
    /// assert_eq!(exclusive_bound.value(), &20);
    /// ```
    pub fn value(&self) -> &T {
        match self {
            Bound::Included(value) => value,
            Bound::Excluded(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Bound;

    #[test]
    fn test_included() {
        let bound = Bound::included(5);
        assert_eq!(bound, Bound::Included(5));
        if let Bound::Included(value) = bound {
            assert_eq!(value, 5);
        } else {
            panic!("Expected Bound::Included");
        }
    }

    #[test]
    fn test_excluded() {
        let bound = Bound::excluded(10);
        assert_eq!(bound, Bound::Excluded(10));
        if let Bound::Excluded(value) = bound {
            assert_eq!(value, 10);
        } else {
            panic!("Expected Bound::Excluded");
        }
    }

    #[test]
    fn test_value_included() {
        let bound = Bound::included(15);
        assert_eq!(bound.value(), &15);
    }

    #[test]
    fn test_value_excluded() {
        let bound = Bound::excluded(20);
        assert_eq!(bound.value(), &20);
    }

    #[test]
    fn test_partial_eq() {
        let bound1 = Bound::included(25);
        let bound2 = Bound::included(25);
        assert_eq!(bound1, bound2);

        let bound3 = Bound::excluded(30);
        let bound4 = Bound::excluded(30);
        assert_eq!(bound3, bound4);

        let bound5 = Bound::included(35);
        let bound6 = Bound::excluded(35);
        assert_ne!(bound5, bound6);
    }

    #[test]
    fn test_partial_ord() {
        let bound1 = Bound::included(5);
        let bound2 = Bound::included(10);
        assert!(bound1 < bound2);

        let bound3 = Bound::excluded(15);
        let bound4 = Bound::excluded(20);
        assert!(bound3 < bound4);

        let bound5 = Bound::included(25);
        let bound6 = Bound::excluded(25);
        assert!(bound5 < bound6);
    }
}