/// Is It Negative Zero
/// https://www.codewars.com/kata/5c5086287bc6600001c7589a/train/rust

fn is_negative_zero(n: f32) -> bool {
    n.is_sign_negative() && n == 0.0
}

#[cfg(test)]
mod test {
    use super::is_negative_zero;
    use std::f32;

    #[test]
    fn sample_tests() {
        assert_eq!(is_negative_zero(-0.0), true);
        assert_eq!(is_negative_zero(f32::NEG_INFINITY), false);
        assert_eq!(is_negative_zero(-5.0), false);
        assert_eq!(is_negative_zero(-4.0), false);
        assert_eq!(is_negative_zero(-3.0), false);
        assert_eq!(is_negative_zero(-2.0), false);
        assert_eq!(is_negative_zero(-1.0), false);
        assert_eq!(is_negative_zero(-f32::MIN), false);
        assert_eq!(is_negative_zero(0.0), false);
        assert_eq!(is_negative_zero(f32::MIN), false);
        assert_eq!(is_negative_zero(1.0), false);
        assert_eq!(is_negative_zero(2.0), false);
        assert_eq!(is_negative_zero(3.0), false);
        assert_eq!(is_negative_zero(4.0), false);
        assert_eq!(is_negative_zero(5.0), false);
        assert_eq!(is_negative_zero(f32::INFINITY), false);
    }
}
