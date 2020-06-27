// Thinking & Testing : How many "word"?
// https://www.codewars.com/kata/56eff1e64794404a720002d2/train/rust

pub fn testit(s: &str) -> usize {
    s.to_lowercase()
        .chars()
        .fold(('w', 0), |(need, count), c| {
            if c == need {
                match need {
                    'w' => ('o', count),
                    'o' => ('r', count),
                    'r' => ('d', count),
                    'd' => ('w', count + 1),
                    _ => unreachable!(),
                }
            } else {
                (need, count)
            }
        })
        .1 as usize
}

#[cfg(test)]
mod tests {
    use super::testit;
    #[test]
    fn sample_tests() {
        assert_eq!(testit("word"), 1);
        assert_eq!(testit("hello world"), 1);
        assert_eq!(testit("I love codewars."), 0);
        assert_eq!(testit("My cat waiting for a dog."), 1);
        assert_eq!(testit("We often read book, a word hidden in the book."), 2);
        assert_eq!(testit("When you in order to do something by a wrong way, your heart will missed somewhere."), 2);
        assert_eq!(testit("This sentence have one word."), 1);
        assert_eq!(testit("This sentence have two word, not one word."), 2);
        assert_eq!(testit("One word + one word = three word ;-"), 3);
        assert_eq!(testit("Can you find more word for me?"), 1);
    }
}
