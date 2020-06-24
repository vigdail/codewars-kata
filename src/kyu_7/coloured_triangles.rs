// Coloured Triangles
// https://www.codewars.com/kata/5a25ac6ac5e284cfbe000111/train/rust

fn triangle(row_str: &str) -> String {
    let mut s: Vec<_> = row_str.chars().collect();
    loop {
        if s.len() == 1 {
            return s[0].to_string();
        }

        s = s
            .windows(2)
            .map(|c| {
                let x = c[0];
                let y = c[1];
                match (x, y) {
                    ('R', 'G') | ('G', 'R') => 'B',
                    ('G', 'B') | ('B', 'G') => 'R',
                    ('B', 'R') | ('R', 'B') => 'G',
                    _ => x,
                }
            })
            .collect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle() {
        assert_eq!(triangle("GB"), "R");
        assert_eq!(triangle("RRR"), "R");
        assert_eq!(triangle("RGBG"), "B");
        assert_eq!(triangle("RBRGBRB"), "G");
        assert_eq!(triangle("RBRGBRBGGRRRBGBBBGG"), "G");
        assert_eq!(triangle("GB"), "R");
        assert_eq!(triangle("B"), "B");
    }
}
