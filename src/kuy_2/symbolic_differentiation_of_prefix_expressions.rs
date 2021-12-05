use std::fmt::Display;

// Symbolic differentiation of prefix expressions
// https://www.codewars.com/kata/584daf7215ac503d5a0001ae/train/rust

#[derive(Clone, Debug)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Clone, Debug)]
enum UnOp {
    Sin,
    Cos,
    // @TODO
}
#[derive(Clone, Debug)]
enum Token {
    Number(u32),
    BinOp(BinOp),
    Var,
    UnOp(UnOp),
    Expression(Vec<Token>),
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Number(n) => n.to_string(),
            Token::BinOp(op) => op.to_string(),
            Token::Var => "x".to_owned(),
            Token::UnOp(_) => todo!(),
            Token::Expression(exp) => format!(
                "({})",
                exp.iter()
                    .map(|t| t.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
        }
    }
}

impl ToString for BinOp {
    fn to_string(&self) -> String {
        match self {
            BinOp::Add => "+".to_owned(),
            BinOp::Sub => "-".to_owned(),
            BinOp::Mul => "*".to_owned(),
            BinOp::Div => "/".to_owned(),
        }
    }
}

trait Diff {
    fn diff(&self, tokens: &mut Vec<Token>) -> Token;
}

impl Diff for Token {
    fn diff(&self, tokens: &mut Vec<Token>) -> Token {
        match self {
            Token::Number(n) => n.diff(tokens),
            Token::BinOp(op) => op.diff(tokens),
            Token::Var => Token::Number(1),
            _ => unimplemented!(),
        }
    }
}

impl Diff for u32 {
    fn diff(&self, _: &mut Vec<Token>) -> Token {
        return Token::Number(0);
    }
}

impl Diff for BinOp {
    fn diff(&self, tokens: &mut Vec<Token>) -> Token {
        let left = tokens.pop().unwrap();
        let right = tokens.pop().unwrap();
        match self {
            BinOp::Add => Token::Expression(vec![
                Token::BinOp(BinOp::Add),
                left.diff(tokens),
                right.diff(tokens),
            ]),
            BinOp::Sub => Token::Expression(vec![
                Token::BinOp(BinOp::Sub),
                left.diff(tokens),
                right.diff(tokens),
            ]),
            BinOp::Mul => Token::Expression(vec![
                Token::BinOp(BinOp::Add),
                Token::Expression(vec![
                    Token::BinOp(BinOp::Mul),
                    left.clone(),
                    right.diff(tokens),
                ]),
                Token::Expression(vec![Token::BinOp(BinOp::Mul), left.diff(tokens), right]),
            ]),
            BinOp::Div => Token::Expression(vec![
                Token::BinOp(BinOp::Div),
                // top
                Token::BinOp(BinOp::Sub),
                Token::BinOp(BinOp::Mul),
                left.clone(),
                right.diff(tokens),
                Token::BinOp(BinOp::Mul),
                left.diff(tokens),
                right.clone(),
                // bottom
                Token::BinOp(BinOp::Mul),
                right.clone(),
                right,
            ]),
        }
    }
}

fn parse(expr: &str) -> Vec<Token> {
    expr.split(' ')
        .filter_map(|s| match s {
            "+" => Some(Token::BinOp(BinOp::Add)),
            "-" => Some(Token::BinOp(BinOp::Sub)),
            "*" => Some(Token::BinOp(BinOp::Mul)),
            "/" => Some(Token::BinOp(BinOp::Div)),
            "x" => Some(Token::Var),
            _ => {
                if let Some(x) = s.parse::<u32>().ok() {
                    Some(Token::Number(x))
                } else {
                    None
                }
            }
        })
        .collect()
}

pub fn diff(expr: &str) -> String {
    let mut res: Vec<Token> = vec![];
    let mut tokens = parse(expr).into_iter().rev().collect::<Vec<_>>();
    println!("Input: {:?}", tokens);
    while !tokens.is_empty() {
        let token = tokens.pop().unwrap();

        res.push(token.diff(&mut tokens));
    }

    println!("{:?}", res);

    res.iter()
        .map(|t| t.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_to_string() {
        let input = "* 2 x".to_owned();
        let result = diff(&input);
        println!("{:?}", result);
        assert_eq!(&result, "(+ (* 2 1) (* 0 x))")
    }

    #[test]
    fn simple() {
        assert_eq!(diff("5"), "0");
        assert_eq!(diff("x"), "1");
        assert_eq!(diff("+ x x"), "1");
        assert_eq!(diff("+ 1 x"), "1");
        assert_eq!(diff("/ 1 x"), "1");
        assert_eq!(diff("- 5 + 10 x"), "0");
    }

    #[test]
    #[ignore]
    fn test_fixed() {
        assert_eq!(diff("5"), "0");
        assert_eq!(diff("x"), "1");
        assert_eq!(diff("5"), "0");
        assert_eq!(diff("(+ x x)"), "2");
        assert_eq!(diff("(- x x)"), "0");
        assert_eq!(diff("(* x 2)"), "2");
        assert_eq!(diff("(/ x 2)"), "0.5");
        assert_eq!(diff("(^ x 2)"), "(* 2 x)");
        assert_eq!(diff("(cos x)"), "(* -1 (sin x))");
        assert_eq!(diff("(sin x)"), "(cos x)");

        // assert_eq!(diff("(tan x)"), "(+ 1 (^ (tan x) 2))");

        let result = diff("(tan x)");
        assert!(
            result == "(+ 1 (^ (tan x) 2))" || result == "(/ 1 (^ (cos x) 2))",
            "expected (+ 1 (^ (tan x) 2)) or (/ 1 (^ (cos x) 2))"
        );

        assert_eq!(diff("(exp x)"), "(exp x)");
        assert_eq!(diff("(ln x)"), "(/ 1 x)");
        assert_eq!(diff("(+ x (+ x x))"), "3");
        assert_eq!(diff("(- (+ x x) x)"), "1");
        assert_eq!(diff("(* 2 (+ x 2))"), "2");
        assert_eq!(diff("(/ 2 (+ 1 x))"), "(/ -2 (^ (+ 1 x) 2))");
        assert_eq!(diff("(cos (+ x 1))"), "(* -1 (sin (+ x 1)))");

        let result = diff("(cos (* 2 x))");
        assert!(
        result == "(* 2 (* -1 (sin (* 2 x))))"
            || result == "(* -2 (sin (* 2 x)))"
            || result == "(* (* -1 (sin (* 2 x))) 2)",
        "expected (* 2 (* -1 (sin (* 2 x)))) or (* -2 (sin (* 2 x))) or (* (* -1 (sin (* 2 x))) 2)"
    );

        assert_eq!(diff("(sin (+ x 1))"), "(cos (+ x 1))");
        assert_eq!(diff("(sin (* 2 x))"), "(* 2 (cos (* 2 x)))");

        // assert_eq!(diff("(tan (* 2 x))"), "(* 2 (+ 1 (^ (tan (* 2 x)) 2)))");

        let result = diff("(tan (* 2 x))");
        assert!(
            result == "(* 2 (+ 1 (^ (tan (* 2 x)) 2)))"
                || result == "(* 2 (/ 1 (^ (cos (* 2 x)) 2)))",
            "expected (* 2 (+ 1 (^ (tan (* 2 x)) 2))) or (* 2 (/ 1 (^ (cos (* 2 x)) 2)))"
        );

        assert_eq!(diff("(exp (* 2 x))"), "(* 2 (exp (* 2 x)))");
        assert_eq!(diff(&diff("(sin x)")), "(* -1 (sin x))");
        assert_eq!(diff(&diff("(exp x)")), "(exp x)");

        let result = diff(&diff("(^ x 3)"));
        assert!(
            result == "(* 3 (* 2 x))" || result == "(* 6 x)",
            "expected (* 3 (* 2 x)) or (* 6 x)"
        );
    }
}
