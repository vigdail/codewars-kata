// A Rule of Divisibility by 13
// https://www.codewars.com/kata/564057bc348c7200bd0000ff/train/rust

pub fn thirt(n: i64) -> i64 {
    let x = [1, 10, 9, 12, 3, 4];
    let mut r = n;
    let mut prev = 0;
    while r != prev {
        prev = r;
        r = r
            .to_string()
            .chars()
            .rev()
            .zip(x.iter().cycle())
            .fold(0, |acc, (a, b)| {
                acc + a.to_digit(10).unwrap_or(0) as i64 * b
            });
    }

    r
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testequal(n: i64, exp: i64) -> () {
        assert_eq!(exp, thirt(n))
    }

    #[test]
    fn basics_thirt() {
        testequal(8529, 79);
        testequal(85299258, 31);
        testequal(5634, 57);
    }
}
