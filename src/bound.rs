#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum Bound<T> {
    Included(T),  // Closed boundary [
    Excluded(T),  // Open boundary (
}

impl<T> Bound<T> {
    pub fn included(value: T) -> Self {
        Bound::Included(value)
    }

    pub fn excluded(value: T) -> Self {
        Bound::Excluded(value)
    }

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
        match bound {
            Bound::Included(value) => assert_eq!(value, 5),
            _ => panic!("Expected Bound::Included"),
        }
    }

    #[test]
    fn test_excluded() {
        let bound = Bound::excluded(10);
        match bound {
            Bound::Excluded(value) => assert_eq!(value, 10),
            _ => panic!("Expected Bound::Excluded"),
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
