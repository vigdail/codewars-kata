// Floating-point Approximation (III)
// https://www.codewars.com/kata/5b0c0ec907756ffcff00006e/train/rust

// We need to take in account Loss of Significance
// https://en.wikipedia.org/wiki/Loss_of_significance
fn quadratic(a: f64, b: f64, c: f64) -> f64 {
    let d = b * b - 4.0 * a * c;
    let x1 = -(b + d.sqrt()) / 2.0 / a;
    let x2 = c / a / x1;
    x2

    // also solution may be just -c/b, why???
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_fuzzy_equals(a: f64, b: f64, c: f64) -> () {
        let merr = 1e-12;
        println!("{:?} {:e} {:?}", a, b, c);
        let x = quadratic(a, b, c);
        let smallest = x.abs() < 1.0e-1;
        if smallest == false {
            println!("This root is not the good one");
        }
        let actual = a * x * x + b * x + c;
        println!("Actual f(x) {:e}", actual);
        let inrange = actual.abs() <= merr;
        if inrange == false {
            println!("Expected value near: 0 but got: {:e}", actual);
        }
        let correct = smallest && inrange;
        assert_eq!(correct, true);
    }
    #[test]
    fn basic_tests() {
        assert_fuzzy_equals(7.0, 4.00e+13, 8.0);
        assert_fuzzy_equals(9.0, 1.00e+14, 1.0);
        assert_fuzzy_equals(3.0, 3.00e+09, 1.0);
        assert_fuzzy_equals(7.0, 4.00e+09, 7.0);
    }
}
