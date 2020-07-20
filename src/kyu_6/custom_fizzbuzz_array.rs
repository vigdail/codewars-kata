// https://www.codewars.com/kata/5355a811a93a501adf000ab7/train/rust
// Custom FizzBuzz Array

pub fn fizz_buzz_custom_solver(
    string_one: &str,
    string_two: &str,
    num_one: usize,
    num_two: usize,
) -> Vec<String> {
    (1..=100)
        .map(|i| {
            let mut r = String::new();
            if i % num_one == 0 {
                r.push_str(string_one);
            }
            if i % num_two == 0 {
                r.push_str(string_two);
            }
            if r.len() == 0 {
                return i.to_string();
            }
            r
        })
        .collect()
}

#[macro_export]
macro_rules! fizz_buzz_custom {
    () => {
        fizz_buzz_custom_solver("Fizz", "Buzz", 3, 5)
    };
    ($str_one:expr) => {
        fizz_buzz_custom_solver($str_one, "Buzz", 3, 5)
    };
    ($str_one:expr, $str_two:expr) => {
        fizz_buzz_custom_solver($str_one, $str_two, 3, 5)
    };
    ($str_one:expr, $str_two:expr, $num_one:expr) => {
        fizz_buzz_custom_solver($str_one, $str_two, $num_one, 5)
    };
    ($str_one:expr, $str_two:expr, $num_one:expr, $num_two:expr) => {
        fizz_buzz_custom_solver($str_one, $str_two, $num_one, $num_two)
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(fizz_buzz_custom!()[3], "4".to_string());
        assert_eq!(fizz_buzz_custom!()[15], "16".to_string());
        assert_eq!(fizz_buzz_custom!()[14], "FizzBuzz".to_string());
    }
}
