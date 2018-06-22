use std::io::{self, BufRead};
use board::Board;

pub fn get_coordinate_from_user(_sign: char, _board: Board) -> Result<(usize, usize), String> {
    println!("Enter a coordinate in the format x,y:");
    let input = read_line();
    parse_user_input(input)
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
