// RoboScript #3 - Implement the RS2 Specification
// https://www.codewars.com/kata/58738d518ec3b4bf95000192/train/rust
use std::collections::HashSet;

pub fn execute(code: &str) -> String {
    let (_, program) = parse_input(code);
    let mut cpu = Intrerpteter::new();
    cpu.run(&program)
}

#[derive(Debug)]
enum Token {
    Sequence(Vec<Instruction>),
    Command(char),
}

#[derive(Debug)]
struct Instruction {
    token: Token,
    repeat: Option<u32>,
}

fn parse_input(input: &str) -> (&str, Vec<Instruction>) {
    let mut r = vec![];
    let mut iter = input.chars().enumerate();
    while let Some((i, n)) = iter.next() {
        match n {
            ')' => {
                return (&input[i..], r);
            }
            '(' => {
                let (rest, command) = parse_input(&input[i + 1..]);

                for _ in 0..input.len() - i - rest.len() {
                    iter.next();
                }

                r.push(Instruction {
                    token: Token::Sequence(command),
                    repeat: None,
                });
            }
            c if c.is_digit(10) => {
                let repeat = c.to_digit(10).unwrap();
                let mut prev = r.pop().unwrap();
                prev.repeat = Some(prev.repeat.unwrap_or(0) * 10 + repeat);
                r.push(prev);
            }
            _ => {
                let command = Instruction {
                    token: Token::Command(n),
                    repeat: None,
                };
                r.push(command);
            }
        }
    }
    ("", r)
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

    pub fn run(&mut self, code: &Vec<Instruction>) -> String {
        let mut iter = code.iter();
        while let Some(c) = iter.next() {
            let Instruction { token, repeat } = c;
            let repeat = repeat.unwrap_or(1);
            match token {
                Token::Command(name) => {
                    self.repeat_command(*name, repeat);
                }
                Token::Sequence(seq) => {
                    for _ in 0..repeat {
                        self.run(seq.clone());
                    }
                }
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
        (self.bounds.top..self.bounds.bottom)
            .map(|y| {
                (self.bounds.left..self.bounds.right)
                    .map(|x| {
                        if self.points.contains(&(x, y)) {
                            '*'
                        } else {
                            ' '
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\r\n")
    }
}

#[cfg(test)]
macro_rules! assert_equal {
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
#[test]
fn examples_in_description() {
    assert_equal!(
        execute("LF5(RF3)(RF3R)F7"),
        "    ****\r\n    *  *\r\n    *  *\r\n********\r\n    *   \r\n    *   ",
    );
    // assert_equal!(
    //     execute("(L(F5(RF3))(((R(F3R)F7))))"),
    //     "    ****\r\n    *  *\r\n    *  *\r\n********\r\n    *   \r\n    *   ",
    // );
    // assert_equal!(
    //     execute("F4L(F4RF4RF4LF4L)2F4RF4RF4"),
    //     "    *****   *****   *****\r\n    *   *   *   *   *   *\r\n    *   *   *   *   *   *\r\n    *   *   *   *   *   *\r\n*****   *****   *****   *",
    //   );
    // assert_equal!(
    // execute("F4L((F4R)2(F4L)2)2(F4R)2F4"),
    // "    *****   *****   *****\r\n    *   *   *   *   *   *\r\n    *   *   *   *   *   *\r\n    *   *   *   *   *   *\r\n*****   *****   *****   *",
    //   );
}
