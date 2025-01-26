// impl<T: PartialOrd + Clone + PartialEq> From<AtomicInterval<T>> for Interval<T> {
//     fn from(interval: AtomicInterval<T>) -> Self {
//         Interval {
//             intervals: vec![interval],
//         }
//     }
// }


// pub struct Interval<T: PartialOrd + Clone + PartialEq> {
//     pub intervals: Vec<AtomicInterval<T>>,
// }

// impl <T: PartialOrd + Clone + PartialEq> Interval<T> {
//     pub fn new() -> Self {
//         Interval { intervals: Vec::new() }
//     }

//     pub fn add(&mut self, interval: AtomicInterval<T>) {
//         self.intervals.push(interval);
//     }

//     fn contains(&self, value: &Self) -> bool {
//         self.intervals.iter().any(
//             |this| value.intervals.iter().any(
//                 |other| this == other
//             )
//         )
//     }

//     pub fn intersection(&self, other: &Self) -> Option<Self> {
//         let mut result = Interval::new();

//         for interval in &self.intervals {
//             for other_interval in &other.intervals {
//                 if let Some(intersection) = interval.intersection(other_interval) {
//                     result.add(intersection);
//                 }
//             }
//         }

//         if result.intervals.is_empty() {
//             None
//         } else {
//             Some(result)
//         }
//     }

//     pub fn union(&self, other: &Self) -> Self {
//         let mut result = Interval::new();

//         for interval in &self.intervals {
//             result.add(interval.clone());
//         }

//         for interval in &other.intervals {
//             result.add(interval.clone());
//         }

//         result
//     }

//     pub fn difference(&self, other: &Self) -> Vec<Self> {
//         let mut result = Vec::new();

//         for interval in &self.intervals {
//             let mut temp = interval.clone();
//             for other_interval in &other.intervals {
//                 let mut new_temp = Interval::new();
//                 for diff in temp.difference(other_interval) {
//                     new_temp.add(diff);
//                 }
//                 temp = new_temp;
//             }
//             for diff in temp.intervals {
//                 result.push(Interval { intervals: vec![diff] });
//             }
//         }

//         result
//     }
// }

// pub struct Spans<T: PartialOrd + Clone> {
//     pub intervals: Vec<Interval<T>>,
// }

// impl <T: PartialOrd + Clone> Spans<T> {
//     pub fn new() -> Self {
//         Spans { intervals: Vec::new() }
//     }

//     pub fn add(&mut self, interval: Interval<T>) {
//         self.intervals.push(interval);
//     }

//     pub fn contains(&self, value: &T) -> bool {
//         self.intervals.iter().any(|interval| interval.contains(value))
//     }
// }