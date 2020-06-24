/// How Green Is My Valley
/// https://www.codewars.com/kata/56e3cd1d93c3d940e50006a4/train/rust

fn make_valley(arr: Vec<i32>) -> Vec<i32> {
    let mut arr = arr;
    arr.sort_by(|a, b| b.cmp(a));

    arr.iter()
        .step_by(2)
        .chain(arr.iter().skip(1).step_by(2).rev())
        .map(|&x| x)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(arr: Vec<i32>, exp: Vec<i32>) -> () {
        println!("arr: {:?}", arr);
        let ans = make_valley(arr);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(
            vec![17, 17, 15, 14, 8, 7, 7, 5, 4, 4, 1],
            vec![17, 15, 8, 7, 4, 1, 4, 5, 7, 14, 17],
        );
        dotest(vec![20, 7, 6, 2], vec![20, 6, 2, 7]);
    }
}
