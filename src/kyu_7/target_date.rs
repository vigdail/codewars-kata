/// Target Date
/// https://www.codewars.com/kata/569218bc919ccba77000000b/train/rust

fn date_nb_days(a0: i32, a: i32, p: i32) -> String {
    let mut r = a0 as f32;
    let mut i = 0;
    while r < a as f32 {
        r += r * p as f32 / 36000.0;
        i += 1;
    }

    println!("{}", i);

    "".to_string()
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
    #[ignore]
    fn basic_tests() {
        dotest(4281, 5087, 2, "2024-07-03");
        dotest(4620, 5188, 2, "2021-09-19");
    }
}
