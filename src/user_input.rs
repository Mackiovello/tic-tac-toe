use std::io::{self, BufRead};

pub fn get_coordinate_from_user() -> Coordinate {
    println!("Enter a coordinate in the format x,y:");

    // TODO: Add input validation and split validation
    //       into a separate function

    let input = read_line();
    let vec = input.split(',').collect::<Vec<&str>>();
    let x = vec[0].trim().parse::<usize>().unwrap();
    let y = vec[1].trim().parse::<usize>().unwrap();
    Coordinate { x, y }
}

pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

fn is_valid_coordinate(string: String) -> bool {
    true
}

fn read_line() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line
}

#[cfg(test)]
mod input_validation_tests {
    use super::*;

    #[test]
    fn comma_without_space_is_valid() {
        let result: bool = is_valid_coordinate(String::from("0,0"));
        assert_eq!(result, true);
    }

    #[test]
    fn comma_with_space_is_invalid() {
        let result: bool = is_valid_coordinate(String::from("0, 0"));
        assert_eq!(result, false);
    }

    #[test]
    fn only_space_is_invalid() {
        let result: bool = is_valid_coordinate(String::from("0 0"));
        assert_eq!(result, false);
    }
}
