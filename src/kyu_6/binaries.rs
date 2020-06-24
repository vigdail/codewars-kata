/// https://www.codewars.com/kata/5d98b6b38b0f6c001a461198/train/rust
/// Binaries

fn code(s: &str) -> String {
    s.chars()
        .map(|c| {
            let bin = format!("{:b}", c.to_digit(10).unwrap_or(0));
            let k = bin.len();
            format!("{}1{}", "0".repeat(k - 1), bin)
        })
        .collect::<Vec<_>>()
        .join("")
}

fn decode(s: &str) -> String {
    let mut mode = true;
    let mut k = 0;
    let mut bin = Vec::new();
    let mut chars = s.chars();
    let mut res = Vec::new();
    while let Some(c) = chars.next() {
        println!("{}", c);
        match mode {
            true => {
                match c {
                    '0' => k += 1,
                    _ => {
                        k += 1;
                        mode = false;
                    }
                };
            }
            false => {
                bin.push(c.to_string());
                k -= 1;
                if k == 0 {
                    mode = true;
                    res.push(
                        i32::from_str_radix(&bin.join(""), 2)
                            .unwrap_or(0)
                            .to_string(),
                    );
                    bin = Vec::new();
                }
            }
        };
    }
    format!("{}", res.join(""))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing_code(s: &str, exp: String) -> () {
        let ans = code(s);
        assert_eq!(ans, exp, "Testing: {}", s);
    }
    fn testing_decode(s: &str, exp: String) -> () {
        let ans = decode(s);
        assert_eq!(ans, exp, "Testing: {}", s);
    }

    #[test]
    fn basic_tests_code() {
        testing_code("1", "11".to_string());
        testing_code("62", "0011100110".to_string());
        testing_code(
            "55337700",
            "001101001101011101110011110011111010".to_string(),
        );
        testing_code(
            "1119441933000055",
            "1111110001100100110000110011000110010111011110101010001101001101".to_string(),
        );
        testing_code("69", "00111000011001".to_string());
        testing_code("86", "00011000001110".to_string());
    }
    #[test]
    fn basic_tests_decode() {
        testing_decode("10", "0".to_string());
        testing_decode("11", "1".to_string());
        testing_decode("10001111", "07".to_string());
        testing_decode("001100001100001100001110001110001110011101110111001110001110001110001111001111001111001100001100001100", "444666333666777444".to_string());
        testing_decode(
            "01110111110001100100011000000110000011110011110111011100110000110001100110",
            "33198877334422".to_string(),
        );
        testing_decode("0011010011010011011010101111110011000011000011000011100011100011100011100011100011100001100100011001000110011100011001001111001111001111001111001111001111", "55500011144466666699919777777".to_string());
        testing_decode("01110111011111000110010011110011110011110011110011110011110111011101110110011001100110011001101111111010101100011001000110000001100000011000", "3331977777733322222211100019888".to_string());
    }
}
