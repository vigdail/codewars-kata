/// Hello World
/// https://www.codewars.com/kata/5968644eea5c541501000110/train/rust

pub enum Input {
    Text(String),
    Number(usize),
    Decimal(f32),
    Truth(bool),
    List(Vec<String>),
}

pub fn hello(input: Input) -> String {
    use Input::*;
    match input {
        Number(n) => format!("Hello {}", n),
        Decimal(x) => format!("Hello {}", x),
        Text(x) => format!("Hello {}", x),
        Truth(x) => format!("Hello {}", x),
        List(x) => format!("Hello {:?}", x),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Input::*;

    #[test]
    fn test_string() {
        assert_eq!(hello(Text("Noclip".to_string())), "Hello Noclip");
        assert_eq!(hello(Text("Alice".to_string())), "Hello Alice");
        assert_eq!(hello(Text("Bob".to_string())), "Hello Bob");
    }
}
