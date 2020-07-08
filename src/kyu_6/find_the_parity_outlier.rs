// Find The Parity Outlier
// https://www.codewars.com/kata/5526fc09a1bbd946250002dc/train/rust

pub fn find_outlier(values: &[i32]) -> i32 {
    let mut odds = 0;
    let mut evens = 0;
    let mut last_even = 0;
    let mut last_odd = 0;
    for i in values {
        if i % 2 == 0 {
            evens += 1;
            last_even = *i;
        } else {
            odds += 1;
            last_odd = *i;
        }
        if evens == 1 && odds > 1 {
            return last_even;
        }
        if odds == 1 && evens > 1 {
            return last_odd;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let t1 = [2, 6, 8, -10, 3];
        let t2 = [
            206847684, 1056521, 7, 17, 1901, 21104421, 7, 1, 35521, 1, 7781,
        ];
        let t3 = [std::i32::MAX, 0, 1];
        assert_eq!(3, find_outlier(&t1));
        assert_eq!(206847684, find_outlier(&t2));
        assert_eq!(0, find_outlier(&t3));
    }
}
