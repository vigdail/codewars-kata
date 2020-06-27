/// https://www.codewars.com/kata/55e6f5e58f7817808e00002e/train/rust
/// A Rule of Divisibility by 7
pub fn seven(n: i64) -> (i64, i32) {
    if n == 0 {
        return (0, 0);
    }
    let mut n = n;
    let i = (0..)
        .take_while(|_| {
            if n < 100 {
                return false;
            }
            let s = n.to_string();
            let x = s[0..s.len() - 1].parse::<i64>().unwrap();
            let y = s.chars().last().unwrap().to_digit(10).unwrap() as i64;
            n = x - 2 * y;
            return n >= 100;
        })
        .count() as i32
        + 1;
    (n, i)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i64, exp: (i64, i32)) -> () {
        println!(" n: {:?};", n);
        let ans = seven(n);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(477557101, (28, 7));
        dotest(477557102, (47, 7));
        dotest(1603, (7, 2));
        dotest(0, (0, 0));
    }
}
