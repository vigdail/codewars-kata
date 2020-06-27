// https://www.codewars.com/kata/55eea63119278d571d00006a/train/rust
// Smallest unused ID

pub fn next_id(ids: &[usize]) -> usize {
    (0..=ids.len())
        .skip_while(|x| ids.contains(x))
        .next()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basics() {
        assert_eq!(next_id(&[0, 1, 2, 4, 5]), 3);
        assert_eq!(next_id(&[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 11);
        assert_eq!(next_id(&[1, 0, 2, 3, 5, 6, 10, 7, 4, 8, 10]), 9);
        assert_eq!(next_id(&[1]), 0);
    }
}
