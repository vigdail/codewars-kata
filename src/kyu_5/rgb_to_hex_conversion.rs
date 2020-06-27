// RGB To Hex Conversion
// https://www.codewars.com/kata/513e08acc600c94f01000001/train/rust

pub fn rgb(r: i32, g: i32, b: i32) -> String {
    format!(
        "{:02X}{:02X}{:02X}",
        r.max(0).min(255),
        g.max(0).min(255),
        b.max(0).min(255)
    )
}

#[cfg(test)]
mod sample_tests {
    use self::super::*;

    macro_rules! compare {
        ( $got : expr, $expected : expr ) => {
            if $got != $expected {
                panic!("Got: {}\nExpected: {}\n", $got, $expected);
            }
        };
    }

    #[test]
    fn tests() {
        compare!(rgb(0, 0, 0), "000000");
        compare!(rgb(1, 2, 3), "010203");
        compare!(rgb(255, 255, 255), "FFFFFF");
        compare!(rgb(254, 253, 252), "FEFDFC");
        compare!(rgb(-20, 275, 125), "00FF7D");
    }
}
