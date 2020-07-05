// Esolang Interpreters #4 - Boolfuck Interpreter
// https://www.codewars.com/kata/5861487fdb20cff3ab000030/train/rust

use std::collections::VecDeque;

pub fn boolfuck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let mut cpu = CPU::new(code, input);
    cpu.run();
    cpu.output
}

#[derive(Clone)]
enum Command {
    Flip,
    Read,
    Write,
    Left,
    Right,
    While(usize),
    EndWhile(usize),
}

struct RAM {
    mem: VecDeque<bool>,
    pointer: usize,
}

impl RAM {
    pub fn move_left(&mut self) {
        if self.pointer == 0 {
            self.mem.push_front(false);
        } else {
            self.pointer -= 1;
        }
    }

    pub fn move_right(&mut self) {
        self.pointer += 1;
        if self.pointer >= self.mem.len() {
            self.mem.push_back(false);
        }
    }

    pub fn flip(&mut self) {
        self.set(!self.get());
    }

    pub fn set(&mut self, value: bool) {
        self.mem[self.pointer] = value;
    }

    pub fn get(&self) -> bool {
        *self.mem.get(self.pointer).unwrap()
    }
}

impl RAM {
    pub fn new() -> RAM {
        let mut mem = VecDeque::new();
        mem.push_back(false);
        RAM { mem, pointer: 0 }
    }
}

struct CPU {
    mem: RAM,
    program: Vec<Command>,
    ip: usize,
    output_buffer: String,
    input_buffer: String,
    output: Vec<u8>,
}

impl CPU {
    pub fn new(code: &str, input: Vec<u8>) -> CPU {
        CPU {
            mem: RAM::new(),
            program: CPU::parse_commands(code),
            ip: 0,
            output_buffer: String::new(),
            input_buffer: CPU::parse_input(input),
            output: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        while self.ip < self.program.len() {
            self.run_command(self.program.get(self.ip).unwrap().clone());
        }
        if self.output_buffer.len() != 0 {
            self.output.push(
                u8::from_str_radix(&self.output_buffer.chars().rev().collect::<String>(), 2)
                    .unwrap(),
            );
        }
    }

    fn run_command(&mut self, command: Command) {
        use Command::*;
        match command {
            Flip => self.mem.flip(),
            Left => self.mem.move_left(),
            Right => self.mem.move_right(),
            Read => {
                let input = match self.input_buffer.pop() {
                    Some('1') => true,
                    Some('0') => false,
                    _ => unreachable!(),
                };
                self.mem.set(input);
            }
            Write => {
                let output = match self.mem.get() {
                    true => '1',
                    false => '0',
                };
                self.output_buffer.push(output);
                if self.output_buffer.len() == 8 {
                    self.output.push(
                        u8::from_str_radix(
                            &self.output_buffer.chars().rev().collect::<String>(),
                            2,
                        )
                        .unwrap(),
                    );
                    self.output_buffer.clear();
                }
            }
            While(p) => {
                if self.mem.get() == false {
                    self.ip = p;
                    return;
                }
            }
            EndWhile(p) => {
                if self.mem.get() != false {
                    self.ip = p;
                    return;
                }
            }
        }
        self.ip += 1;
    }

    fn parse_input(input: Vec<u8>) -> String {
        input
            .into_iter()
            .map(|x| format!("{:08b}", x))
            .rev()
            .collect::<String>()
    }

    fn parse_commands(code: &str) -> Vec<Command> {
        let mut stack = Vec::new();
        let mut commands = Vec::new();
        let mut count = 0;

        for c in code.chars() {
            let command = match c {
                '+' => Command::Flip,
                ',' => Command::Read,
                ';' => Command::Write,
                '<' => Command::Left,
                '>' => Command::Right,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_tests() {
        assert_eq!(boolfuck(",;,;,;,;,;,;,;,;", b"a".to_vec()), b"a".to_vec());
    }

    #[test]
    fn example_test_cases() {
        // Hello World Program taken from the official website
        assert_eq!(boolfuck(";;;+;+;;+;+;+;+;+;+;;+;;+;;;+;;+;+;;+;;;+;;+;+;;+;+;;;;+;+;;+;;;+;;+;+;+;;;;;;;+;+;;+;;;+;+;;;+;+;;;;+;+;;+;;+;+;;+;;;+;;;+;;+;+;;+;;;+;+;;+;;+;+;+;;;;+;+;;;+;+;+;", Vec::new()), b"Hello, world!\n", "Your interpreter did not work with the code example provided on the official website");
        // Echo until byte(0) encountered
        assert_eq!(boolfuck(">,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>;>;>;>;>;>;>;>;>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>,>,>,>,>,>,>,>,>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]", b"Codewars\x00".to_vec()), b"Codewars");
        // Two numbers multiplier
        assert_eq!(boolfuck(">,>,>,>,>,>,>,>,>>,>,>,>,>,>,>,>,<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]>[>]+<[+<]>>>>>>>>>[+]>[>]+<[+<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+<<<<<<<<[>]+<[+<]>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]<<<<<<<<<<<<<<<<<<<<<<<<<<[>]+<[+<]>>>>>>>>>[+]>>>>>>>>>>>>>>>>>>+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]<<<<<<<<<<<<<<<<<<+<<<<<<<<+[>+]<[<]>>>>>>>>>[+]+<<<<<<<<+[>+]<[<]>>>>>>>>>]<[+<]>>>>>>>>>>>>>>>>>>>;>;>;>;>;>;>;>;<<<<<<<<", vec![8, 9]), vec![72]);
    }
}
