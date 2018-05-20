use std::io::{self, BufRead};

fn main() {
    let line = read_line();
    println!("{}", line);
    print_board();
}

fn get_coordinate_from_user() -> Coordinate {
    // println!("Enter a coordinate in the format x, y:");
    // let input = read_line();
    // let vec = input.split_whitespace().collect();
    // Coordinate { x: vec[0], y: vec[1] }
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
    x: i32,
    y: i32
}