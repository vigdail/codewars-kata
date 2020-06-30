// RoboScript #2 - Implement the RS1 Specification
// https://www.codewars.com/kata/5870fa11aa0428da750000da/train/rust

use std::collections::HashSet;

pub fn execute(code: &str) -> String {
    let mut cpu = Intrerpteter::new();
    cpu.run(code)
}

struct Bounds {
    top: isize,
    bottom: isize,
    left: isize,
    right: isize,
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn turn_left(&self) -> Dir {
        match self {
            Dir::Up => Dir::Left,
            Dir::Down => Dir::Right,
            Dir::Left => Dir::Down,
            Dir::Right => Dir::Up,
        }
    }

    pub fn turn_right(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Right => Dir::Down,
        }
    }
}

struct Intrerpteter {
    x: isize,
    y: isize,
    dir: Dir,
    bounds: Bounds,
    points: HashSet<(isize, isize)>,
}

impl Intrerpteter {
    pub fn new() -> Intrerpteter {
        let mut points = HashSet::new();
        points.insert((0, 0));
        Intrerpteter {
            x: 0,
            y: 0,
            dir: Dir::Right,
            points,
            bounds: Bounds {
                top: 0,
                left: 0,
                right: 1,
                bottom: 1,
            },
        }
    }

    pub fn run(&mut self, code: &str) -> String {
        let mut iter = code.chars().peekable();
        while let Some(c) = iter.next() {
            match c {
                'F' | 'L' | 'R' => {
                    let mut buf = Vec::new();
                    while iter.peek().map(|&x| x.is_digit(10)).unwrap_or(false) {
                        buf.push(iter.next().unwrap());
                    }
                    let repeat = buf.iter().collect::<String>().parse().unwrap_or(1);
                    self.repeat_command(c, repeat);
                }
                _ => {}
            }
        }
        self.to_string()
    }

    fn repeat_command(&mut self, command: char, repeat: u32) {
        for _ in 0..repeat {
            self.process_command(command);
        }
    }

    fn process_command(&mut self, command: char) {
        match command {
            'F' => match self.dir {
                Dir::Up => self.y -= 1,
                Dir::Down => self.y += 1,
                Dir::Left => self.x -= 1,
                Dir::Right => self.x += 1,
            },
            'L' => self.dir = self.dir.turn_left(),
            'R' => self.dir = self.dir.turn_right(),
            _ => {}
        }

        self.update_bounds();
        self.points.insert((self.x, self.y));
    }

    fn update_bounds(&mut self) {
        if self.x < self.bounds.left {
            self.bounds.left = self.x;
        }
        if self.x >= self.bounds.right {
            self.bounds.right = self.x + 1;
        }
        if self.y < self.bounds.top {
            self.bounds.top = self.y;
        }
        if self.y >= self.bounds.bottom {
            self.bounds.bottom = self.y + 1;
        }
    }

    fn to_string(&self) -> String {
        let v = self.build_field();
        format!(
            "{}",
            v.iter()
                .map(|x| x.iter().collect())
                .collect::<Vec<String>>()
                .join("\r\n")
        )
    }

    fn build_field(&self) -> Vec<Vec<char>> {
        let width = (self.bounds.right - self.bounds.left) as usize;
        let height = (self.bounds.bottom - self.bounds.top) as usize;

        let mut v = vec![vec![' '; width]; height];
        for (x, y) in &self.points {
            let x = (x - self.bounds.left) as usize;
            let y = (y - self.bounds.top) as usize;
            v[y][x] = '*';
        }
        v
    }
}

#[cfg(test)]
macro_rules! expect_equal {
    ($actual:expr, $expected:expr $(,)*) => {{
        let actual = $actual;
        let expected = $expected;
        assert_eq!(
            actual, expected,
            "\ngot:\n{}\n\nexpected:\n{}\n",
            actual, expected
        );
    }};
}

#[cfg(test)]
mod tests {
    use super::execute;
    #[test]
    fn examples_in_description() {
        expect_equal!(execute(""), "*");
        expect_equal!(execute("FFFFF"), "******");
        expect_equal!(
            execute("FFFFFLFFFFFLFFFFFLFFFFFL"),
            "******\r\n*    *\r\n*    *\r\n*    *\r\n*    *\r\n******",
        );
        expect_equal!(
            execute("LFFFFFRFFFRFFFRFFFFFFF"),
            "    ****\r\n    *  *\r\n    *  *\r\n********\r\n    *   \r\n    *   ",
        );
        expect_equal!(
            execute("LF5RF3RF3RF7"),
            "    ****\r\n    *  *\r\n    *  *\r\n********\r\n    *   \r\n    *   ",
        );
    }
}
