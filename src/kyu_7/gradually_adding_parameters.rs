/// https://www.codewars.com/kata/555b73a81a6285b6ce000047/train/rust
/// Gradually Adding Parameters
pub fn add(args: &[i64]) -> i64 {
    args.iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + (i + 1) as i64 * x)
}

#[cfg(test)]
mod test {
    use super::add;
    #[test]
    fn basic_tests() {
        assert_eq!(add(&[]), 0);
        assert_eq!(add(&[4, -3, -2]), -8);
    }
}
