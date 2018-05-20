use std::io::{self, BufRead};

fn main() {
    let coordinate: Coordinate = get_coordinate_from_user();
    println!("{} {}", coordinate.x, coordinate.y);
    print_board();
}

fn get_coordinate_from_user() -> Coordinate {
    println!("Enter a coordinate in the format x,y:");
    let input = read_line();
    let vec = input.split(',').collect::<Vec<&str>>();
    let x = vec[0].trim().parse::<u8>().unwrap();
    let y = vec[1].trim().parse::<u8>().unwrap(); 
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

struct Coordinate {
    x: u8,
    y: u8
}