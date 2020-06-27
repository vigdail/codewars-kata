pub fn switcher(numbers: Vec<&str>) -> String {
    numbers
        .iter()
        .map(|s| match s.parse::<u8>().unwrap() {
            n @ 1..=26 => ('a' as u8 + 26 - n) as char,
            27 => '!',
            28 => '?',
            29 => ' ',
            _ => unreachable!(),
        })
        .collect()
}
// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::switcher;

    #[test]
    fn example_tests() {
        assert_eq!(
            switcher(vec!["24", "12", "23", "22", "4", "26", "9", "8"]),
            "codewars"
        );
        assert_eq!(
            switcher(vec![
                "25", "7", "8", "4", "14", "23", "8", "25", "23", "29", "16", "16", "4"
            ]),
            "btswmdsbd kkw"
        );
        assert_eq!(switcher(vec!["4", "24"]), "wc");
    }
}
