/// Convert a Number to a String
/// https://www.codewars.com/kata/5265326f5fda8eb1160004c8/train/rust

pub fn number_to_string(i: i32) -> String {
    i.to_string()
}

#[cfg(test)]
mod tests {
    use super::number_to_string;

    #[test]
    fn returns_number_as_a_string() {
        assert_eq!(number_to_string(67), "67");
        assert_eq!(number_to_string(1 + 2), "3");
    }
}
