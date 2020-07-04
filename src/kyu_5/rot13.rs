// Rot13
// https://www.codewars.com/kata/530e15517bc88ac656000716/train/rust

pub fn rot13(message: &str) -> String {
    message
        .chars()
        .map(|c| match c {
            'a'..='z' => ((c as u8 + 13 - 'a' as u8) % 26 + 'a' as u8) as char,
            'A'..='Z' => ((c as u8 + 13 - 'A' as u8) % 26 + 'A' as u8) as char,
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(rot13("test"), "grfg");
        assert_eq!(rot13("Test"), "Grfg");
    }
}
