use std::io::{self, BufRead};

fn main() {
    let mut board = Board::new();
    let mut won: bool = false;

    let mut current_user = CoordinateValue::UserOne;

    while !won {
        let coordinate = get_coordinate_from_user();
        board = add_value_to_board(board, coordinate, current_user);

        current_user = match current_user {
            CoordinateValue::UserOne => CoordinateValue::UserTwo,
            CoordinateValue::UserTwo => CoordinateValue::UserOne,
            _ => panic!("NOPE")
        };

        let (game_over, new_board) = is_winning_board(board);
        won = game_over;
        board = new_board;
        print_board(board);
    }
}

// Dummy implementation - should change
fn is_winning_board(board: Board) -> (bool, Board) {
    let mut values: Vec<CoordinateValue> = Vec::new();

    for (i, _) in board.grid.iter().enumerate() {
        for (j, _) in board.grid[i].iter().enumerate() {
            values.push(board.grid[j][i])
        }
    }

    (values.into_iter().filter(|x| *x == CoordinateValue::UserOne).count() == 3, board)
}

fn add_value_to_board(mut board: Board, coordinate: Coordinate, coordinate_value: CoordinateValue) -> Board {
    board.grid[coordinate.x][coordinate.y] = coordinate_value;
    board
}

fn get_coordinate_from_user() -> Coordinate {
    println!("Enter a coordinate in the format x,y:");

    // TODO: Add input validation and split validation
    //       into a separate function

    let input = read_line();
    let vec = input.split(',').collect::<Vec<&str>>();
    let x = vec[0].trim().parse::<usize>().unwrap();
    let y = vec[1].trim().parse::<usize>().unwrap();
    Coordinate { x, y }
}

fn print_board(board: Board) {

    let mut values: Vec<String> = Vec::new();

    for (i, _) in board.grid.iter().enumerate() {
        for (j, _) in board.grid[i].iter().enumerate() {
            values.push(coordinate_value_to_sign(board.grid[j][i]))
        }
    }

    println!(
"      0     1     2
         |     |
  0  {}   |  {}  |  {}
    _____|_____|_____
         |     |
  1  {}   |  {}  |  {}
    _____|_____|_____
         |     |
  2  {}   |  {}  |  {}
         |     |     ",  
    values[0], 
    values[1], 
    values[2], 
    values[3], 
    values[4], 
    values[5], 
    values[6],
    values[7], 
    values[8]);
}

fn coordinate_value_to_sign(value: CoordinateValue) -> String {
    match value {
        CoordinateValue::UserOne => "O",
        CoordinateValue::UserTwo => "X",
        CoordinateValue::Empty => "-"
    }.to_string()
}

fn read_line() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line
}

#[derive(Debug, Clone, Copy)]
struct Board {
    grid: [[CoordinateValue; 3]; 3],
}

impl Board {
    fn new() -> Board {
        Board {
            grid: [[CoordinateValue::Empty; 3]; 3],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CoordinateValue {
    UserOne,
    UserTwo,
    Empty,
}

struct Coordinate {
    x: usize,
    y: usize,
}