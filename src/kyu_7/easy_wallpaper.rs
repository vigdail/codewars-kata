/// Easy wallpaper
/// https://www.codewars.com/kata/567501aec64b81e252000003/train/rust

pub fn wall_paper(l: f64, w: f64, h: f64) -> String {
    if l == 0.0 || w == 0.0 || h == 0.0 {
        return "zero".to_string();
    }
    let roll_len = 10.0;
    let roll_width = 0.52;
    let res = (l + w) * 2.0 * h / (roll_len * roll_width) * 1.15;

    n_to_string(res.ceil() as u32)
}

fn n_to_string(n: u32) -> String {
    match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        20 => "twenty",
        _ => unreachable!(),
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(l: f64, w: f64, h: f64, exp: &str) -> () {
        println!("l: {:?}", l);
        println!("w: {:?}", w);
        println!("h: {:?}", h);
        let ans = wall_paper(l, w, h);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }
    #[test]
    fn basic_tests() {
        dotest(6.3, 4.5, 3.29, "sixteen");
        dotest(6.3, 5.8, 3.13, "seventeen");
    }
}
