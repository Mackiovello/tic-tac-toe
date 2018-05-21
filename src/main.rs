use std::io::{self, BufRead};

fn main() {
    let board = Board::new();

    let new_board = add_value_to_board(board, Coordinate {x: 0, y: 0}, CoordinateValue::UserOne);
    // println!("{:?}", new_board);

    // let coordinate: Coordinate = get_coordinate_from_user();
    // println!("{} {}", coordinate.x, coordinate.y);
    // print_board();
}

fn add_value_to_board(board: Board, coordinate: Coordinate, coordinate_value: CoordinateValue) -> Board {
    let mut new_board = Board::new();
    println!("{:?}", new_board.grid[coordinate.x][coordinate.y]);
    // new_board.grid[coordinate.x] = coordinate_value;
    new_board
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

fn print_board() {
    println!(
"             0     1     2
                |     |
          0  -  |  -  |  -
           _____|_____|_____
                |     |
          1  -  |  -  |  -
           _____|_____|_____
                |     |
          2  -  |  -  |  -
                |     |     ");
}

fn read_line() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line
}

#[derive(Debug)]
struct Board {
    grid: Vec<Vec<CoordinateValue>>
}

impl Board {
    fn new() -> Board {
        // TODO: This should not be a vector since it can't grow or shrink
        Board { grid: vec![vec![CoordinateValue::Empty; 3]; 3]}
    }
}

#[derive(Clone, Debug)]
enum CoordinateValue {
    UserOne,
    UserTwo,
    Empty
}

struct Coordinate {
    x: usize,
    y: usize
}
