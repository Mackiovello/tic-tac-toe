extern crate itertools;

use win_condition::itertools::Itertools;

pub fn is_winning_grid(grid: [[char; 3]; 3]) -> bool {
    is_column_win(grid) || is_row_win(grid) || is_diagonal_win(grid)    
}

fn is_diagonal_win(grid: [[char; 3]; 3]) -> bool {
    let mut right_diagonal: Vec<char> = Vec::new();
    let mut left_diagonal: Vec<char> = Vec::new();
    for x in 0..3 {
        right_diagonal.push(grid[x][x]);
        left_diagonal.push(grid[x][2-x]);
    }

    unique_non_empty_row(right_diagonal) || unique_non_empty_row(left_diagonal)
}

fn unique_non_empty_row(row: Vec<char>) -> bool {
    row.clone()
        .into_iter()
        .unique()
        .count() == 1 && 
        row[0] != '-'
}

fn is_column_win(grid: [[char; 3]; 3]) -> bool {
    let transposed = transpose_board(grid);
    is_row_win(transposed)
}

fn transpose_board(grid: [[char; 3]; 3]) -> [[char; 3]; 3] {
    let mut transposed = [['-'; 3]; 3];

    for i in 0..3{
        for j in 0..3{
            transposed[i][j] = grid[j][i];
        }
    }

    transposed
}

fn is_row_win(grid: [[char; 3]; 3]) -> bool {
    grid
        .iter()
        .filter(|x| unique_non_empty_row(x.to_vec()))
        .count() > 0
}

#[cfg(test)]
mod win_condition_tests {
    use super::*;

    #[test]
    fn empty_board_is_no_win() {
        assert_eq!(is_winning_grid([['-'; 3]; 3]), false);
    }

    #[test]
    fn complete_row_is_win() {
        let grid = [['O'; 3], ['-'; 3], ['-'; 3]];
        assert_eq!(is_winning_grid(grid), true);
    }

    #[test]
    fn diagonal_is_win() {
        let grid = [
            ['O', '-', '-'],
            ['-', 'O', '-'],
            ['-', '-', 'O'],                
        ];
        assert_eq!(is_winning_grid(grid), true);
    }

    #[test]
    fn complete_column_is_win() {
        let grid = [
            ['O', '-', '-'],
            ['O', '-', '-'],
            ['O', '-', '-'],
        ];
        assert_eq!(is_winning_grid(grid), true);
    }

    #[test]
    fn combined_row_is_no_win() {
        let grid = [
            ['O', 'X', 'X'],
            ['-', '-', '-'],
            ['-', '-', '-'],
        ];
        assert_eq!(is_winning_grid(grid), false);
    }

    #[test]
    fn combined_column_is_no_win() {
        let grid = [
            ['O', '-', '-'],
            ['X', '-', '-'],
            ['O', '-', '-'],
        ];
        assert_eq!(is_winning_grid(grid), false);
    }

    #[test]
    fn combined_diagonal_is_no_win() {
        let grid = [
            ['O', '-', '-'],
            ['-', 'X', '-'],
            ['-', '-', 'O'],
        ];
        assert_eq!(is_winning_grid(grid), false);
    }
}