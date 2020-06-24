/// Two to One
/// https://www.codewars.com/kata/5656b6906de340bd1b0000ac/train/rust

fn longest(a1: &str, a2: &str) -> String {
    let mut s = a1.chars().chain(a2.chars()).collect::<Vec<_>>();
    s.sort_by(|a, b| a.cmp(b));
    s.dedup();

    s.iter().collect()
}

mod the_best {
    use std::collections::BTreeSet;
    fn longest(a1: &str, a2: &str) -> String {
        a1.chars()
            .chain(a2.chars())
            .collect::<BTreeSet<char>>()
            .iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn testing(s1: &str, s2: &str, exp: &str) -> () {
        println!("s1:{:?} s2:{:?}", s1, s2);
        println!("{:?} {:?}", longest(s1, s2), exp);
        println!("{}", longest(s1, s2) == exp);
        assert_eq!(&longest(s1, s2), exp)
    }

    #[test]
    fn basic_tests() {
        testing("aretheyhere", "yestheyarehere", "aehrsty");
        testing(
            "loopingisfunbutdangerous",
            "lessdangerousthancoding",
            "abcdefghilnoprstu",
        );
    }
}
