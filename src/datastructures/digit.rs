use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

pub type DigitValue = Option<Digit>;
pub type DigitHashSet = HashSet<Digit>;

pub struct DigitSet {
    data: Vec<DigitValue>,
}

impl DigitSet {
    pub fn new() -> Self {
        DigitSet { data: Vec::new() }
    }
    pub fn new_full() -> Self {
        use Digit::*;

        DigitSet {
            data: vec![
                Some(One),
                Some(Two),
                Some(Three),
                Some(Four),
                Some(Five),
                Some(Six),
                Some(Seven),
                Some(Eight),
                Some(Nine),
            ],
        }
    }
    pub fn new_full_and_rotate_left(times: usize) -> Self {
        use Digit::*;

        let mut data = vec![
            Some(One),
            Some(Two),
            Some(Three),
            Some(Four),
            Some(Five),
            Some(Six),
            Some(Seven),
            Some(Eight),
            Some(Nine),
        ];
        data.rotate_left(times);
        DigitSet {
            data: data,
        }    
    }
    pub fn contains(&self, digit: Digit) -> bool {
        self.data.contains(&Some(digit))
    }
    pub fn rotate_right(&mut self, times: usize) {
        self.data.rotate_right(times)
    }
    pub fn rotate_left(&mut self, times: usize) {
        self.data.rotate_left(times)
    }
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a DigitValue> + 'a {
        self.data.iter()
    }
    pub fn new_hashset(&self) -> DigitHashSet {
        self.data
            .iter()
            .filter(|d| d.is_some())
            .map(|d| d.unwrap())
            .collect()
    }
    pub fn get_data(&self) -> Vec<DigitValue> {
        self.data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::{Digit, Digit::*, DigitSet, DigitValue};

    #[test]
    fn test_if_digit_can_be_none() {
        let myvalue: DigitValue = Some(One);
        assert_eq!(myvalue, Some(One));
    }
    #[test]
    fn test_digit_set_contains_nothing() {
        let digitset = DigitSet::new();
        assert_eq!(digitset.contains(One), false);
    }
    #[test]
    fn test_digit_set_contains_all() {
        let digitset = DigitSet::new_full();
        assert_eq!(digitset.contains(One), true);
        assert_eq!(digitset.contains(Two), true);
        assert_eq!(digitset.contains(Three), true);
        assert_eq!(digitset.contains(Four), true);
        assert_eq!(digitset.contains(Five), true);
        assert_eq!(digitset.contains(Six), true);
        assert_eq!(digitset.contains(Seven), true);
        assert_eq!(digitset.contains(Eight), true);
        assert_eq!(digitset.contains(Nine), true);
    }
    #[test]
    fn test_digit_set_rotation() {
        let mut digitset = DigitSet::new_full();
        digitset.rotate_right(3);
        assert_eq!(*digitset.iter().next().unwrap(), Some(Seven));
        digitset.rotate_left(3);
        assert_eq!(*digitset.iter().next().unwrap(), Some(One));
    }
    #[test]
    fn test_digit_into_hashset() {
        let hashset = DigitSet::new_full().new_hashset();
        assert_eq!(hashset.iter().count(), 9);
    }
}
