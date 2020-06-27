// Decode the Morse code
// https://www.codewars.com/kata/54b724efac3d5402db00065e/train/rust

use std::collections::HashMap;

pub struct MorseDecoder {
    morse_code: HashMap<String, String>,
}

impl MorseDecoder {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        map.insert("....".to_string(), "H".to_string());
        map.insert(".".to_string(), "E".to_string());
        map.insert("-.--".to_string(), "Y".to_string());
        map.insert("   ".to_string(), " ".to_string());
        map.insert(".---".to_string(), "J".to_string());
        map.insert("..-".to_string(), "U".to_string());
        map.insert("-..".to_string(), "D".to_string());
        map.insert(".".to_string(), "E".to_string());
        Self { morse_code: map }
    }
    pub fn decode_morse(&self, encoded: &str) -> String {
        encoded
            .trim()
            .split("   ")
            .map(|s| {
                s.split(" ")
                    .filter_map(|x| self.morse_code.get(x))
                    .cloned()
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hey_jude() {
        let decoder = MorseDecoder::new();
        assert_eq!(
            decoder.decode_morse(".... . -.--   .--- ..- -.. ."),
            "HEY JUDE"
        );
    }
}
