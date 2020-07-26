// Hexagon Beam Max Sum
// https://www.codewars.com/kata/5ecc1d68c6029000017d8aaf/train/rust

pub fn max_hexagon_beam(n: u8, seq: &[i32]) -> i32 {
    let v = build(n, seq);
    vec![
        v.iter().map(|x| x.iter().sum()).max().unwrap(),
        (0..v.len())
            .map(|x| {
                let a = (0..v.len()).map(|y| get_x(&v, n as isize, x as isize, y as isize));
                let c = a.collect::<Vec<_>>();
                let s = c.iter().sum();
                s
            })
            .max()
            .unwrap(),
        (0..v.len())
            .map(|x| {
                (0..v.len())
                    .map(|i| {
                        let c = if i > n as usize - 1 {
                            i - n as usize + 1
                        } else {
                            0
                        };
                        let t = v.get(i as usize).unwrap();
                        let w = x as isize - c as isize + 1;
                        if w < 0 {
                            return 0;
                        }
                        if (t.len() as isize - w) < 0 {
                            return 0;
                        }
                        if i < n as usize + x {
                            let q = *t.get(t.len() - w as usize).unwrap_or(&0);
                            q
                        } else {
                            0
                        }
                    })
                    .sum()
            })
            .max()
            .unwrap(),
    ]
    .into_iter()
    .max()
    .unwrap()
}

fn build(n: u8, seq: &[i32]) -> Vec<Vec<i32>> {
    let mut v = Vec::new();
    let mut m = n as i32;
    let mut iter = seq.iter().cycle();
    let mut inc = 1;
    for i in 0..n * 2 - 1 {
        let mut x = Vec::new();
        for _ in 0..m {
            x.push(*iter.next().unwrap_or(&0));
        }
        m += inc;
        if i == n - 1 {
            m -= 2;
            inc = -1;
        }
        v.push(x);
    }
    v
}

fn get_x(v: &Vec<Vec<i32>>, n: isize, x: isize, y: isize) -> i32 {
    let x_t = if y >= n { x - y + n - 1 } else { x };
    if x_t < 0 {
        return 0;
    }

    *v.get(y as usize).unwrap().get(x_t as usize).unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        let tests: Vec<(u8, Vec<i32>, i32)> = vec![
            (2, vec![5, 8, 3, 8], 24),
            (3, vec![1, 3, 5, 7], 23),
            (4, vec![2, 4, 6, 8], 34),
            (5, vec![1, 0, 4, -6], 9),
            (5, vec![2], 18),
            (10, vec![1, 2, 3, 4, 5], 64),
        ];
        for (n, seq, expected) in tests.iter() {
            let result = max_hexagon_beam(*n, seq);
            assert_eq!(result, *expected, "n = {}, seq = {:?}", n, seq);
        }
    }
}
