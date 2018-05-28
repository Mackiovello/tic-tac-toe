use std::io::{self, BufRead};

pub trait Player: PartialEq + Copy {
    fn get_sign(&self) -> char;

    fn get_coordinate(&self, grid: [[char; 3]; 3]) -> Result<(usize, usize), String>;
}

#[derive(PartialEq, Clone, Copy)]
pub struct HumanPlayer {
    pub sign: char,
}

impl Player for HumanPlayer {
    fn get_sign(&self) -> char {
        self.sign
    }

    fn get_coordinate(&self, _grid: [[char; 3]; 3]) -> Result<(usize, usize), String> {
        println!("Enter a coordinate in the format x,y:");
        let input = read_line();
        parse_user_input(input)
    }
}

fn parse_user_input(input: String) -> Result<(usize, usize), String> {
    if input.trim().is_empty() { 
        return Err("The input can't be empty".to_string());
    }

    let vec = input.split(',').collect::<Vec<&str>>();

    if vec.len() > 2 || vec.len() == 1 {
        return Err("You have to pass two values".to_string());
    }

    let x_result = vec[0].trim().parse::<usize>();
    let y_result = vec[1].trim().parse::<usize>();

    if x_result.is_ok() && y_result.is_ok() {
        let x = x_result.unwrap();
        let y = y_result.unwrap();

        if x > 2 || y > 2 {
            return Err("Value can't be larger than 2".to_string());
        }

        return Ok((x, y));
    }

    Err("The value can't be smaller than 0".to_string())
}

fn read_line() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line
}

#[derive(PartialEq, Clone, Copy)]
pub struct RobotPlayer {
    pub sign: char,
}

impl RobotPlayer {
    fn get_opponent_sign(&self) -> char {
        if self.get_sign() == 'X' { 'O' } else { 'X' }
    }
}

impl Player for RobotPlayer {
    fn get_sign(&self) -> char {
        self.sign
    }

    fn get_coordinate(&self, grid: [[char; 3]; 3]) -> Result<(usize, usize), String> {
        if let Some(winning_coordinate) = winning_move(grid, self.sign) {
            return Ok(winning_coordinate);
        }

        if let Some(blocking_coordinate) = winning_move(grid, self.get_opponent_sign()) {
            return Ok(blocking_coordinate);
        }

        Err("No choice found".to_string())
    }
}

fn winning_move(grid: [[char; 3]; 3], sign: char) -> Option<(usize, usize)> {
    let winning_coordinate_functions = [
        get_winning_row_coordinate,
        get_winning_column_coordinate,
        get_winning_diagonal_coordinate
    ];

    for func in winning_coordinate_functions.iter() {
        if let Some(winning_coordinate) = func(grid, sign) {
            return Some(winning_coordinate);
        };
    }

    None
} 

fn get_winning_row_coordinate(grid: [[char; 3]; 3], sign: char) -> Option<(usize, usize)> {
    for (y, row) in grid.iter().enumerate() {
        let empty_value = row.clone().iter().position(|s| *s == '-');

        if empty_value.is_some() && row.iter().filter(|&v| *v == sign).count() == 2 {
            return Some((empty_value.unwrap(), y))
        }
    }
    None
}

fn get_winning_column_coordinate(grid: [[char; 3]; 3], sign: char) -> Option<(usize, usize)> {
    let transposed = transpose_grid(grid);

    match get_winning_row_coordinate(transposed, sign) {
        Some(coordinate) => Some((coordinate.1, coordinate.0)),
        None => None
    }
}

fn get_winning_diagonal_coordinate(grid: [[char; 3]; 3], sign: char) -> Option<(usize, usize)> {
    let mut top_bottom_diagonal: Vec<char> = Vec::new();
    let mut bottom_top_diagonal: Vec<char> = Vec::new();
    for x in 0..3 {
        top_bottom_diagonal.push(grid[x][x]);
        bottom_top_diagonal.push(grid[x][2 - x]);
    }

    let empty_value = top_bottom_diagonal.clone().iter().position(|s| *s == '-');

    if empty_value.is_some() && top_bottom_diagonal.iter().filter(|&v| *v == sign).count() == 2 {
        return Some((empty_value.unwrap(), empty_value.unwrap()))
    }

    let empty_value = bottom_top_diagonal.clone().iter().position(|s| *s == '-');

    if empty_value.is_some() && bottom_top_diagonal.iter().filter(|&v| *v == sign).count() == 2 {
        return Some((2 - empty_value.unwrap(), empty_value.unwrap()))
    }

    None
}

fn transpose_grid(grid: [[char; 3]; 3]) -> [[char; 3]; 3] {
    let mut transposed = [['-'; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            transposed[i][j] = grid[j][i];
        }
    }
    transposed
}

#[cfg(test)]
mod robot_player_tests {
    use super::{RobotPlayer, Player};

    #[test]
    fn creates_winning_row_if_available() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [
            ['-', '-', 'X'],
            ['O', '-', 'O'],
            ['-', 'X', '-']
        ];
        let coordinate = player.get_coordinate(grid).unwrap();
        assert_eq!(coordinate, (1, 1));
    }

    #[test]
    fn creates_winning_column_if_available() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [
            ['O', 'X', 'X'],
            ['O', '-', '-'],
            ['-', '-', '-']
        ];
        let coordinate = player.get_coordinate(grid).unwrap();
        assert_eq!(coordinate, (0, 2));
    }

    #[test]
    fn creates_winning_diagonal_if_available() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [
            ['O', 'X', 'X'],
            ['-', 'O', '-'],
            ['-', '-', '-']
        ];
        let coordinate = player.get_coordinate(grid).unwrap();
        assert_eq!(coordinate, (2, 2)); 
    }

    #[test]
    fn blocks_row_if_no_winning_move() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [
            ['-', 'X', 'X'],
            ['-', 'O', '-'],
            ['O', '-', '-']
        ];
        let coordinate = player.get_coordinate(grid).unwrap();
        assert_eq!(coordinate, (0, 0));
    }

    #[test]
    fn blocks_column_if_no_winning_move() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [
            ['-', '-', 'X'],
            ['-', 'O', 'X'],
            ['O', '-', '-']
        ];
        let coordinate = player.get_coordinate(grid).unwrap();
        assert_eq!(coordinate, (2, 2));
    }

    #[test]
    fn blocks_diagonal_if_no_winning_move() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [
            ['-', 'O', 'X'],
            ['-', 'X', 'O'],
            ['-', '-', '-']
        ];
        let coordinate = player.get_coordinate(grid).unwrap();
        assert_eq!(coordinate, (0, 2));
    }

    #[test]
    fn creates_fork_with_middle_if_no_win_or_block() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [
            ['X', '-', '-'],
            ['-', '-', 'O'],
            ['O', 'X', '-']
        ]; 
        let coordinate = player.get_coordinate(grid).unwrap();
        assert_eq!(coordinate, (1, 1));
    }

    #[test]
    fn creates_fork_with_side_if_no_win_or_block() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [
            ['-', 'O', '-'],
            ['-', 'X', 'O'],
            ['-', 'X', '-']
        ]; 
        let coordinate = player.get_coordinate(grid).unwrap();
        assert_eq!(coordinate, (2, 0));
    }

    #[test]
    fn blocks_fork_if_no_win_or_block() {
        let player = RobotPlayer { sign: 'X' };
        let grid = [
            ['-', 'O', '-'],
            ['-', 'X', 'O'],
            ['-', 'X', '-']
        ]; 
        let coordinate = player.get_coordinate(grid).unwrap();
        assert_eq!(coordinate, (2, 0));
    } 

    #[test]
    fn prevents_fork_opportunity_if_no_win_or_block() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [
            ['X', '-', '-'],
            ['-', 'O', '-'],
            ['-', '-', 'X']
        ]; 
        let coordinate = player.get_coordinate(grid).unwrap();
        let good_choices: Vec<(usize, usize)> = vec![(0, 1), (1, 0), (1, 2), (2, 1)];
        assert_eq!(good_choices.iter().any(|x| *x == coordinate), true);
    }

    #[test]
    fn take_center_when_possible() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [
            ['X', '-', '-'],
            ['-', '-', '-'],
            ['-', '-', '-']
        ]; 
        let coordinate = player.get_coordinate(grid).unwrap();
        assert_eq!(coordinate, (1, 1));
    }

    #[test]
    fn takes_corner_when_possible() {
        let player = RobotPlayer { sign: 'O' };
        let grid = [
            ['-', '-', '-'],
            ['-', 'X', '-'],
            ['-', '-', '-']
        ]; 
        let coordinate = player.get_coordinate(grid).unwrap();
        let good_choices: Vec<(usize, usize)> = vec![(0, 0), (2, 0), (2, 2), (0, 2)];
        assert_eq!(good_choices.iter().any(|x| *x == coordinate), true);
    }
}

#[cfg(test)]
mod input_parsing_tests {
    use super::*;

    #[test]
    fn empty_string_gives_error() {
        let result = parse_user_input(String::from(""));
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn whitespace_gives_error() {
        let result = parse_user_input(String::from(" "));
        assert_eq!(result.is_err(), true);        
    }

    #[test]
    fn comma_with_space_works() {
        let result = parse_user_input(String::from("1, 1"));
        assert_eq!(result.unwrap(), (1, 1));
    }

    #[test]
    fn comma_without_space_works() {
        let result = parse_user_input(String::from("1,1"));
        assert_eq!(result.unwrap(), (1, 1));
    }

    #[test]
    fn too_many_values_gives_error() {
        let result = parse_user_input(String::from("1,1,1"));
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn negative_values_give_error() {
        let result = parse_user_input(String::from("-1,1"));
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn too_large_values_give_error() {
        let result = parse_user_input(String::from("3,5"));
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn only_one_value_give_error() {
        let result = parse_user_input(String::from("1"));
        assert_eq!(result.is_err(), true);
    }
}
