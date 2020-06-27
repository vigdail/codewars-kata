// Catalog
// https://www.codewars.com/kata/59d9d8cb27ee005972000045/train/rust

#[derive(PartialEq, Debug)]
struct Prod {
    name: String,
    prx: String,
    qty: String,
}

impl std::fmt::Display for Prod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} > prx: ${} qty: {}", self.name, self.prx, self.qty)
    }
}

fn catalog(s: &str, article: &str) -> String {
    let r = s
        .split("\n")
        .filter_map(parse_prod)
        .filter(|p| p.name.contains(article))
        .map(|p| format!("{}", p))
        .collect::<Vec<_>>()
        .join("\n");
    if r.len() > 0 {
        r
    } else {
        "Nothing".to_string()
    }
}

fn parse_prod(s: &str) -> Option<Prod> {
    let (s, _) = parse_tag_name(s).ok()?;
    let (s, name) = parse_tag_content(&s).ok()?;
    let (s, prx) = parse_tag_content(&s).ok()?;
    let (_, qty) = parse_tag_content(&s).ok()?;

    Some(Prod { name, prx, qty })
}

fn parse_tag_content<'a>(input: &'a str) -> Result<(&'a str, String), &'a str> {
    let (input, tag) = parse_tag_name(input)?;
    let name = consume_while(input, |x| x != '<');
    let input = &input[name.len() + tag.len() + 2..];
    Ok((input, name))
}

fn parse_tag_name<'a>(input: &'a str) -> Result<(&'a str, String), &'a str> {
    if Some('<') != input.chars().next() {
        return Err(input);
    }

    let result = consume_while(input, |x| x != '>');

    Ok((&input[result.len() + 1..], result))
}

fn consume_while<'a, F>(input: &'a str, condition: F) -> String
where
    F: Fn(char) -> bool,
{
    let mut iter = input.chars().peekable();

    let mut result = String::new();
    while iter.peek().map_or(false, |c| condition(*c)) {
        result.push(iter.next().unwrap());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    fn s() -> String {
        let a: Vec<&str> = vec![
            "<prod><name>drill</name><prx>99</prx><qty>5</qty></prod>",
            "<prod><name>hammer</name><prx>10</prx><qty>50</qty></prod>",
            "<prod><name>screwdriver</name><prx>5</prx><qty>51</qty></prod>",
            "<prod><name>table saw</name><prx>1099.99</prx><qty>5</qty></prod>",
            "<prod><name>saw</name><prx>9</prx><qty>10</qty></prod>",
            "<prod><name>chair</name><prx>100</prx><qty>20</qty></prod>",
            "<prod><name>fan</name><prx>50</prx><qty>8</qty></prod>",
            "<prod><name>wire</name><prx>10.8</prx><qty>15</qty></prod>",
            "<prod><name>battery</name><prx>150</prx><qty>12</qty></prod>",
            "<prod><name>pallet</name><prx>10</prx><qty>50</qty></prod>",
            "<prod><name>wheel</name><prx>8.80</prx><qty>32</qty></prod>",
            "<prod><name>extractor</name><prx>105</prx><qty>17</qty></prod>",
            "<prod><name>bumper</name><prx>150</prx><qty>3</qty></prod>",
            "<prod><name>ladder</name><prx>112</prx><qty>12</qty></prod>",
            "<prod><name>hoist</name><prx>13.80</prx><qty>32</qty></prod>",
            "<prod><name>platform</name><prx>65</prx><qty>21</qty></prod>",
            "<prod><name>car wheel</name><prx>505</prx><qty>7</qty></prod>",
            "<prod><name>bicycle wheel</name><prx>150</prx><qty>11</qty></prod>",
            "<prod><name>big hammer</name><prx>18</prx><qty>12</qty></prod>",
            "<prod><name>saw for metal</name><prx>13.80</prx><qty>32</qty></prod>",
            "<prod><name>wood pallet</name><prx>65</prx><qty>21</qty></prod>",
            "<prod><name>circular fan</name><prx>80</prx><qty>8</qty></prod>",
            "<prod><name>exhaust fan</name><prx>62</prx><qty>8</qty></prod>",
            "<prod><name>window fan</name><prx>62</prx><qty>8</qty></prod>",
        ];
        return a.join("\n\n");
    }

    fn dotest(s: &str, article: &str, exp: &str) -> () {
        println!("s:{:?}", s);
        println!("article:{:?}", article);
        let ans = catalog(s, article);
        println!("actual: {:?}", ans);
        println!("expect: {:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }
    #[test]
    fn basic_tests() {
        let s = &s();
        dotest(s, "ladder", "ladder > prx: $112 qty: 12");
        dotest(s, "ladder", "ladder > prx: $112 qty: 12");
        dotest(s, "apples", "Nothing");
    }

    #[test]
    fn multiple_entires_test() {
        let s ="<prod><name>fan</name><prx>24.8</prx><qty>14</qty></prod>\n\n<prod><name>bumper</name><prx>110.7</prx><qty>9</qty></prod>\n\n<prod><name>table saw</name><prx>17.6</prx><qty>12</qty></prod>\n\n<prod><name>big hammer</name><prx>110.7</prx><qty>14</qty></prod>\n\n<prod><name>exhaust fan</name><prx>120.90</prx><qty>9</qty></prod>";
        dotest(
            s,
            "fan",
            "fan > prx: $24.8 qty: 14\nexhaust fan > prx: $120.90 qty: 9",
        );
    }

    #[test]
    fn test_parse_prod() {
        let s = "<prod><name>drill</name><prx>99</prx><qty>5</qty></prod>";
        let prod = parse_prod(s);
        let expect = Some(Prod {
            name: "drill".to_string(),
            prx: "99".to_string(),
            qty: "5".to_string(),
        });
        assert_eq!(prod, expect);
    }
}
