/// https://www.codewars.com/kata/58708934a44cfccca60000c4/train/rust
/// RoboScript #1 - Implement Syntax Highlighting
use std::iter::Peekable;
use std::str::Chars;
enum Token {
    F(String),
    L(String),
    R(String),
    Digit(String),
    Other(String),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (color, data) = match self {
            Token::F(data) => ("pink", data),
            Token::L(data) => ("red", data),
            Token::R(data) => ("green", data),
            Token::Digit(data) => ("orange", data),
            Token::Other(data) => return write!(f, "{}", data),
        };
        write!(f, "<span style=\"color: {}\">{}</span>", color, data)
    }
}

fn consume_while<'a, F>(iter: &mut Peekable<Chars<'a>>, condition: F) -> String
where
    F: Fn(char) -> bool,
{
    let mut result = String::new();
    while iter.peek().map_or(false, |c| condition(*c)) {
        result.push(iter.next().unwrap());
    }

    result
}

pub fn highlight(code: &str) -> String {
    let mut r = Vec::<Token>::new();
    let mut iter = code.chars().peekable();
    while let Some(c) = iter.peek() {
        let token = match c {
            'F' => Token::F(consume_while(&mut iter, |x| x == 'F')),
            'R' => Token::R(consume_while(&mut iter, |x| x == 'R')),
            'L' => Token::L(consume_while(&mut iter, |x| x == 'L')),
            _ => {
                if c.is_digit(10) {
                    Token::Digit(consume_while(&mut iter, |x| x.is_digit(10)))
                } else {
                    Token::Other(iter.next().unwrap().to_string())
                }
            }
        };
        r.push(token);
    }
    r.iter().map(|x| format!("{}", x)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    macro_rules! assert_highlight {
        ($code:expr , $expected:expr $(,)*) => {{
            let actual = highlight($code);
            let expected = $expected;
            println!("Code without syntax highlighting: {}", $code);
            println!("Your code with syntax highlighting: {}", actual);
            println!("Expected syntax highlighting: {}", expected);
            assert_eq!(actual, expected);
        }};
    }

    #[test]
    fn examples_in_description() {
        assert_highlight!("F", r#"<span style="color: pink">F</span>"#,);
        assert_highlight!("FF", r#"<span style="color: pink">FF</span>"#,);
        assert_highlight!("FFF", r#"<span style="color: pink">FFF</span>"#,);
        assert_highlight!(
            "FRL",
            r#"<span style="color: pink">F</span><span style="color: green">R</span><span style="color: red">L</span>"#,
        );
        assert_highlight!(
            "F3RF5LF7",
            r#"<span style="color: pink">F</span><span style="color: orange">3</span><span style="color: green">R</span><span style="color: pink">F</span><span style="color: orange">5</span><span style="color: red">L</span><span style="color: pink">F</span><span style="color: orange">7</span>"#,
        );
        assert_highlight!(
            "FFFR345F2LL",
            r#"<span style="color: pink">FFF</span><span style="color: green">R</span><span style="color: orange">345</span><span style="color: pink">F</span><span style="color: orange">2</span><span style="color: red">LL</span>"#,
        );
    }
}
