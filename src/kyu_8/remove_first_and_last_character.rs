/// Remove First and Last Character
/// https://www.codewars.com/kata/56bc28ad5bdaeb48760009b0/train/rust

pub fn remove_char(s: &str) -> String {
    s.chars().skip(1).take(s.len() - 2).collect()
}

#[cfg(test)]
mod tests {
    use super::remove_char;

    #[test]
    fn sample_cases() {
        assert_eq!(remove_char("eloquent"), "loquen");
        assert_eq!(remove_char("country"), "ountr");
        assert_eq!(remove_char("person"), "erso");
        assert_eq!(remove_char("place"), "lac");
        assert_eq!(remove_char("ok"), "");
        assert_eq!(remove_char("ooopsss"), "oopss");
    }
}
