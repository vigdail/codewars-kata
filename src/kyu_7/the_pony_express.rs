// https://www.codewars.com/kata/5b18e9e06aefb52e1d0001e9/train/rust

pub fn riders(stations: &Vec<u32>) -> u32 {
    let mut i = 1;
    stations.iter().fold(0, |acc, x| {
        let len = acc + x;
        if len > 100 {
            i += 1;
            return *x;
        }
        len
    });
    i
}

pub fn riders_best(stations: &Vec<u32>) -> u32 {
    stations
        .iter()
        .fold((0, 1), |(mile, rider), &x| {
            if mile + x > 100 {
                (x, rider + 1)
            } else {
                (mile + x, rider)
            }
        })
        .1
}

#[cfg(test)]
mod test {
    use super::riders;
    #[test]
    fn sample_tests() {
        assert_eq!(riders(&vec![18, 15]), 1);
        assert_eq!(riders(&vec![43, 23, 40, 13]), 2);
        assert_eq!(riders(&vec![33, 8, 16, 47, 30, 30, 46]), 3);
        assert_eq!(
            riders(&vec![6, 24, 6, 8, 28, 8, 23, 47, 17, 29, 37, 18, 40, 49]),
            4
        );
        assert_eq!(
            riders(&vec![
                7, 5, 21, 19, 25, 5, 23, 18, 24, 21, 20, 7, 17, 24, 14, 9, 12, 23, 16, 25, 16, 17,
                24, 21, 7, 14, 15, 13, 12, 25, 17, 25, 18, 9, 20, 6, 7, 15, 5, 24, 22, 14, 16, 7,
                9, 6, 6, 6, 12, 24, 23, 20, 7, 18, 19, 9, 23, 8, 19, 14, 8, 9, 24, 25, 22, 6, 11,
                24, 5, 24, 17, 24, 23, 16, 20, 8, 24, 18, 10, 19, 23, 19, 24, 24, 6, 23, 12, 5, 24,
                10, 10, 23, 9, 8, 10, 5, 7, 8, 24, 15, 12, 22, 21, 6, 25, 22, 21, 10, 22, 7, 22,
                17, 18, 17, 15, 17, 8, 23, 5, 19, 9, 15, 24, 7, 10, 13, 18, 25, 16, 17, 14, 14, 21,
                8, 5, 18, 24, 20, 5, 22, 13, 22, 16, 13, 16, 13, 23, 10, 6, 16, 14, 19, 10, 10, 7,
                15, 14, 19, 12, 15, 12, 16, 5, 8, 14, 12, 15, 8, 11, 20, 17, 19, 9, 17, 22, 8, 22,
                10, 23, 12, 19, 16, 6, 10
            ]),
            31
        );
    }
}
