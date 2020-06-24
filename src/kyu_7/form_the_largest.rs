fn max_number(n: u32) -> u32 {
    let mut chars = n.to_string().chars().collect::<Vec<char>>();
    chars.sort_by(|a, b| b.cmp(a));
    chars.iter().collect::<String>().parse::<u32>().unwrap()
}

#[cfg(test)]
mod test {
    use super::max_number;

    #[test]
    fn basic_tests() {
        assert_eq!(max_number(213), 321);
        assert_eq!(max_number(7389), 9873);
        assert_eq!(max_number(63729), 97632);
        assert_eq!(max_number(566797), 977665);
        assert_eq!(max_number(17693284), 98764321);
    }
}
