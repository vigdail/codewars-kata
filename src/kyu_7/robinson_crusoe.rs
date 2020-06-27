/// Robinson Crusoe
/// https://www.codewars.com/kata/5d95b7644a336600271f52ba/train/rust
/// =================================================================
use std::f64;

pub fn crusoe(n: i32, d: f64, ang: i32, distmult: f64, angmult: f64) -> (f64, f64) {
    let mut ang = f64::consts::PI * ang as f64 / 180.0;
    let mut d = d;
    let mut x = d * ang.cos();
    let mut y = d * ang.sin();
    for _ in 1..n {
        ang = ang * angmult;
        d = d * distmult;
        x += d * ang.cos();
        y += d * ang.sin();
    }

    (x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_fuzzy(
        n: i32,
        d: f64,
        ang: i32,
        distmult: f64,
        angmult: f64,
        expect: (f64, f64),
    ) -> () {
        let merr = 1e-12;
        let actual = crusoe(n, d, ang, distmult, angmult);
        let inrange = (actual.0 - expect.0).abs() <= merr && (actual.0 - expect.0).abs() <= merr;
        if inrange == false {
            println!("Expected.0 value near: {} but got: {}", expect.0, actual.0);
            println!("Expected.1 value near: {} but got: {}", expect.1, actual.1);
        }
        assert_eq!(inrange, true);
    }

    #[test]
    fn basic_tests() {
        assert_fuzzy(8, 0.22, 3, 1.01, 1.15, (1.814652098870, 0.164646220964));
        assert_fuzzy(29, 0.13, 21, 1.01, 1.09, (0.318341393410, 2.292862212314));
        assert_fuzzy(45, 0.10, 3, 1.01, 1.10, (2.689897523779, 2.477953232467));
    }
}
