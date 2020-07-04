// Chess Fun #1: Chess Board Cell Color
// https://www.codewars.com/kata/5894134c8afa3618c9000146/train/rust

pub fn chessboard_cell_color(cell1: &str, cell2: &str) -> bool {
    get_color(cell1) == get_color(cell2)
}

fn get_color(cell: &str) -> u8 {
    cell.chars().map(|c| c as u8).sum::<u8>() % 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(chessboard_cell_color("A1", "C3"), true);
        assert_eq!(chessboard_cell_color("A1", "H3"), false);
        assert_eq!(chessboard_cell_color("A1", "A2"), false);
        assert_eq!(chessboard_cell_color("A1", "C1"), true);
        assert_eq!(chessboard_cell_color("A1", "A1"), true);
    }
}
