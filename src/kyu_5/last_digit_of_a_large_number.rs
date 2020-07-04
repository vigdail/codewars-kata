// Last digit of a large number
// https://www.codewars.com/kata/5511b2f550906349a70004e1/train/rust

pub fn last_digit(str1: &str, str2: &str) -> i32 {
    let pow = if str2.len() > 1 {
        let b = str2[str2.len() - 2..str2.len()].parse::<u32>().unwrap_or(0) % 4;
        if b == 0 {
            4
        } else {
            b
        }
    } else {
        str2.parse().unwrap_or(0)
    };
    str1.chars()
        .last()
        .and_then(|c| c.to_digit(10))
        .unwrap_or(0)
        .pow(pow) as i32
        % 10 as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(last_digit("4", "1"), 4);
        assert_eq!(last_digit("4", "2"), 6);
        assert_eq!(last_digit("9", "7"), 9);
        assert_eq!(last_digit("10", "10000000000"), 0);
        assert_eq!(last_digit("1606938044258990275541962092341162602522202993782792835301376","2037035976334486086268445688409378161051468393665936250636140449354381299763336706183397376"), 6);
        assert_eq!(
            last_digit(
                "3715290469715693021198967285016729344580685479654510946723",
                "68819615221552997273737174557165657483427362207517952651"
            ),
            7
        );
    }
}
