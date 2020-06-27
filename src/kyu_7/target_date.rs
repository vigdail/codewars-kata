// Target Date
// https://www.codewars.com/kata/569218bc919ccba77000000b/train/rust

use chrono::NaiveDate;

fn date_nb_days(a0: i32, a: i32, p: i32) -> String {
    let a = a as f64;
    let p = p as f64 / 36000.0;
    let mut r = a0 as f64;
    let mut i = 0;
    while r < a {
        r += r * p;
        i += 1;
    }
    let start = NaiveDate::from_ymd(2016, 1, 1);
    format!("{}", start + chrono::Duration::days(i))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(a0: i32, a: i32, p: i32, exp: &str) -> () {
        println!(" a0: {:?};", a0);
        println!("a: {:?};", a);
        println!("p: {:?};", p);
        let ans = date_nb_days(a0, a, p);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(100, 150, 2, "2035-12-26");
        dotest(4281, 5087, 2, "2024-07-03");
        dotest(4620, 5188, 2, "2021-09-19");

        dotest(258, 586, 5, "2032-03-05");
    }
}
