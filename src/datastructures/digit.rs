
#[derive(Debug,Clone,PartialEq,Hash)]
pub enum Digit {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}

pub type DigitValue = Option<Digit>;


#[cfg(test)]
mod tests {
    use super::DigitValue;
    use super::Digit::*;

    #[test]
    fn test_if_digit_can_be_none() {
        let myvalue: DigitValue = Some(One);
        assert_eq!(myvalue, Some(One));
    }
}
