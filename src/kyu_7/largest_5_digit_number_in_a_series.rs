// Largest 5 digit number in a series
// https://www.codewars.com/kata/51675d17e0c1bed195000001/train/rust

pub fn largest_five_digit_number(num: &str) -> u32 {
    num.chars()
        .collect::<Vec<_>>()
        .windows(5)
        .fold("".to_string(), |max, x| {
            let n = x.iter().collect::<String>();
            if max < n {
                n
            } else {
                max
            }
        })
        .parse()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(largest_five_digit_number(&"1234567890"), 67890);
    }
}
