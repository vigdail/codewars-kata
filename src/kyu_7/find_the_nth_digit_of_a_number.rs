/// https://www.codewars.com/kata/577b9960df78c19bca00007e/train/rust
/// Find the nth Digit of a Number

pub fn find_digit(num: i32, nth: i32) -> i32 {
    if nth <= 0 {
        return -1;
    }
    num.abs()
        .to_string()
        .chars()
        .rev()
        .nth((nth - 1) as usize)
        .and_then(|x| x.to_digit(10))
        .unwrap_or(0) as i32
}

#[cfg(test)]
mod test {
    use super::find_digit;

    #[test]
    fn example_test() {
        assert_eq!(find_digit(5673, 4), 5);
        assert_eq!(find_digit(129, 2), 2);
        assert_eq!(find_digit(-2825, 3), 8);
        assert_eq!(find_digit(-456, 4), 0);
        assert_eq!(find_digit(0, 20), 0);
        assert_eq!(find_digit(65, 0), -1);
        assert_eq!(find_digit(24, -8), -1);
    }
}
