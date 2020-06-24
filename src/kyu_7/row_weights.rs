/// Row Weights
/// https://www.codewars.com/kata/5abd66a5ccfd1130b30000a9/train/rust

fn row_weights(array: Vec<u32>) -> (u32, u32) {
    (
        array.iter().step_by(2).sum(),
        array.iter().skip(1).step_by(2).sum(),
    )
}

#[cfg(test)]
mod test {
    use super::row_weights;

    #[test]
    fn basic_tests() {
        assert_eq!(row_weights(vec![13, 27, 49]), (62, 27));
        assert_eq!(row_weights(vec![50, 60, 70, 80]), (120, 140));
        assert_eq!(row_weights(vec![80]), (80, 0));
    }
}
