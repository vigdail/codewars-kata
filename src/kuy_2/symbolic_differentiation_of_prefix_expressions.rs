// Symbolic differentiation of prefix expressions
// https://www.codewars.com/kata/584daf7215ac503d5a0001ae/train/rust

#[derive(Clone, Debug)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Clone, Debug)]
enum Fun {
    Sin,
    Cos,
    Tan,
    Exp,
    Ln,
}

#[derive(Clone, Debug)]
enum Token {
    Number(i32),
    BinOp(BinOp),
    Var,
    Fun(Fun),
    Expression(Vec<Token>),
}

impl From<i32> for Token {
    fn from(n: i32) -> Self {
        Token::Number(n)
    }
}

impl From<BinOp> for Token {
    fn from(op: BinOp) -> Self {
        Token::BinOp(op)
    }
}

impl From<Fun> for Token {
    fn from(fun: Fun) -> Self {
        Token::Fun(fun)
    }
}

impl From<Vec<Token>> for Token {
    fn from(expr: Vec<Token>) -> Self {
        Token::Expression(expr)
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Number(n) => n.to_string(),
            Token::BinOp(op) => op.to_string(),
            Token::Var => "x".to_owned(),
            Token::Fun(fun) => fun.to_string(),
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
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Pow => "^",
        }
        .to_owned()
    }
}

impl ToString for Fun {
    fn to_string(&self) -> String {
        match self {
            Fun::Sin => "sin",
            Fun::Cos => "cos",
            Fun::Tan => "tan",
            Fun::Exp => "exp",
            Fun::Ln => "ln",
        }
        .to_owned()
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
            Token::Fun(fun) => fun.diff(tokens),
            Token::Var => Token::Number(1),
            Token::Expression(expr) => expr
                .iter()
                .map(|t| t.diff(tokens))
                .collect::<Vec<_>>()
                .into(),
        }
    }
}

impl Diff for i32 {
    fn diff(&self, _: &mut Vec<Token>) -> Token {
        return Token::Number(0);
    }
}

impl Diff for BinOp {
    fn diff(&self, tokens: &mut Vec<Token>) -> Token {
        let left = tokens.pop().unwrap();
        let right = tokens.pop().unwrap();
        match self {
            BinOp::Add => vec![BinOp::Add.into(), left.diff(tokens), right.diff(tokens)].into(),
            BinOp::Sub => vec![BinOp::Sub.into(), left.diff(tokens), right.diff(tokens)].into(),
            BinOp::Mul => vec![
                BinOp::Add.into(),
                vec![BinOp::Mul.into(), left.clone(), right.diff(tokens)].into(),
                vec![BinOp::Mul.into(), left.diff(tokens), right].into(),
            ]
            .into(),
            BinOp::Div => vec![
                BinOp::Div.into(),
                // top
                vec![
                    BinOp::Sub.into(),
                    vec![BinOp::Mul.into(), left.clone(), right.diff(tokens)].into(),
                    vec![BinOp::Mul.into(), left.diff(tokens), right.clone()].into(),
                ]
                .into(),
                // bottom
                vec![BinOp::Pow.into(), right, 2.into()].into(),
            ]
            .into(),
            BinOp::Pow => match right {
                Token::Number(n) => vec![BinOp::Mul.into(), (n - 1).into(), left].into(),
                _ => unimplemented!(),
            },
        }
    }
}

// TODO: Test it
impl Diff for Fun {
    fn diff(&self, tokens: &mut Vec<Token>) -> Token {
        let arg = tokens.pop().unwrap();

        match self {
            Fun::Sin => vec![Fun::Cos.into(), arg],
            Fun::Cos => vec![
                BinOp::Mul.into(),
                (-1).into(),
                vec![Fun::Sin.into(), arg].into(),
            ],
            Fun::Tan => vec![
                Token::BinOp(BinOp::Div),
                vec![
                    BinOp::Pow.into(),
                    vec![Token::Fun(Fun::Cos), arg].into(),
                    2.into(),
                ]
                .into(),
            ],
            Fun::Exp => vec![Fun::Exp.into(), arg],
            Fun::Ln => vec![BinOp::Div.into(), 1.into(), arg],
        }
        .into()
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
                if let Some(x) = s.parse::<i32>().ok() {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_to_string() {
        let input = Token::Expression(vec![Token::BinOp(BinOp::Add), Token::Number(2), Token::Var]);
        let result = input.to_string();
        println!("{:?}", result);
        assert_eq!(&result, "(+ 2 x)");

        let input = Token::Expression(vec![Token::Fun(Fun::Tan), Token::Var]);
        let result = input.to_string();
        println!("{:?}", result);
        assert_eq!(&result, "(tan x)");

        let input = Token::Expression(vec![
            Token::BinOp(BinOp::Mul),
            Token::Expression(vec![Token::Fun(Fun::Tan), Token::Var]),
            Token::Number(20),
        ]);
        let result = input.to_string();
        println!("{:?}", result);
        assert_eq!(&result, "(* (tan x) 20)");
    }

    #[test]
    fn simple() {
        assert_eq!(diff("5"), "0");
        assert_eq!(diff("x"), "1");
        assert_eq!(diff("+ x x"), "2");
        assert_eq!(diff("+ 1 x"), "1");
        assert_eq!(diff("/ 1 x"), "/ 1 (^ x 2)");
        assert_eq!(diff("- 5 (+ 10 x)"), "1");
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
