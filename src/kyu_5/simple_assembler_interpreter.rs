// Simple assembler interpreter
// https://www.codewars.com/kata/58e24788e24ddee28e000053/train/rust

use std::collections::HashMap;

pub fn simple_assembler(program: Vec<&str>) -> HashMap<String, i64> {
    let mut cpu = Interpreter::new(program);
    cpu.process()
}

struct Interpreter {
    p: usize,
    commands: Vec<Command>,
    registers: HashMap<String, i64>,
}

impl Interpreter {
    pub fn new(program: Vec<&str>) -> Interpreter {
        let registers = HashMap::new();
        let commands = program
            .iter()
            .filter_map(|&s| Self::parse_command(s))
            .collect::<Vec<_>>();

        Interpreter {
            registers,
            commands,
            p: 0,
        }
    }

    pub fn process(&mut self) -> HashMap<String, i64> {
        while self.p < self.commands.len() {
            self.process_command();
        }

        self.registers.clone()
    }
    fn process_command(&mut self) {
        let command = self.commands.get(self.p).unwrap();

        use Arg::*;
        use Command::*;
        match command {
            Mov(Register(left), right) => {
                let &n = match right {
                    Value(i) => i,
                    Register(r) => self.registers.get(r).unwrap(),
                };
                self.registers.insert(left.clone(), n);
                self.p += 1;
            }
            Inc(Register(reg)) => {
                let &n = self.registers.get(reg).unwrap();
                self.registers.insert(reg.clone(), n + 1);

                self.p += 1;
            }
            Dec(Register(reg)) => {
                let &n = self.registers.get(reg).unwrap();
                self.registers.insert(reg.clone(), n - 1);
                self.p += 1;
            }
            Jnz(left, Value(right)) => {
                let &n = match left {
                    Register(reg) => self.registers.get(reg).unwrap(),
                    Value(i) => i,
                };
                if n != 0 {
                    self.p = (self.p as i64 + right) as usize;
                } else {
                    self.p += 1;
                }
            }
            _ => unreachable!(),
        };
    }

    fn parse_command(s: &str) -> Option<Command> {
        let mut s = s.split(" ");
        let res = match s.next() {
            Some("mov") => Command::Mov(Self::parse_arg(s.next()?), Self::parse_arg(s.next()?)),
            Some("inc") => Command::Inc(Self::parse_arg(s.next()?)),
            Some("dec") => Command::Dec(Self::parse_arg(s.next()?)),
            Some("jnz") => Command::Jnz(Self::parse_arg(s.next()?), Self::parse_arg(s.next()?)),
            _ => return None,
        };

        Some(res)
    }

    fn parse_arg(arg: &str) -> Arg {
        match arg.parse::<i64>().ok() {
            Some(n) => Arg::Value(n),
            None => Arg::Register(arg.to_string()),
        }
    }
}

enum Arg {
    Register(String),
    Value(i64),
}

enum Command {
    Mov(Arg, Arg),
    Inc(Arg),
    Dec(Arg),
    Jnz(Arg, Arg),
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! map {
        ($($key:expr => $value:expr),*) => {{
             let mut map = HashMap::new();
             $(
                 map.insert($key.to_string(), $value);
             )*
             map
        }};
    }

    #[test]
    fn short_tests() {
        let program = vec!["mov a 5", "inc a", "dec a", "dec a", "jnz a -1", "inc a"];
        let expected = map! { "a" => 1 };
        compare_registers(expected, simple_assembler(program));
    }

    #[test]
    fn second_test() {
        let program = vec![
            "mov c 1", "mov b 0", "mov a 2", "dec a", "inc b", "jnz a -2", "dec c", "mov a b",
            "jnz c -5", "jnz 0 1", "mov c a",
        ];
        let expected = map! { "a" => 2, "c" => 2, "b" => 2};
        compare_registers(expected, simple_assembler(program));
    }

    #[test]
    fn empty_program_test() {
        let program = vec![""];
        let expected = HashMap::new();
        compare_registers(expected, simple_assembler(program));
    }

    #[test]
    fn mov_test() {
        let program = vec!["mov a 1"];
        let expected = map! { "a" => 1 };
        compare_registers(expected, simple_assembler(program));

        let program = vec!["mov a 1", "mov b 2", "mov a b"];
        let expected = map! { "a" => 2, "b" => 2 };
        compare_registers(expected, simple_assembler(program));
    }

    #[test]
    fn inc_test() {
        let program = vec!["mov a 1", "inc a"];
        let expected = map! { "a" => 2 };
        compare_registers(expected, simple_assembler(program));
    }

    #[test]
    fn dec_test() {
        let program = vec!["mov a 2", "dec a"];
        let expected = map! { "a" => 1 };
        compare_registers(expected, simple_assembler(program));
    }

    fn compare_registers(expected: HashMap<String, i64>, actual: HashMap<String, i64>) {
        let result = expected
            .iter()
            .all(|(key, value)| actual.get(key).map(|v| v == value).unwrap_or(false));
        assert!(
            result,
            "Expected the registers to be like that:\n{:#?}\n\nBut got this:\n{:#?}\n",
            expected, actual
        )
    }
}
