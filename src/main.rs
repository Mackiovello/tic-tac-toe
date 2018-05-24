mod user_input;

fn main() {
    let mut board = Board::new();
    let mut won: bool = false;

    let mut current_user = Users::UserOne;

    while !won {
        let coordinate = user_input::get_coordinate_from_user();
        board = add_value_to_board(board, coordinate, current_user);

        current_user = match current_user {
            Users::UserOne => Users::UserTwo,
            Users::UserTwo => Users::UserOne,
            _ => panic!("That option is optional")
        };

        let (game_over, new_board) = is_winning_board_old(board);
        won = game_over;
        board = new_board;
        print_board(board);
    }
}

// Dummy implementation - should change
fn is_winning_board_old(board: Board) -> (bool, Board) {
    let mut values: Vec<Users> = Vec::new();

    for (i, _) in board.grid.iter().enumerate() {
        for (j, _) in board.grid[i].iter().enumerate() {
            values.push(board.grid[j][i])
        }
    }

    (values.into_iter().filter(|x| *x == Users::UserOne).count() == 3, board)
}

fn is_winning_board(board: Board) -> bool {
    is_column_win(board) || is_row_win(board) || is_diagonal_win(board)
}

fn is_diagonal_win(board: Board) -> bool {
    let grid = board.grid;

    let mut right_diagonal: Vec<Users> = Vec::new();
    let mut left_diagonal: Vec<Users> = Vec::new();
    for x in 0..3 {
        right_diagonal.push(grid[x][x]);
        left_diagonal.push(grid[x][2-x]);
    }

    let mut user_one_right = right_diagonal.clone().into_iter();
    let mut user_two_right = right_diagonal.clone().into_iter();
    let mut user_one_left = left_diagonal.clone().into_iter();
    let mut user_two_left = left_diagonal.clone().into_iter();    

    user_one_right.all(|x| x == Users::UserOne) || 
    user_two_right.all(|x| x == Users::UserTwo) ||
    user_one_left.all(|x| x == Users::UserOne) ||
    user_two_left.all(|x| x == Users::UserTwo)
}

fn is_column_win(board: Board) -> bool {
    let transposed = transpose_board(board);
    is_row_win(transposed)
}

fn transpose_board(board: Board) -> Board {
    let mut transposed = Board::new();

    for i in 0..board.grid.len(){
        for j in 0..board.grid[i].len(){
            transposed.grid[i][j] = board.grid[j][i];
        }
    }

    transposed
}

fn is_row_win(board: Board) -> bool {
    for row in board.grid.iter() {
        let mut iter_row_one = row.into_iter();
        let mut iter_row_two = row.into_iter();

        let win = iter_row_one.all(|x| *x == Users::UserOne) ||
                  iter_row_two.all(|x| *x == Users::UserTwo);
        if win {
            return true;      
        } 
    }
    false
}

fn add_value_to_board(mut board: Board, coordinate: (usize, usize), coordinate_value: Users) -> Board {
    board.grid[coordinate.0][coordinate.1] = coordinate_value;
    board
}

fn print_board(board: Board) {

    let mut values: Vec<String> = Vec::new();

    for (i, _) in board.grid.iter().enumerate() {
        for (j, _) in board.grid[i].iter().enumerate() {
            values.push(user_to_sign(board.grid[j][i]))
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

fn user_to_sign(value: Users) -> String {
    match value {
        Users::UserOne => "O",
        Users::UserTwo => "X",
        Users::Empty => "-"
    }.to_string()
}

#[derive(Debug, Clone, Copy)]
struct Board {
    grid: [[Users; 3]; 3],
}

impl Board {
    fn new() -> Board {
        Board {
            grid: [[Users::Empty; 3]; 3],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Users {
    UserOne,
    UserTwo,
    Empty,
}

#[cfg(test)]
mod win_condition_tests {
    use super::*;

    #[test]
    fn empty_board_is_no_win() {
        let board = Board {
            grid: [[Users::Empty; 3]; 3]
        };
        assert_eq!(is_winning_board(board), false);
    }

    #[test]
    fn complete_row_is_win() {
        let board = Board {
            grid: [
                [Users::UserOne; 3], 
                [Users::Empty; 3], 
                [Users::Empty; 3]
            ]
        };
        assert_eq!(is_winning_board(board), true);
    }

    #[test]
    fn diagonal_is_win() {
        let board = Board {
            grid: [
                [Users::UserOne, Users::Empty, Users::Empty],
                [Users::Empty, Users::UserOne, Users::Empty],
                [Users::Empty, Users::Empty, Users::UserOne],                
            ]
        };
        assert_eq!(is_winning_board(board), true);
    }

    #[test]
    fn complete_column_is_win() {
        let board = Board {
            grid: [
                [Users::UserOne, Users::Empty, Users::Empty],
                [Users::UserOne, Users::Empty, Users::Empty],
                [Users::UserOne, Users::Empty, Users::Empty],
            ]
        };
        assert_eq!(is_winning_board(board), true);
    }

    #[test]
    fn combined_row_is_no_win() {
        let board = Board {
            grid: [
                [Users::UserOne, Users::UserTwo, Users::UserTwo],
                [Users::Empty, Users::Empty, Users::Empty],
                [Users::Empty, Users::Empty, Users::Empty],
            ]
        };
        assert_eq!(is_winning_board(board), false);
    }

    #[test]
    fn combined_column_is_no_win() {
        let board = Board {
            grid: [
                [Users::UserOne, Users::Empty, Users::Empty],
                [Users::UserTwo, Users::Empty, Users::Empty],
                [Users::UserOne, Users::Empty, Users::Empty],
            ]
        };
        assert_eq!(is_winning_board(board), false);
    }

    #[test]
    fn combined_diagonal_is_no_win() {
        let board = Board {
            grid: [
                [Users::UserOne, Users::Empty, Users::Empty],
                [Users::Empty, Users::UserTwo, Users::Empty],
                [Users::Empty, Users::Empty, Users::UserOne],
            ]
        };
        assert_eq!(is_winning_board(board), false);
    }
}