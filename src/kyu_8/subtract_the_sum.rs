// Subtract the Sum
// https://www.codewars.com/kata/56c5847f27be2c3db20009c3
// Btw this kata designed in such way as to return only "apple" at any input =(

fn subtract_sum(n: u32) -> &'static str {
    let v = [
        "kiwi",
        "pear",
        "kiwi",
        "banana",
        "melon",
        "banana",
        "melon",
        "pineapple",
        "apple",
        "pineapple",
        "cucumber",
        "pineapple",
        "cucumber",
        "orange",
        "grape",
        "orange",
        "grape",
        "apple",
        "grape",
        "cherry",
        "pear",
        "cherry",
        "pear",
        "kiwi",
        "banana",
        "kiwi",
        "apple",
        "melon",
        "banana",
        "melon",
        "pineapple",
        "melon",
        "pineapple",
        "cucumber",
        "orange",
        "apple",
        "orange",
        "grape",
        "orange",
        "grape",
        "cherry",
        "pear",
        "cherry",
        "pear",
        "apple",
        "pear",
        "kiwi",
        "banana",
        "kiwi",
        "banana",
        "melon",
        "pineapple",
        "melon",
        "apple",
        "cucumber",
        "pineapple",
        "cucumber",
        "orange",
        "cucumber",
        "orange",
        "grape",
        "cherry",
        "apple",
        "cherry",
        "pear",
        "cherry",
        "pear",
        "kiwi",
        "pear",
        "kiwi",
        "banana",
        "apple",
        "banana",
        "melon",
        "pineapple",
        "melon",
        "pineapple",
        "cucumber",
        "pineapple",
        "cucumber",
        "apple",
        "grape",
        "orange",
        "grape",
        "cherry",
        "grape",
        "cherry",
        "pear",
        "cherry",
        "apple",
        "kiwi",
        "banana",
        "kiwi",
        "banana",
        "melon",
        "banana",
        "melon",
        "pineapple",
        "apple",
        "pineapple",
    ];
    let mut n = n;
    loop {
        n = n - n
            .to_string()
            .chars()
            .fold(0, |acc, c| acc + c.to_digit(10).unwrap_or(0));

        if n <= 100 {
            break;
        }
    }
    let i = n as usize - 1;
    v[i]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(subtract_sum(10), "apple");
        assert_eq!(subtract_sum(1000), "apple");
        assert_eq!(subtract_sum(105), "apple");
        assert_eq!(subtract_sum(999), "apple");
        assert_eq!(subtract_sum(11), "apple");
        assert_eq!(subtract_sum(56), "apple");
        assert_eq!(subtract_sum(93), "apple");
    }
}
