/// Hello World
/// https://www.codewars.com/kata/5968644eea5c541501000110/train/rust

enum Input {
    Text(String),
    Number(usize),
    Decimal(f32),
    Truth(bool),
    List(Vec<String>),
}

fn hello(input: Input) -> String {
    use Input::*;
    match input {
        Number(n) => format!("Hello {}", n),
        Decimal(x) => format!("Hello {}", x),
        Text(x) => format!("Hello {}", x),
        Truth(x) => format!("Hello {}", x),
        List(x) => format!("Hello {:?}", x),
    }
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

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
    fn test_num() {
        assert_eq!(hello(Number(42)), "Hello 42");
        assert_eq!(hello(Decimal(42.0)), "Hello 42.0");
    }
}
