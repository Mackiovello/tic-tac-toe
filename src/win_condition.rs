extern crate itertools;

use win_condition::itertools::Itertools;
use board::Board;

pub fn is_winning_grid(board: Board) -> bool {
    is_column_win(board) || is_row_win(board) || is_diagonal_win(board)
}

fn is_diagonal_win(board: Board) -> bool {
    let mut right_diagonal: Vec<char> = Vec::new();
    let mut left_diagonal: Vec<char> = Vec::new();
    for x in 0..3 {
        right_diagonal.push(board.grid[x][x]);
        left_diagonal.push(board.grid[x][2 - x]);
    }

    unique_non_empty_row(right_diagonal) || unique_non_empty_row(left_diagonal)
}

fn unique_non_empty_row(row: Vec<char>) -> bool {
    row.clone().into_iter().unique().count() == 1 && row[0] != '-'
}

fn is_column_win(board: Board) -> bool {
    let transposed = board.transpose();
    is_row_win(transposed)
}

fn is_row_win(board: Board) -> bool {
    board
        .grid
        .iter()
        .filter(|x| unique_non_empty_row(x.to_vec()))
        .count() > 0
}

#[cfg(test)]
mod win_condition_tests {
    use super::*;

    #[test]
    fn empty_board_is_no_win() {
        assert_eq!(
            is_winning_grid(Board {
                grid: [['-'; 3]; 3],
            }),
            false
        );
    }

    #[test]
    fn complete_row_is_win() {
        let grid = [['O'; 3], ['-'; 3], ['-'; 3]];
        assert_eq!(is_winning_grid(Board { grid }), true);
    }

    #[test]
    fn diagonal_is_win() {
        let grid = [['O', '-', '-'], ['-', 'O', '-'], ['-', '-', 'O']];
        assert_eq!(is_winning_grid(Board { grid }), true);
    }

    #[test]
    fn complete_column_is_win() {
        let grid = [['O', '-', '-'], ['O', '-', '-'], ['O', '-', '-']];
        assert_eq!(is_winning_grid(Board { grid }), true);
    }

    #[test]
    fn combined_row_is_no_win() {
        let grid = [['O', 'X', 'X'], ['-', '-', '-'], ['-', '-', '-']];
        assert_eq!(is_winning_grid(Board { grid }), false);
    }

    #[test]
    fn combined_column_is_no_win() {
        let grid = [['O', '-', '-'], ['X', '-', '-'], ['O', '-', '-']];
        assert_eq!(is_winning_grid(Board { grid }), false);
    }

    #[test]
    fn combined_diagonal_is_no_win() {
        let grid = [['O', '-', '-'], ['-', 'X', '-'], ['-', '-', 'O']];
        assert_eq!(is_winning_grid(Board { grid }), false);
    }
}
