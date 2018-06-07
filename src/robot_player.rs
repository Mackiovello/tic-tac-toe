use board::Board;
use players::Player;

#[derive(PartialEq, Clone, Copy)]
pub struct RobotPlayer {
    pub sign: char,
}

impl RobotPlayer {
    fn get_opponent_sign(&self) -> char {
        if self.get_sign() == 'X' {
            'O'
        } else {
            'X'
        }
    }
}

impl Player for RobotPlayer {
    fn get_sign(&self) -> char {
        self.sign
    }

    fn get_coordinate(&self, board: Board) -> Result<(usize, usize), String> {
        if let Some(winning_coordinate) = winning_move(board, self.sign) {
            return Ok(winning_coordinate);
        }

        if let Some(blocking_coordinate) = winning_move(board, self.get_opponent_sign()) {
            return Ok(blocking_coordinate);
        }

        if let Some(fork_coordinate) = fork_move(board, self.sign) {
            return Ok(fork_coordinate);
        }

        Err("No choice found".to_string())
    }
}

fn fork_move(board: Board, sign: char) -> Option<(usize, usize)> {
    let empty_squares: Vec<(usize, usize)> = get_empty_squares(board, sign);

    for square in empty_squares {
        let attempted_grid = board.add_value(square, sign);
        if two_winning_moves(attempted_grid.unwrap(), sign) {
            return Some(square);
        }
    }

    None
}

fn two_winning_moves(board: Board, sign: char) -> bool {
    let opponent_sign = if sign == 'X' { 'O' } else { 'X' };
    match winning_move(board, sign) {
        Some(coordinate) => {
            let attempted_grid = board.add_value(coordinate, opponent_sign);
            if let Some(_) = winning_move(attempted_grid.unwrap(), sign) {
                return true;
            } else {
                return false;
            }
        }
        None => return false,
    }
}

fn get_empty_squares(_board: Board, _sign: char) -> Vec<(usize, usize)> {
    // Return the coordinates for all the empty squares
    vec![(0, 0); 3]
}

fn winning_move(board: Board, sign: char) -> Option<(usize, usize)> {
    let winning_coordinate_functions = [
        get_winning_row_coordinate,
        get_winning_column_coordinate,
        get_winning_diagonal_coordinate,
    ];

    for func in winning_coordinate_functions.iter() {
        if let Some(winning_coordinate) = func(board, sign) {
            return Some(winning_coordinate);
        };
    }

    None
}

fn get_winning_row_coordinate(board: Board, sign: char) -> Option<(usize, usize)> {
    for (y, row) in board.grid.iter().enumerate() {
        let empty_value = row.clone().iter().position(|s| *s == '-');

        if empty_value.is_some() && row.iter().filter(|&v| *v == sign).count() == 2 {
            return Some((empty_value.unwrap(), y));
        }
    }
    None
}

fn get_winning_column_coordinate(board: Board, sign: char) -> Option<(usize, usize)> {
    let transposed = board.transpose();

    match get_winning_row_coordinate(transposed, sign) {
        Some(coordinate) => Some((coordinate.1, coordinate.0)),
        None => None,
    }
}

fn get_winning_diagonal_coordinate(board: Board, sign: char) -> Option<(usize, usize)> {
    let mut top_bottom_diagonal: Vec<char> = Vec::new();
    let mut bottom_top_diagonal: Vec<char> = Vec::new();
    for x in 0..3 {
        top_bottom_diagonal.push(board.grid[x][x]);
        bottom_top_diagonal.push(board.grid[x][2 - x]);
    }

    let empty_value = top_bottom_diagonal.clone().iter().position(|s| *s == '-');

    if empty_value.is_some() && top_bottom_diagonal.iter().filter(|&v| *v == sign).count() == 2 {
        return Some((empty_value.unwrap(), empty_value.unwrap()));
    }

    let empty_value = bottom_top_diagonal.clone().iter().position(|s| *s == '-');

    if empty_value.is_some() && bottom_top_diagonal.iter().filter(|&v| *v == sign).count() == 2 {
        return Some((2 - empty_value.unwrap(), empty_value.unwrap()));
    }

    None
}

#[cfg(test)]
mod board_analysis_tests {
    use super::*;

    #[test]
    fn empty_board_is_not_two_winning_moves() {
        let grid = [['-'; 3]; 3];
        assert_eq!(two_winning_moves(Board { grid }, 'O'), false);
    }

    #[test]
    fn simple_fork_is_two_winning_moves() {
        let grid = [['O', '-', '-'], ['X', 'O', '-'], ['O', '-', '-']];
        assert_eq!(two_winning_moves(Board { grid }, 'O'), true);
    }

    #[test]
    #[ignore]
    fn one_winning_move_is_not_two_winning_moves() {
        let grid = [['O', '-', '-'], ['O', '-', '-'], ['-', '-', '-']];
        assert_eq!(two_winning_moves(Board { grid }, 'O'), false);
    }
}

#[cfg(test)]
mod robot_player_tests {
    use super::{Board, Player, RobotPlayer};

    #[test]
    fn creates_winning_row_if_available() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [['-', '-', 'X'], ['O', '-', 'O'], ['-', 'X', '-']];
        let coordinate = player.get_coordinate(Board { grid }).unwrap();
        assert_eq!(coordinate, (1, 1));
    }

    #[test]
    fn creates_winning_column_if_available() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [['O', 'X', 'X'], ['O', '-', '-'], ['-', '-', '-']];
        let coordinate = player.get_coordinate(Board { grid }).unwrap();
        assert_eq!(coordinate, (0, 2));
    }

    #[test]
    fn creates_winning_diagonal_if_available() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [['O', 'X', 'X'], ['-', 'O', '-'], ['-', '-', '-']];
        let coordinate = player.get_coordinate(Board { grid }).unwrap();
        assert_eq!(coordinate, (2, 2));
    }

    #[test]
    fn blocks_row_if_no_winning_move() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [['-', 'X', 'X'], ['-', 'O', '-'], ['O', '-', '-']];
        let coordinate = player.get_coordinate(Board { grid }).unwrap();
        assert_eq!(coordinate, (0, 0));
    }

    #[test]
    fn blocks_column_if_no_winning_move() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [['-', '-', 'X'], ['-', 'O', 'X'], ['O', '-', '-']];
        let coordinate = player.get_coordinate(Board { grid }).unwrap();
        assert_eq!(coordinate, (2, 2));
    }

    #[test]
    fn blocks_diagonal_if_no_winning_move() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [['-', 'O', 'X'], ['-', 'X', 'O'], ['-', '-', '-']];
        let coordinate = player.get_coordinate(Board { grid }).unwrap();
        assert_eq!(coordinate, (0, 2));
    }

    #[test]
    #[ignore]
    fn creates_fork_with_middle_if_no_win_or_block() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [['X', '-', '-'], ['-', '-', 'O'], ['O', 'X', '-']];
        let coordinate = player.get_coordinate(Board { grid }).unwrap();
        assert_eq!(coordinate, (1, 1));
    }

    #[test]
    #[ignore]
    fn creates_fork_with_side_if_no_win_or_block() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [['-', 'O', '-'], ['-', 'X', 'O'], ['-', 'X', '-']];
        let coordinate = player.get_coordinate(Board { grid }).unwrap();
        assert_eq!(coordinate, (2, 0));
    }

    #[test]
    #[ignore]
    fn blocks_fork_if_no_win_or_block() {
        let player = RobotPlayer { sign: 'X' };
        let grid = [['-', 'O', '-'], ['-', 'X', 'O'], ['-', 'X', '-']];
        let coordinate = player.get_coordinate(Board { grid }).unwrap();
        assert_eq!(coordinate, (2, 0));
    }

    #[test]
    #[ignore]
    fn prevents_fork_opportunity_if_no_win_or_block() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [['X', '-', '-'], ['-', 'O', '-'], ['-', '-', 'X']];
        let coordinate = player.get_coordinate(Board { grid }).unwrap();
        let good_choices: Vec<(usize, usize)> = vec![(0, 1), (1, 0), (1, 2), (2, 1)];
        assert_eq!(good_choices.iter().any(|x| *x == coordinate), true);
    }

    #[test]
    #[ignore]
    fn take_center_when_possible() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [['X', '-', '-'], ['-', '-', '-'], ['-', '-', '-']];
        let coordinate = player.get_coordinate(Board { grid }).unwrap();
        assert_eq!(coordinate, (1, 1));
    }

    #[test]
    #[ignore]
    fn takes_corner_when_possible() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [['-', '-', '-'], ['-', 'X', '-'], ['-', '-', '-']];
        let coordinate = player.get_coordinate(Board { grid }).unwrap();
        let good_choices: Vec<(usize, usize)> = vec![(0, 0), (2, 0), (2, 2), (0, 2)];
        assert_eq!(good_choices.iter().any(|x| *x == coordinate), true);
    }
}
