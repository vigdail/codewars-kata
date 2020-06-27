/// Build a square
/// https://www.codewars.com/kata/59a96d71dbe3b06c0200009c/train/rust

pub fn generate_shape(n: i32) -> String {
    (0..n)
        .map(|_| "+".repeat(n as usize))
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod test {
    use super::generate_shape;
    #[test]
    fn sample_test() {
        assert_eq!(generate_shape(3), "+++\n+++\n+++");
    }
}
