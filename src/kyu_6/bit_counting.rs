// Bit Counting
// https://www.codewars.com/kata/526571aae218b8ee490006f4/train/rust

pub fn count_bits(n: i64) -> u32 {
    n.count_ones()
}

#[test]
fn returns_expected() {
    assert_eq!(count_bits(0), 0);
    assert_eq!(count_bits(4), 1);
    assert_eq!(count_bits(7), 3);
    assert_eq!(count_bits(9), 2);
    assert_eq!(count_bits(10), 2);
}
