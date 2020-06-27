// Leap Years
// https://www.codewars.com/kata/526c7363236867513f0005ca/train/rust

pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}

//
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sample_tests() {
        assert_eq!(is_leap_year(1234), false);
        assert_eq!(is_leap_year(1984), true);
        assert_eq!(is_leap_year(2000), true);
        assert_eq!(is_leap_year(2010), false);
        assert_eq!(is_leap_year(2013), false);
    }
}
