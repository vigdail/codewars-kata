/// Operations with sequence
/// https://www.codewars.com/kata/596ddaccdd42c1cf0e00005c/train/rust

fn calc(array: Vec<i32>) -> i32 {
    array
        .iter()
        .enumerate()
        .map(|(i, &x)| {
            let mut r = x;
            if x > 0 {
                r = x * x;
            }
            if (i + 1) % 3 == 0 {
                r *= 3;
            }
            if (i + 1) % 5 == 0 {
                r *= -1;
            }
            r
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::calc;
    #[test]
    fn tests() {
        assert_eq!(calc(vec![0, 2, 1, -6, -3, 3]), 31);
        assert_eq!(calc(vec![0]), 0);
        assert_eq!(calc(vec![1, 1, 1, 1, 1]), 5);
        assert_eq!(calc(vec![1, 1, -9, 9, 16, -15, -45, -73, 26]), 1665);
        assert_eq!(calc(vec![1, -1, 10, -9, 16, 15, 45, -73, -26]), 2584);
        assert_eq!(calc(vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 0);
        assert_eq!(calc(vec![-5, -5, -5, -5, -5, -5, -5]), -45);
    }
}
