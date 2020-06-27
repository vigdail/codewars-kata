/// https://www.codewars.com/kata/56dbe0e313c2f63be4000b25/train/rust
/// Moves in squared strings (I)

pub fn hor_mirror(s: String) -> String {
    s.split('\n').rev().collect::<Vec<_>>().join("\n")
}
pub fn vert_mirror(s: String) -> String {
    s.split('\n')
        .map(|x| x.chars().rev().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}
// first parameter: dots have to be replaced by function of one variable
pub fn oper(f: impl Fn(String) -> String, s: String) -> String {
    f(s)
}

#[cfg(test)]
mod test {
    use super::*;

    fn testing1(s: &str, exp: &str) -> () {
        assert_eq!(oper(hor_mirror, s.to_string()), exp)
    }
    fn testing2(s: &str, exp: &str) -> () {
        assert_eq!(oper(vert_mirror, s.to_string()), exp)
    }

    #[test]
    fn basics_oper() {
        testing1("lVHt\nJVhv\nCSbg\nyeCt", "yeCt\nCSbg\nJVhv\nlVHt");
        testing1("njMK\ndbrZ\nLPKo\ncEYz", "cEYz\nLPKo\ndbrZ\nnjMK");
        testing1("QMxo\ntmFe\nWLUG\nowoq", "owoq\nWLUG\ntmFe\nQMxo");

        testing2(
            "hSgdHQ\nHnDMao\nClNNxX\niRvxxH\nbqTVvA\nwvSyRu",
            "QHdgSh\noaMDnH\nXxNNlC\nHxxvRi\nAvVTqb\nuRySvw",
        );
        testing2(
            "IzOTWE\nkkbeCM\nWuzZxM\nvDddJw\njiJyHF\nPVHfSx",
            "EWTOzI\nMCebkk\nMxZzuW\nwJddDv\nFHyJij\nxSfHVP",
        );
        testing2("cuQW\nxOuD\nfZwp\neqFx", "WQuc\nDuOx\npwZf\nxFqe");
    }
}
