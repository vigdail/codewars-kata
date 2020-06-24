/// https://www.codewars.com/kata/5b049d57de4c7f6a6c0001d7/train/rust
/// Apparently-Modifying Strings
fn apparently(string: &str) -> String {
    format!("{} ", string)
        .split(" ")
        .collect::<Vec<_>>()
        .windows(2)
        .map(|x| {
            let (prev, next) = (x[0], x[1]);
            match prev {
                "and" | "but" => match next {
                    "apparently" => prev.to_string(),
                    _ => format!("{} apparently", prev),
                },
                _ => prev.to_string(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod test {
    use super::*;
    fn test_exp(a: &str, exp: &str) {
        assert_eq!(apparently(a), exp.to_string());
    }

    #[test]
    fn test_apparently() {
        test_exp("It was great and I have never been on live television before but sometimes I dont watch this.", "It was great and apparently I have never been on live television before but apparently sometimes I dont watch this.");
        test_exp("and", "and apparently");
        test_exp("apparently", "apparently");
        test_exp("and apparently", "and apparently");
        test_exp("and apparently apparently", "and apparently apparently");
        test_exp("", "");
        test_exp(" ", " ");
    }
}
