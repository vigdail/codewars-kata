// Don't give me five!
// https://www.codewars.com/kata/5813d19765d81c592200001a/train/rust

fn dont_give_me_five(start: isize, end: isize) -> isize {
    (start..=end)
        .filter(|i| !i.to_string().contains("5"))
        .count() as isize
}

#[cfg(test)]
mod tests {
    use super::dont_give_me_five;

    #[test]
    fn returns_expected() {
        assert_eq!(dont_give_me_five(1, 9), 8);
        assert_eq!(dont_give_me_five(4, 17), 12);
    }
}
