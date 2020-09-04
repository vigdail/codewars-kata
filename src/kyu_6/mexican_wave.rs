// Mexican Wave
// https://www.codewars.com/kata/58f5c63f1e26ecda7e000029/train/rust

pub fn wave(s: &str) -> Vec<String> {
    s.char_indices()
        .map(|(i, c)| s[..i].to_string() + &c.to_uppercase().to_string() + &s[i + 1..])
        .filter(|wave| wave != s)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let expect = ["Hello", "hEllo", "heLlo", "helLo", "hellO"];
        assert_eq!(wave("hello"), expect);

        let expect = [
            "Codewars", "cOdewars", "coDewars", "codEwars", "codeWars", "codewArs", "codewaRs",
            "codewarS",
        ];
        assert_eq!(wave("codewars"), expect);
        let expect: [&str; 0] = [];
        assert_eq!(wave(""), expect);

        let expect = [
            "Two words",
            "tWo words",
            "twO words",
            "two Words",
            "two wOrds",
            "two woRds",
            "two worDs",
            "two wordS",
        ];
        assert_eq!(wave("two words"), expect);
        let expect = [" Gap ", " gAp ", " gaP "];
        assert_eq!(wave(" gap "), expect);
    }
}
