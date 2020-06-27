/// Even numbers in an array
/// https://www.codewars.com/kata/5a431c0de1ce0ec33a00000c/train/rust

pub fn even_numbers(array: &Vec<i32>, number: usize) -> Vec<i32> {
    array
        .iter()
        .filter(|&x| x % 2 == 0)
        .rev()
        .take(number)
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .copied()
        .copied()
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {

    use super::even_numbers;
    #[test]
    fn sample_tests() {
        assert_eq!(
            even_numbers(&vec!(1, 2, 3, 4, 5, 6, 7, 8, 9), 3),
            vec!(4, 6, 8)
        );
        assert_eq!(
            even_numbers(&vec!(-22, 5, 3, 11, 26, -6, -7, -8, -9, -8, 26), 2),
            vec!(-8, 26)
        );
        assert_eq!(
            even_numbers(&vec!(6, -25, 3, 7, 5, 5, 7, -3, 23), 1),
            vec!(6)
        );
    }
}
