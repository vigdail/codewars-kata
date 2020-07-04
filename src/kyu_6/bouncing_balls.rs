// Bouncing Balls
// https://www.codewars.com/kata/5544c7a5cb454edb3c000047/train/rust

pub fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if bounce >= 1.0 || bounce <= 0.0 || h <= 0.0 || window >= h {
        return -1;
    }

    let mut i = 0;
    let mut h = h;
    loop {
        h = bounce * h;
        if h <= window {
            break;
        }
        i += 1;
    }

    i * 2 + 1
}

#[cfg(test)]
mod test {
    use super::*;

    fn testequal(h: f64, bounce: f64, window: f64, exp: i32) -> () {
        assert_eq!(bouncing_ball(h, bounce, window), exp)
    }

    #[test]
    fn tests_bouncing_ball() {
        testequal(3.0, 0.66, 1.5, 3);
        testequal(30.0, 0.66, 1.5, 15);
        testequal(40.0, 0.4, 10.0, 3);
        testequal(10.0, 0.6, 10.0, -1);
    }
}
