/// https://www.codewars.com/kata/57a6633153ba33189e000074/train/rust
/// Ordered Count of Characters
fn ordered_count(sip: &str) -> Vec<(char, i32)> {
    let mut v = Vec::new();
    let mut x = Vec::new();
    sip.chars().for_each(|c| {
        if v.contains(&c) {
            x[v.iter().position(|&a| a == c).unwrap()] += 1;
        } else {
            v.push(c);
            x.push(1);
        }
    });
    v.into_iter().zip(x.into_iter()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abradacadabra() {
        assert_eq!(
            ordered_count("abracadabra"),
            vec![('a', 5), ('b', 2), ('r', 2), ('c', 1), ('d', 1)]
        );
    }
    #[test]
    fn test_banana() {
        assert_eq!(ordered_count("banana"), vec![('b', 1), ('a', 3), ('n', 2)]);
    }
    #[test]
    fn test_master_solver() {
        assert_eq!(
            ordered_count("i am a master kata solver"),
            vec![
                ('i', 1),
                (' ', 5),
                ('a', 5),
                ('m', 2),
                ('s', 2),
                ('t', 2),
                ('e', 2),
                ('r', 2),
                ('k', 1),
                ('o', 1),
                ('l', 1),
                ('v', 1)
            ]
        );
    }
}
