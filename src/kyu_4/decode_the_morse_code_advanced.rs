// Decode the Morse code, advanced
// https://www.codewars.com/kata/54b72c16cd7f5154e9000457/train/rust
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

    pub fn decode_bits(&self, encoded: &str) -> String {
        let groups = Self::group(encoded.trim_matches('0'))
            .into_iter()
            .map(|(c, x)| (c, x.len()))
            .collect::<Vec<_>>();
        let unit = groups.iter().map(|(_, count)| count).min().unwrap();
        groups
            .iter()
            .map(|(c, count)| match (c, count / unit) {
                ('1', 1) => ".",
                ('0', 1) => "",
                ('1', 3) => "-",
                ('0', 3) => " ",
                _ => "   ",
            })
            .collect()
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

    fn group(input: &str) -> Vec<(char, String)> {
        input
            .chars()
            .enumerate()
            .fold(
                (Vec::new(), String::new(), input.chars().nth(0).unwrap()),
                |(mut res, mut buf, mut last), (i, x)| {
                    match x {
                        _ if x == last => buf.push(x),
                        _ => {
                            if buf.len() > 0 {
                                res.push((last, buf));
                            }
                            last = x;
                            buf = String::new();
                            buf.push(last);
                        }
                    }
                    if i == input.len() - 1 {
                        res.push((last, buf));
                        buf = String::new();
                    }
                    (res, buf, last)
                },
            )
            .0
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn examples() {
        let decoder = MorseDecoder::new();
        assert_eq!(decoder.decode_morse(&decoder.decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")), "HEY JUDE".to_string());
    }

    #[test]
    fn test_group() {
        assert_eq!(
            MorseDecoder::group("001100"),
            vec![
                ('0', "00".to_string()),
                ('1', "11".to_string()),
                ('0', "00".to_string())
            ]
        );
        assert_eq!(
            MorseDecoder::group("01110"),
            vec![
                ('0', "0".to_string()),
                ('1', "111".to_string()),
                ('0', "0".to_string())
            ]
        );
        assert_eq!(MorseDecoder::group("111"), vec![('1', "111".to_string())]);
    }
}
