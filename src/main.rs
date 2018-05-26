mod user_input;
mod players;
mod players2;

use std::fmt;
use players::{Player};
use players2::{HumanPlayer, Player2};

// TODO: Implement an always-winning bot with https://en.wikipedia.org/wiki/Tic-tac-toe#Strategy

fn main() {
    let player_one = HumanPlayer { sign: 'O' };
    let player_two = HumanPlayer { sign: 'X' };    
    
    let game = new_game2((player_one, player_two));
    
        play_game2(game);
    // play_game(game, &user_input::get_coordinate_from_user);
}

fn play_game2<T: Player2>(game: Game2<T>) {
    if game.is_over {
        println!("Game over");
    }
    else {
        let coordinate = game.current_player.get_coordinate(); 
        match add_value_to_board2(game.board, coordinate, game.current_player) {
            Ok(b) => {
                let new_game = Game2 {
                    board: b,
                    players: game.players,
                    is_over: game.is_over,
                    current_player: game.current_player
                };
                let new_game2 = next_turn(new_game);
                println!("{:?}", new_game2.board);
                play_game2(new_game2);
            },
            Err(e) => {
                println!("{}", e);
                println!("{:?}", game.board);
            }
        }
    }    
}

fn play_game(game: Game, get_user_input: &Fn() -> ((usize, usize))) {
    if game.is_over {
        println!("Game over");
    }
    else {
        let coordinate = get_user_input(); 
        match add_value_to_board(game.board, coordinate, game.current_player) {
            Ok(b) => {
                let new_game = Game {
                    board: b,
                    is_over: game.is_over,
                    current_player: game.current_player
                }.next_turn();
                println!("{}", new_game.board);
                play_game(new_game, get_user_input);
            },
            Err(e) => {
                println!("{}", e);
                println!("{}", game.board);
            }
        }
    }
}

struct Game2<T: Player2> {
    board: Board2,
    players: (T, T),
    current_player: T,
    is_over: bool
}

fn new_game2<T: Player2>(players: (T, T)) -> Game2<T> {
    let board = Board2::new();
    let current_player = players.0;
    let players = players;
    let is_over = false;
    Game2 { board, current_player, players, is_over }
}

fn next_turn<T: Player2>(game: Game2<T>) -> Game2<T> {
    Game2 {
        board: game.board,
        is_over: true,
        players: game.players,
        current_player: if game.current_player == game.players.0 {
            game.players.1
        } else {
            game.players.0
        }
    }
}

struct Game {
    board: Board,
    current_player: Player,
    is_over: bool 
}

impl Game {
    fn new() -> Game {
        let board = Board::new();
        let current_player = Player::One;
        let is_over = false;
        Game { board, current_player, is_over }
    }

    fn next_turn(&self) -> Game {
        Game {
            board: self.board,
            is_over: is_winning_board(self.board),
            current_player: players::switch_player(self.current_player)
        }
    }
}

fn is_winning_board(board: Board) -> bool {
    is_column_win(board) || is_row_win(board) || is_diagonal_win(board)
}

fn is_diagonal_win(board: Board) -> bool {
    let grid = board.grid;

    let mut right_diagonal: Vec<Player> = Vec::new();
    let mut left_diagonal: Vec<Player> = Vec::new();
    for x in 0..3 {
        right_diagonal.push(grid[x][x]);
        left_diagonal.push(grid[x][2-x]);
    }

    // TODO: fix this mess
    let mut user_one_right = right_diagonal.clone().into_iter();
    let mut user_two_right = right_diagonal.clone().into_iter();
    let mut user_one_left = left_diagonal.clone().into_iter();
    let mut user_two_left = left_diagonal.clone().into_iter();    

    user_one_right.all(|x| x == Player::One) || 
    user_two_right.all(|x| x == Player::Two) ||
    user_one_left.all(|x| x == Player::One) ||
    user_two_left.all(|x| x == Player::Two)
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

        let win = iter_row_one.all(|x| *x == Player::One) ||
                  iter_row_two.all(|x| *x == Player::Two);
        if win {
            return true;      
        } 
    }
    false
}

fn add_value_to_board(board: Board, coordinate: (usize, usize), player: Player) -> Result<Board, String> {
    match board.grid[coordinate.0][coordinate.1] {
        Player::One | Player::Two => Err("The field is already taken".to_string()),
        Player::Empty => {
            let mut new_board = board.clone();
            new_board.grid[coordinate.0][coordinate.1] = player;
            Ok(new_board)
        }
    }
}

fn add_value_to_board2<T: Player2>(board: Board2, coordinate: (usize, usize), player: T) -> Result<Board2, String> {
    if board.grid[coordinate.0][coordinate.1] != '-' {
        return Err("The field is already taken".to_string());
    }

    let mut new_board = board.clone();
    new_board.grid[coordinate.0][coordinate.1] = player.get_sign();
    Ok(new_board)
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut values: Vec<String> = Vec::new();

        for (i, _) in self.grid.iter().enumerate() {
            for (j, _) in self.grid[i].iter().enumerate() {
                values.push(players::player_to_sign(self.grid[j][i]))
            }
        }

        write!(f,
"          0     1     2
             |     |
    0     {}  |  {}  |  {}
        _____|_____|_____
             |     |
    1     {}  |  {}  |  {}
        _____|_____|_____
             |     |
    2     {}  |  {}  |  {}
             |     |     ",  
        values[0], 
        values[1], 
        values[2], 
        values[3], 
        values[4], 
        values[5], 
        values[6],
        values[7], 
        values[8])
    }
}

#[derive(Debug, Clone, Copy)]
struct Board2 {
    grid: [[char; 3]; 3]
}

impl Board2 {
    fn new() -> Board2 {
        Board2 {
            grid: [['-'; 3]; 3],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Board {
    grid: [[Player; 3]; 3],
}

impl Board {
    fn new() -> Board {
        Board {
            grid: [[Player::Empty; 3]; 3],
        }
    }
}

#[cfg(test)]
mod game_flow_tests {
    use super::*;

    #[test]
    fn play_game_with_game_over_does_not_panic() {
        let mut game = Game::new();
        game.is_over = true;

        play_game(game, &|| (0, 0))
    }

    #[test]
    fn next_turn_switches_current_player() {
        let game = Game::new();
        let initial_player = game.current_player;

        let game_after_turn = game.next_turn();
        let players_are_same = initial_player == game_after_turn.current_player;
        assert_eq!(players_are_same, false)
    }
}

#[cfg(test)]
mod win_condition_tests {
    use super::*;

    #[test]
    fn empty_board_is_no_win() {
        let board = Board {
            grid: [[Player::Empty; 3]; 3]
        };
        assert_eq!(is_winning_board(board), false);
    }

    #[test]
    fn complete_row_is_win() {
        let board = Board {
            grid: [
                [Player::One; 3], 
                [Player::Empty; 3], 
                [Player::Empty; 3]
            ]
        };
        assert_eq!(is_winning_board(board), true);
    }

    #[test]
    fn diagonal_is_win() {
        let board = Board {
            grid: [
                [Player::One, Player::Empty, Player::Empty],
                [Player::Empty, Player::One, Player::Empty],
                [Player::Empty, Player::Empty, Player::One],                
            ]
        };
        assert_eq!(is_winning_board(board), true);
    }

    #[test]
    fn complete_column_is_win() {
        let board = Board {
            grid: [
                [Player::One, Player::Empty, Player::Empty],
                [Player::One, Player::Empty, Player::Empty],
                [Player::One, Player::Empty, Player::Empty],
            ]
        };
        assert_eq!(is_winning_board(board), true);
    }

    #[test]
    fn combined_row_is_no_win() {
        let board = Board {
            grid: [
                [Player::One, Player::Two, Player::Two],
                [Player::Empty, Player::Empty, Player::Empty],
                [Player::Empty, Player::Empty, Player::Empty],
            ]
        };
        assert_eq!(is_winning_board(board), false);
    }

    #[test]
    fn combined_column_is_no_win() {
        let board = Board {
            grid: [
                [Player::One, Player::Empty, Player::Empty],
                [Player::Two, Player::Empty, Player::Empty],
                [Player::One, Player::Empty, Player::Empty],
            ]
        };
        assert_eq!(is_winning_board(board), false);
    }

    #[test]
    fn combined_diagonal_is_no_win() {
        let board = Board {
            grid: [
                [Player::One, Player::Empty, Player::Empty],
                [Player::Empty, Player::Two, Player::Empty],
                [Player::Empty, Player::Empty, Player::One],
            ]
        };
        assert_eq!(is_winning_board(board), false);
    }
}
