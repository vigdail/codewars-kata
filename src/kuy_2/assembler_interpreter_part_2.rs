// Assembler interpreter (part II)
// https://www.codewars.com/kata/58e61f3d8ff24f774400002c/train/rust
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
pub struct AssemblerInterpreter {
    ip: usize,
    zero_flag: Option<Ordering>,
    program: Vec<Command>,
    regs: HashMap<String, u32>,
    labels: HashMap<String, usize>,
    stack: Vec<usize>,
    output: String,
}

impl AssemblerInterpreter {
    pub fn interpret(input: &str) -> Option<String> {
        let mut cpu = Self::new();
        cpu.run(input)
    }

    fn new() -> AssemblerInterpreter {
        AssemblerInterpreter {
            ip: 0,
            zero_flag: None,
            program: Vec::new(),
            regs: HashMap::new(),
            labels: HashMap::new(),
            stack: Vec::new(),
            output: String::new(),
        }
    }

    fn run(&mut self, input: &str) -> Option<String> {
        self.parse(input);
        while let Some(command) = self.program.get(self.ip) {
            match command {
                Command::End => return Some(self.output.clone()),
                Command::Ret => {
                    self.ip = self.stack.pop().unwrap();
                }
                Command::UnOp { name, param } => self.process_unop(name.clone(), param.clone()),
                Command::BinOp { name, left, right } => {
                    self.process_binop(name.clone(), left.clone(), right.clone())
                }
                Command::Message(params) => {
                    for param in params {
                        match param {
                            MsgToken::String(s) => {
                                self.output.push_str(&s);
                            }
                            MsgToken::Reg(reg) => {
                                self.output
                                    .push_str(&self.get_value(reg.clone()).to_string());
                            }
                        }
                    }
                    self.ip += 1;
                }
            }
        }

        None
    }

    fn process_unop(&mut self, name: String, param: String) {
        match name.as_str() {
            "call" => {
                let ip = *self.labels.get(&param).unwrap();
                self.stack.push(self.ip + 1);
                self.ip = ip;
                return;
            }
            "inc" => {
                *self.regs.entry(param).or_insert(0) += 1;
            }
            "dec" => {
                *self.regs.entry(param).or_insert(0) -= 1;
            }
            "jmp" => {
                let ip = *self.labels.get(&param).unwrap();
                self.ip = ip;
                return;
            }
            "jne" => {
                if self.zero_flag != Some(Ordering::Equal) {
                    let ip = *self.labels.get(&param).unwrap();
                    self.ip = ip;
                    return;
                }
            }
            "je" => {
                if self.zero_flag == Some(Ordering::Equal) {
                    let ip = *self.labels.get(&param).unwrap();
                    self.ip = ip;
                    return;
                }
            }
            "jge" => match self.zero_flag {
                Some(Ordering::Equal) | Some(Ordering::Greater) => {
                    let ip = *self.labels.get(&param).unwrap();
                    self.ip = ip;
                    return;
                }
                _ => {}
            },
            "jg" => {
                if self.zero_flag == Some(Ordering::Greater) {
                    let ip = *self.labels.get(&param).unwrap();
                    self.ip = ip;
                    return;
                }
            }
            "jle" => match self.zero_flag {
                Some(Ordering::Equal) | Some(Ordering::Less) => {
                    let ip = *self.labels.get(&param).unwrap();
                    self.ip = ip;
                    return;
                }
                _ => {}
            },
            "jl" => {
                if self.zero_flag == Some(Ordering::Less) {
                    let ip = *self.labels.get(&param).unwrap();
                    self.ip = ip;
                    return;
                }
            }
            _ => {}
        }
        self.ip += 1;
    }

    fn process_binop(&mut self, name: String, left: String, right: String) {
        let target = *self.regs.get(&left).unwrap_or(&0);
        let source = self.get_value(right);

        match name.as_str() {
            "mov" => {
                *self.regs.entry(left).or_insert(source) = source;
            }
            "add" => {
                *self.regs.entry(left).or_insert(target) += source;
            }
            "sub" => {
                *self.regs.entry(left).or_insert(target) -= source;
            }
            "mul" => {
                *self.regs.entry(left).or_insert(target) *= source;
            }
            "div" => {
                *self.regs.entry(left).or_insert(target) /= source;
            }
            "cmp" => {
                self.zero_flag = Some(target.cmp(&source));
            }
            _ => {}
        }
        self.ip += 1;
    }

    fn parse(&mut self, input: &str) {
        let mut program = Vec::new();
        let mut labels = HashMap::new();
        for line in input.split("\n") {
            let token = Self::tokenize_line(line);
            if let Some(token) = token {
                match token {
                    Token::Command(command) => {
                        program.push(command);
                    }
                    Token::Label(label) => {
                        labels.insert(label, program.len());
                    }
                }
            };
        }

        self.program = program;
        self.labels = labels;
    }

    fn get_value(&self, input: String) -> u32 {
        if let Ok(n) = input.parse() {
            n
        } else {
            *self.regs.get(&input).unwrap_or(&0)
        }
    }

    fn tokenize_line(line: &str) -> Option<Token> {
        let trimed = line
            .chars()
            .take_while(|&c| c != ';')
            .collect::<String>()
            .trim()
            .to_string();

        let mut parts = trimed.split_whitespace();
        if let Some(cmd) = parts.next() {
            match cmd {
                "mov" | "add" | "sub" | "mul" | "div" | "cmp" => {
                    let left = parts.next().unwrap();
                    let left = left[0..left.len() - 1].to_string();
                    Some(Token::Command(Command::BinOp {
                        name: cmd.to_string(),
                        left,
                        right: parts.next().unwrap().to_string(),
                    }))
                }
                "inc" | "dec" | "jmp" | "jne" | "je" | "jge" | "jg" | "jle" | "jl" | "call" => {
                    let token = Token::Command(Command::UnOp {
                        name: cmd.to_string(),
                        param: parts.next().unwrap().to_string(),
                    });

                    Some(token)
                }
                "ret" => Some(Token::Command(Command::Ret)),
                "end" => Some(Token::Command(Command::End)),
                "msg" => Self::parse_msg(&trimed),
                _ => {
                    let label_regex = Regex::new(".+:$").unwrap();
                    if label_regex.is_match(cmd) {
                        let token = Token::Label(cmd[..cmd.len() - 1].to_string());
                        Some(token)
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    }

    fn parse_msg(line: &str) -> Option<Token> {
        let line = &line["msg".len()..].trim();
        let mut params = Vec::new();
        let mut is_string = false;
        let mut p = String::new();
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            match c {
                '\'' => {
                    if is_string {
                        is_string = false;
                        params.push(MsgToken::String(p.clone()));
                        p.clear();
                    } else {
                        is_string = true;
                    }
                }
                ',' => {
                    if is_string {
                        p.push(c);
                    } else {
                        if p.len() > 0 {
                            params.push(MsgToken::Reg(p.clone()));
                            p.clear();
                        }
                    }
                }
                c if c.is_whitespace() => {
                    if is_string {
                        p.push(c);
                    }
                }
                _ => p.push(c),
            }
        }

        if p.len() > 0 {
            params.push(MsgToken::Reg(p.clone()));
        }

        let token = Token::Command(Command::Message(params));
        Some(token)
    }
}

#[derive(Debug, PartialEq)]
struct Reg {
    name: String,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum MsgToken {
    String(String),
    Reg(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Command {
    UnOp {
        name: String,
        param: String,
    },
    BinOp {
        name: String,
        left: String,
        right: String,
    },
    Message(Vec<MsgToken>),
    End,
    Ret,
}

#[derive(Debug, PartialEq)]
enum Token {
    Command(Command),
    Label(String),
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn simple_test() {
        let simple_programs = &[
            "\n; My first program\nmov  a, 5\ninc  a\ncall function\nmsg  '(5+1)/2 = ', a    ; output message\nend\n\nfunction:\n    div  a, 2\n    ret\n",
            "\nmov   a, 5\nmov   b, a\nmov   c, a\ncall  proc_fact\ncall  print\nend\n\nproc_fact:\n    dec   b\n    mul   c, b\n    cmp   b, 1\n    jne   proc_fact\n    ret\n\nprint:\n    msg   a, '! = ', c ; output text\n    ret\n",
            "\nmov   a, 8            ; value\nmov   b, 0            ; next\nmov   c, 0            ; counter\nmov   d, 0            ; first\nmov   e, 1            ; second\ncall  proc_fib\ncall  print\nend\n\nproc_fib:\n    cmp   c, 2\n    jl    func_0\n    mov   b, d\n    add   b, e\n    mov   d, e\n    mov   e, b\n    inc   c\n    cmp   c, a\n    jle   proc_fib\n    ret\n\nfunc_0:\n    mov   b, c\n    inc   c\n    jmp   proc_fib\n\nprint:\n    msg   'Term ', a, ' of Fibonacci series is: ', b        ; output text\n    ret\n",
            "\nmov   a, 11           ; value1\nmov   b, 3            ; value2\ncall  mod_func\nmsg   'mod(', a, ', ', b, ') = ', d        ; output\nend\n\n; Mod function\nmod_func:\n    mov   c, a        ; temp1\n    div   c, b\n    mul   c, b\n    mov   d, a        ; temp2\n    sub   d, c\n    ret\n",
            "\nmov   a, 81         ; value1\nmov   b, 153        ; value2\ncall  init\ncall  proc_gcd\ncall  print\nend\n\nproc_gcd:\n    cmp   c, d\n    jne   loop\n    ret\n\nloop:\n    cmp   c, d\n    jg    a_bigger\n    jmp   b_bigger\n\na_bigger:\n    sub   c, d\n    jmp   proc_gcd\n\nb_bigger:\n    sub   d, c\n    jmp   proc_gcd\n\ninit:\n    cmp   a, 0\n    jl    a_abs\n    cmp   b, 0\n    jl    b_abs\n    mov   c, a            ; temp1\n    mov   d, b            ; temp2\n    ret\n\na_abs:\n    mul   a, -1\n    jmp   init\n\nb_abs:\n    mul   b, -1\n    jmp   init\n\nprint:\n    msg   'gcd(', a, ', ', b, ') = ', c\n    ret\n",
            "\ncall  func1\ncall  print\nend\n\nfunc1:\n    call  func2\n    ret\n\nfunc2:\n    ret\n\nprint:\n    msg 'This program should return null'\n",
            "\nmov   a, 2            ; value1\nmov   b, 10           ; value2\nmov   c, a            ; temp1\nmov   d, b            ; temp2\ncall  proc_func\ncall  print\nend\n\nproc_func:\n    cmp   d, 1\n    je    continue\n    mul   c, a\n    dec   d\n    call  proc_func\n\ncontinue:\n    ret\n\nprint:\n    msg a, '^', b, ' = ', c\n    ret\n"
            ];

        let expected = &[
            Some(String::from("(5+1)/2 = 3")),
            Some(String::from("5! = 120")),
            Some(String::from("Term 8 of Fibonacci series is: 21")),
            Some(String::from("mod(11, 3) = 2")),
            Some(String::from("gcd(81, 153) = 9")),
            None,
            Some(String::from("2^10 = 1024")),
        ];

        for (prg, exp) in simple_programs.iter().zip(expected) {
            println!("{:#?}", prg.split("\n").collect::<Vec<_>>());
            let actual = AssemblerInterpreter::interpret(*prg);
            assert_eq!(actual, *exp);
        }
    }

    #[test]
    fn tokenize_line_test() {
        let input = "mov a, b";
        let expected = Token::Command(Command::BinOp {
            name: String::from("mov"),
            left: "a".to_string(),
            right: "b".to_string(),
        });
        assert_eq!(AssemblerInterpreter::tokenize_line(input), Some(expected));
        let input = "   add   a,   b  ; comment";
        let expected = Token::Command(Command::BinOp {
            name: String::from("add"),
            left: "a".to_string(),
            right: "b".to_string(),
        });
        assert_eq!(AssemblerInterpreter::tokenize_line(input), Some(expected));

        let input = "inc a";
        let expected = Token::Command(Command::UnOp {
            name: String::from("inc"),
            param: "a".to_string(),
        });
        assert_eq!(AssemblerInterpreter::tokenize_line(input), Some(expected));

        let input = "ret";
        let expected = Token::Command(Command::Ret);
        assert_eq!(AssemblerInterpreter::tokenize_line(input), Some(expected));

        let input = "; ret";
        assert_eq!(AssemblerInterpreter::tokenize_line(input), None);
    }

    #[test]
    fn tokenize_label() {
        let input = "some_label:";
        let expected = Token::Label("some_label".to_string());
        assert_eq!(AssemblerInterpreter::tokenize_line(input), Some(expected));
    }

    #[test]
    fn parse_msg_test() {
        let input = "msg 'hello'";
        let expected = Token::Command(Command::Message(vec![MsgToken::String(
            "hello".to_string(),
        )]));
        assert_eq!(AssemblerInterpreter::parse_msg(input), Some(expected));

        let input = "msg 'hello', a";
        let expected = Token::Command(Command::Message(vec![
            MsgToken::String("hello".to_string()),
            MsgToken::Reg("a".to_string()),
        ]));
        assert_eq!(AssemblerInterpreter::parse_msg(input), Some(expected));

        let input = "msg 'hello', a, 'and some string'";
        let expected = Token::Command(Command::Message(vec![
            MsgToken::String("hello".to_string()),
            MsgToken::Reg("a".to_string()),
            MsgToken::String("and some string".to_string()),
        ]));
        assert_eq!(AssemblerInterpreter::parse_msg(input), Some(expected));

        let input = "msg 'hello', a, 'and some string', b";
        let expected = Token::Command(Command::Message(vec![
            MsgToken::String("hello".to_string()),
            MsgToken::Reg("a".to_string()),
            MsgToken::String("and some string".to_string()),
            MsgToken::Reg("b".to_string()),
        ]));
        assert_eq!(AssemblerInterpreter::parse_msg(input), Some(expected));
    }

    #[test]
    fn tokenize_input() {
        let input =
            "\nmov   a, 5\nmov   b, a\nmov   c, a\ncall  proc_fact\ncall  print\nend\n\nproc_fact:\n    dec   b\n    mul   c, b\n    cmp   b, 1\n    jne   proc_fact\n    ret\n\nprint:\n    msg   a, '! = ', c ; output text\n    ret\n";
        let mut cpu = AssemblerInterpreter::new();
        let actual = cpu.run(input);
        println!("{:#?}", cpu);
        assert_eq!(actual, Some(String::from("5! = 120")));
    }
}
