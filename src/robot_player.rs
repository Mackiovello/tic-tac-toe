use board::Board;

type Coordinate = (usize, usize);

pub fn get_robot_coordinate(sign: char, board: Board) -> Result<Coordinate, String> {
    let possible_moves: Vec<Box<Fn(Board, char) -> Option<Coordinate>>> = vec!(
        Box::new(winning_move),
        Box::new(block_winning_move),
        Box::new(fork_move),
        Box::new(block_fork_move),
        Box::new(block_fork_opportunity_move),
        Box::new(take_center_move),
        Box::new(take_corner_move),
        Box::new(take_side_move)
    );

    for possible_move in possible_moves {
        if let Some(chosen_coordinate) = possible_move(board, sign) {
            return Ok(chosen_coordinate);
        }
    }

    Err("No choice found".to_string())
}

fn block_winning_move(board: Board, sign: char) -> Option<Coordinate> {
    winning_move(board, get_opponent_sign(sign))
}

fn block_fork_move(board: Board, sign: char) -> Option<Coordinate> {
    fork_move(board, get_opponent_sign(sign))
}

fn take_center_move(board: Board, _sign: char) -> Option<Coordinate> {
    if board.grid[1][1] == '-' {
        Some((1, 1))
    } else {
        None
    }
}

fn take_corner_move(board: Board, sign: char) -> Option<Coordinate> {
    let empty_squares: Vec<Coordinate> = get_empty_squares(board, sign);

    for square in empty_squares {
        if square == (0, 0) || square == (0, 2) || square == (2, 0) || square == (2, 2) {
            return Some(square);
        }
    }

    None
}

fn take_side_move(board: Board, sign: char) -> Option<Coordinate> {
    let empty_squares: Vec<Coordinate> = get_empty_squares(board, sign);

    for square in empty_squares {
        if square == (1, 0) || square == (0, 1) || square == (2, 1) || square == (1, 2) {
            return Some(square);
        }
    }

    None
}

fn block_fork_opportunity_move(board: Board, sign: char) -> Option<Coordinate> {
    let empty_squares: Vec<Coordinate> = get_empty_squares(board, sign);
    let opponent_sign = get_opponent_sign(sign);
    let mut opportunities: Vec<Coordinate> = Vec::new();

    for square in empty_squares {
        let attempted_board = board.add_value(square, opponent_sign);
        if fork_move(attempted_board.unwrap(), opponent_sign).is_some() {
            opportunities.push(square);
        }
    }

    if opportunities.is_empty() {
        None
    } else {
        for opportunity in &opportunities {
            if *opportunity == (1, 1) {
                return Some(*opportunity);
            }
        }

        for opportunity in &opportunities {
            if *opportunity == (0, 0) || *opportunity == (0, 2) || *opportunity == (2, 0)
                || *opportunity == (2, 2)
            {
                return Some(*opportunity);
            }
        }

        Some(opportunities[0])
    }
}

fn get_opponent_sign(sign: char) -> char {
    if sign == 'X' { 'O' } else { 'X' }
}

fn fork_move(board: Board, sign: char) -> Option<Coordinate> {
    let empty_squares: Vec<Coordinate> = get_empty_squares(board, sign);

    for square in empty_squares {
        let attempted_grid = board.add_value(square, sign).unwrap();
        if two_winning_moves(attempted_grid, sign) {
            return Some(square);
        }
    }

    None
}

fn two_winning_moves(board: Board, sign: char) -> bool {
    match winning_move(board, sign) {
        Some(coordinate) => {
            let attempted_grid = board.add_value(coordinate, get_opponent_sign(sign));

            winning_move(attempted_grid.unwrap(), sign).is_some()
        }
        None => false,
    }
}

fn get_empty_squares(board: Board, _sign: char) -> Vec<Coordinate> {
    let mut empty_squares: Vec<Coordinate> = Vec::new();

    for (y, row) in board.grid.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if *value == '-' {
                empty_squares.push((x, y))
            }
        }
    }
    empty_squares
}

fn winning_move(board: Board, sign: char) -> Option<Coordinate> {
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

fn get_winning_row_coordinate(board: Board, sign: char) -> Option<Coordinate> {
    for (y, row) in board.grid.iter().enumerate() {
        let empty_value = row.clone().iter().position(|s| *s == '-');

        if empty_value.is_some() && row.iter().filter(|&v| *v == sign).count() == 2 {
            return Some((empty_value.unwrap(), y));
        }
    }
    None
}

fn get_winning_column_coordinate(board: Board, sign: char) -> Option<Coordinate> {
    let transposed = board.transpose();

    match get_winning_row_coordinate(transposed, sign) {
        Some(coordinate) => Some((coordinate.1, coordinate.0)),
        None => None,
    }
}

fn get_winning_diagonal_coordinate(board: Board, sign: char) -> Option<Coordinate> {
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
mod tests {
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
    fn one_winning_move_is_not_two_winning_moves() {
        let grid = [['O', '-', '-'], ['O', '-', '-'], ['-', '-', '-']];
        assert_eq!(two_winning_moves(Board { grid }, 'O'), false);
    }

    #[test]
    fn creates_winning_row_if_available() {
        let grid = [['-', '-', 'X'], ['O', '-', 'O'], ['-', 'X', '-']];
        let coordinate = get_robot_coordinate('O', Board { grid }).unwrap();
        assert_eq!(coordinate, (1, 1));
    }

    #[test]
    fn creates_winning_column_if_available() {
        let grid = [['O', 'X', 'X'], ['O', '-', '-'], ['-', '-', '-']];
        let coordinate = get_robot_coordinate('O', Board { grid }).unwrap();
        assert_eq!(coordinate, (0, 2));
    }

    #[test]
    fn creates_winning_diagonal_if_available() {
        let grid = [['O', 'X', 'X'], ['-', 'O', '-'], ['-', '-', '-']];
        let coordinate = get_robot_coordinate('O', Board { grid }).unwrap();
        assert_eq!(coordinate, (2, 2));
    }

    #[test]
    fn blocks_row_if_no_winning_move() {
        let grid = [['-', 'X', 'X'], ['-', 'O', '-'], ['O', '-', '-']];
        let coordinate = get_robot_coordinate('O', Board { grid }).unwrap();
        assert_eq!(coordinate, (0, 0));
    }

    #[test]
    fn blocks_column_if_no_winning_move() {
        let grid = [['-', '-', 'X'], ['-', 'O', 'X'], ['O', '-', '-']];
        let coordinate = get_robot_coordinate('O', Board { grid }).unwrap();
        assert_eq!(coordinate, (2, 2));
    }

    #[test]
    fn blocks_diagonal_if_no_winning_move() {
        let grid = [['-', 'O', 'X'], ['-', 'X', 'O'], ['-', '-', '-']];
        let coordinate = get_robot_coordinate('O', Board { grid }).unwrap();
        assert_eq!(coordinate, (0, 2));
    }

    #[test]
    fn creates_fork_with_middle_if_no_win_or_block() {
        let grid = [['-', '-', '-'], ['X', '-', 'X'], ['O', 'X', 'O']];
        let coordinate = get_robot_coordinate('O', Board { grid }).unwrap();
        assert_eq!(coordinate, (1, 1));
    }

    #[test]
    fn creates_fork_with_side_if_no_win_or_block() {
        let grid = [['-', 'O', '-'], ['-', 'X', 'O'], ['-', 'X', '-']];
        let coordinate = get_robot_coordinate('O', Board { grid }).unwrap();
        assert_eq!(coordinate, (2, 0));
    }

    #[test]
    fn blocks_fork_if_no_win_or_block() {
        let grid = [['-', 'O', '-'], ['-', '-', 'O'], ['-', 'X', '-']];
        let coordinate = get_robot_coordinate('X', Board { grid }).unwrap();
        assert_eq!(coordinate, (2, 0));
    }

    #[test]
    fn prevents_fork_opportunity_if_no_win_or_block() {
        let grid = [['X', '-', '-'], ['-', 'O', '-'], ['-', '-', 'X']];
        let coordinate = get_robot_coordinate('O', Board { grid }).unwrap();
        let good_choices: Vec<Coordinate> = vec![(0, 2), (2, 0)];
        assert!(good_choices.iter().any(|x| *x == coordinate));
    }

    #[test]
    fn take_center_when_possible() {
        let grid = [['X', '-', '-'], ['-', '-', '-'], ['-', '-', '-']];
        let coordinate = get_robot_coordinate('O', Board { grid }).unwrap();
        assert_eq!(coordinate, (1, 1));
    }

    #[test]
    fn takes_corner_when_possible() {
        let grid = [['-', '-', '-'], ['-', 'X', '-'], ['-', '-', '-']];
        let coordinate = get_robot_coordinate('O', Board { grid }).unwrap();
        let good_choices: Vec<Coordinate> = vec![(0, 0), (2, 0), (2, 2), (0, 2)];
        assert_eq!(good_choices.iter().any(|x| *x == coordinate), true);
    }

    #[test]
    fn takes_corner_when_only_corner_and_side_left() {
        let grid = [['O', 'O', 'X'], ['X', 'X', 'O'], ['O', '-', '-']];

        let coordinate = get_robot_coordinate('O', Board { grid }).unwrap();
        assert_eq!(coordinate, (2, 2));
    }

    #[test]
    fn takes_side_if_only_sides_left() {
        let grid = [['O', 'O', 'X'], ['X', 'X', 'O'], ['O', '-', 'X']];

        let coordinate = get_robot_coordinate('O', Board { grid }).unwrap();
        assert_eq!(coordinate, (1, 2));
    }
}
