use std::io::{self, BufRead};

fn main() {
    let board = Board::new();



    let mut new_board = add_value_to_board(board, Coordinate { x: 0, y: 0 }, CoordinateValue::UserOne);
    new_board = add_value_to_board(new_board, Coordinate { x: 0, y: 2 }, CoordinateValue::UserOne);
    print_board(new_board);
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

    let v1 = coordinate_value_to_sign(board.grid[0][0]);
    let v2 = coordinate_value_to_sign(board.grid[1][0]);
    let v3 = coordinate_value_to_sign(board.grid[2][0]);
    let v4 = coordinate_value_to_sign(board.grid[0][1]);
    let v5 = coordinate_value_to_sign(board.grid[1][1]);
    let v6 = coordinate_value_to_sign(board.grid[2][1]);
    let v7 = coordinate_value_to_sign(board.grid[0][2]);
    let v8 = coordinate_value_to_sign(board.grid[1][2]);
    let v9 = coordinate_value_to_sign(board.grid[2][2]);

    println!(
        "             0     1     2
                |     |
          0  {}  |  {}  |  {}
           _____|_____|_____
                |     |
          1  {}  |  {}  |  {}
           _____|_____|_____
                |     |
          2  {}  |  {}  |  {}
                |     |     "
    , v1, v2, v3, v4, v5, v6, v7, v8, v9);
}

fn coordinate_value_to_sign(value: CoordinateValue) -> &'static str {
    match value {
        CoordinateValue::UserOne => "O",
        CoordinateValue::UserTwo => "X",
        CoordinateValue::Empty => "-"
    }
}

fn read_line() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line
}

#[derive(Debug)]
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

#[derive(Clone, Copy, Debug)]
enum CoordinateValue {
    UserOne,
    UserTwo,
    Empty,
}

struct Coordinate {
    x: usize,
    y: usize,
}
