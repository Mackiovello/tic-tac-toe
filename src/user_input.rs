use std::io::{self, BufRead};

pub fn get_coordinate_from_user() -> Coordinate {
    println!("Enter a coordinate in the format x,y:");
    let input = read_line();
    parse_user_input(input).unwrap()
}

#[derive(Debug)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Coordinate) -> bool {
        self.x == other.x && self.y == other.y
    }
}

fn parse_user_input(input: String) -> Result<Coordinate, String> {
    let vec = input.split(',').collect::<Vec<&str>>();
    let x = vec[0].trim().parse::<usize>().unwrap();
    let y = vec[1].trim().parse::<usize>().unwrap();
    Ok(Coordinate { x, y })
}

fn read_line() -> String {
    let mut line = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut line).unwrap();
    line
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
        assert_eq!(result.unwrap(), Coordinate { x: 1, y: 1 });
    }

    #[test]
    fn comma_without_space_works() {
        let result = parse_user_input(String::from("1,1"));
        assert_eq!(result.unwrap(), Coordinate { x: 1, y: 1 });
    }
}
