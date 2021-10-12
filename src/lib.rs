pub mod datastructures;


#[cfg(test)]
mod tests {
    use super::datastructures::digit::DigitValue;

    #[test]
    fn test_if_digit_can_be_none() {
        use super::datastructures::digit::Digit::*;
        let myvalue: DigitValue = Some(One);
        assert_eq!(myvalue, Some(One));
    }
}
