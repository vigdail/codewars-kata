// Esolang Interpreters #3 - Custom Paintf**k Interpreter
// https://www.codewars.com/kata/5868a68ba44cfc763e00008d/train/rust

// n - Move data pointer north (up)
// e - Move data pointer east (right)
// s - Move data pointer south (down)
// w - Move data pointer west (left)
// * - Flip the bit at the current cell (same as in Smallfuck)
// [ - Jump past matching ] if bit under current pointer is 0 (same as in Smallfuck)
// ] - Jump back to the matching [ (if bit under current pointer is nonzero) (same as in Smallfuck)
#[derive(Clone)]
enum Command {
    Up,
    Down,
    Left,
    Right,
    Flip,
    While(usize),
    EndWhile(usize),
}

struct Ram {
    data: Vec<Vec<u8>>,
    width: usize,
    height: usize,
    pointer_x: usize,
    pointer_y: usize,
}

impl Ram {
    pub fn new(width: usize, height: usize) -> Ram {
        Ram {
            data: vec![vec![0; width]; height],
            width,
            height,
            pointer_x: 0,
            pointer_y: 0,
        }
    }

    pub fn flip(&mut self) {
        self.data[self.pointer_y][self.pointer_x] = 1 - self.get();
    }

    pub fn get(&mut self) -> u8 {
        self.data[self.pointer_y][self.pointer_x]
    }

    pub fn up(&mut self) {
        if self.pointer_y > 0 {
            self.pointer_y -= 1;
        } else {
            self.pointer_y = self.height - 1;
        }
    }

    pub fn down(&mut self) {
        if self.pointer_y < self.height - 1 {
            self.pointer_y += 1;
        } else {
            self.pointer_y = 0;
        }
    }

    pub fn left(&mut self) {
        if self.pointer_x > 0 {
            self.pointer_x -= 1;
        } else {
            self.pointer_x = self.width - 1;
        }
    }

    pub fn right(&mut self) {
        if self.pointer_x < self.width - 1 {
            self.pointer_x += 1;
        } else {
            self.pointer_x = 0;
        }
    }

    pub fn to_string(&self) -> String {
        self.data
            .iter()
            .map(|x| x.iter().map(|i| i.to_string()).collect::<Vec<_>>().join(""))
            .collect::<Vec<_>>()
            .join("\r\n")
    }
}

struct Interpreter {
    mem: Ram,
    commands: Vec<Command>,
    pointer: usize,
    iterations_left: usize,
}

impl Interpreter {
    pub fn new(code: &str, iterations: usize, width: usize, height: usize) -> Interpreter {
        Interpreter {
            mem: Ram::new(width, height),
            commands: Interpreter::parse_commands(code),
            pointer: 0,
            iterations_left: iterations,
        }
    }

    fn run(&mut self) {
        while self.iterations_left > 0 && self.pointer < self.commands.len() {
            self.run_command(self.commands.get(self.pointer).unwrap().clone());
        }
    }

    fn run_command(&mut self, command: Command) {
        use Command::*;
        match command {
            While(p) => {
                if self.mem.get() == 0 {
                    self.pointer = p;
                    return;
                }
            }
            EndWhile(p) => {
                if self.mem.get() != 0 {
                    self.pointer = p;
                    return;
                }
            }
            Flip => self.mem.flip(),
            Up => self.mem.up(),
            Down => self.mem.down(),
            Left => self.mem.left(),
            Right => self.mem.right(),
        }
        self.pointer += 1;
        self.iterations_left -= 1;
    }

    fn parse_commands(code: &str) -> Vec<Command> {
        let mut stack = Vec::new();
        let mut commands = Vec::new();
        let mut count = 0;
        for c in code.chars() {
            let command = match c {
                '*' => Command::Flip,
                'n' => Command::Up,
                's' => Command::Down,
                'w' => Command::Left,
                'e' => Command::Right,
                '[' => {
                    stack.push(count);
                    Command::While(0)
                }
                ']' => {
                    let prev = match stack.pop() {
                        Some(o) => o,
                        None => unreachable!(),
                    };
                    let open = commands.get_mut(prev).unwrap();
                    *open = Command::While(count);
                    Command::EndWhile(prev)
                }
                _ => continue,
            };
            count += 1;
            commands.push(command);
        }

        commands
    }
}

pub fn interpreter(code: &str, iterations: usize, width: usize, height: usize) -> String {
    let mut cpu = Interpreter::new(code, iterations, width, height);
    cpu.run();
    cpu.mem.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_zero_iterations() {
        assert_eq!(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 0, 6, 9), "000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000");
    }
    #[test]
    fn test_empty_program() {
        assert_eq!(&interpreter("", 10, 6, 9), "000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000");
    }
    #[test]
    fn test_only_one_flip() {
        assert_eq!(&interpreter("*", 1, 6, 9), "100000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000");
    }
    #[test]
    fn other_tests() {
        assert_eq!(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 7, 6, 9), "111100\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000");
        assert_eq!(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 19, 6, 9), "111100\r\n000010\r\n000001\r\n000010\r\n000100\r\n000000\r\n000000\r\n000000\r\n000000");
        assert_eq!(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 42, 6, 9), "111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000");
        assert_eq!(&interpreter("*e*e*e*es*es*ws*ws*w*w*w*n*n*n*ssss*s*s*s*", 100, 6, 9), "111100\r\n100010\r\n100001\r\n100010\r\n111100\r\n100000\r\n100000\r\n100000\r\n100000");
    }

    #[test]
    fn test_double_flip() {
        assert_eq!(&interpreter("**", 2, 6, 9), "000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000");
    }

    #[test]
    fn complex_test() {
        assert_eq!(&interpreter("ss*s*sns***en*wnenns*sew*n*eeww*esnwennw*s**w*swnsw*ewesesws*s*nee*ne*nww*ww*sssnenwnswwseeew*sseeenwnsness*seeesnsnnenswnnns*eens**w*s*see*nwwe*nn*e*see*e*een*we*nesw*es*nns*e*es*wwnsewwwwsw*w**e*swewwnwe*nw**s*wnnws*ssnns**nwe*wwwsnnwssssnsnenswwws*ewess**ens*ew*****nwesennn*ewewwwnwwnnenenese****ns*wenwe*ssnsewwsesnnnnsnenn***new*s**e*s*newswnseseswsennswen*wwn*ssn", 1000, 10, 10),
"1011000000\r\n0000100010\r\n0000000010\r\n1101101100\r\n0101100010\r\n0000010101\r\n0000000100\r\n0000000010\r\n0010000000\r\n0111000000");
    }

    #[test]
    fn test_loop() {
        assert_eq!(&interpreter("*[es*]", 1, 6, 9), "100000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000");
        assert_eq!(&interpreter("*[es*]", 5, 6, 9), "100000\r\n010000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000");
        assert_eq!(&interpreter("*[es*]", 9, 6, 9), "100000\r\n010000\r\n001000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000\r\n000000");
    }
}
