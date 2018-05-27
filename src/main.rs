mod players;
mod win_condition;

use win_condition::is_winning_grid;
use std::fmt;
use players::{HumanPlayer, Player};

// TODO: Implement an always-winning bot with https://en.wikipedia.org/wiki/Tic-tac-toe#Strategy

fn main() {
    let player_one = HumanPlayer { sign: 'O' };
    let player_two = HumanPlayer { sign: 'X' };    
    
    let game = Game::new((player_one, player_two));
    
    play_game(game);
}

fn play_game<T: Player>(game: Game<T>) {
    if game.is_over {
        println!("Game over");
    }
    else {
        match game.current_player.get_coordinate() {
            Ok(coordinate) => {
                match game.board.add_value(coordinate, game.current_player) {
                    Ok(b) => {
                        let new_game = Game {
                            board: b,
                            players: game.players,
                            is_over: game.is_over,
                            current_player: game.current_player
                        };
                        let new_game = next_turn(new_game);
                        println!("{}", new_game.board);
                        play_game(new_game);
                    },
                    Err(e) => {
                        println!("{}", e);
                        println!("{}", game.board);
                    }
                }
            }
            Err(e) => {
                println!("{}", e);
                println!("{}", game.board);
            }
        }

    }    
}

struct Game<T> where T: Player {
    board: Board,
    players: (T, T),
    current_player: T,
    is_over: bool
}

impl <T> Game<T> where T: Player {
    fn new(players: (T, T)) -> Game<T> {
        let board = Board::new();
        let current_player = players.0;
        let players = players;
        let is_over = false;
        Game { board, current_player, players, is_over }
    }
}

fn next_turn<T: Player>(game: Game<T>) -> Game<T> {
    Game {
        board: game.board,
        is_over: is_winning_grid(game.board.grid),
        players: game.players,
        current_player: if game.current_player == game.players.0 {
            game.players.1
        } else {
            game.players.0
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let values = self.grid
                        .iter()
                        .flat_map(|a| a.iter())
                        .cloned()
                        .collect::<Vec<char>>();

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
struct Board {
    grid: [[char; 3]; 3]
}

impl Board {
    fn new() -> Board {
        Board {
            grid: [['-'; 3]; 3],
        }
    }

    fn add_value<T: Player>(&self, coordinate: (usize, usize), player: T) -> Result<Board, String> {
        if coordinate.0 > 2 || coordinate.1 > 2 {
            return Err("The field is out of bounds".to_string());
        }
        
        if self.grid[coordinate.0][coordinate.1] != '-' {
            return Err("The field is already taken".to_string());
        }

        let mut new_board = self.clone();
        new_board.grid[coordinate.0][coordinate.1] = player.get_sign();
        Ok(new_board)
    }
}

#[cfg(test)]
mod game_tests {
    use super::*;

    #[test]
    fn add_value_in_empty_field_adds_value() {
        let sign = 'X';
        let board = Board::new();
        let player = TestPlayer { sign };
        let result_board = board.add_value((0, 0), player).unwrap();
        assert_eq!(result_board.grid[0][0], sign);
    }

    #[test]
    fn add_value_with_custom_sign_uses_sign() {
        let sign = 'A';
        let board = Board::new();
        let player = TestPlayer { sign };
        let result_board = board.add_value((0, 0), player).unwrap();
        assert_eq!(result_board.grid[0][0], sign);
    }

    #[test]
    fn add_value_outside_of_bounds_is_invalid() {
        let board = Board::new();
        let player = TestPlayer { sign: 'X' };
        let result = board.add_value((3, 3), player);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn add_value_to_existing_field_is_invalid() {
        let board = Board {
            grid: [
                ['X'; 3],
                ['-'; 3],
                ['-'; 3],
            ]
        };

        let player = TestPlayer { sign: 'X' };
        let result = board.add_value((0, 0), player);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn play_game_with_game_over_does_not_panic() {
        let players = (HumanPlayer { sign: 'O' }, HumanPlayer { sign: 'X' });
        let mut game = Game::new(players);
        game.is_over = true;

        play_game(game)
    }

    #[test]
    fn next_turn_switches_current_player() {
        let players = (HumanPlayer { sign: 'O' }, HumanPlayer { sign: 'X' });        
        let game = Game::new(players);
        let initial_player = game.current_player;

        let game_after_turn = next_turn(game);
        let players_are_same = initial_player == game_after_turn.current_player;
        assert_eq!(players_are_same, false)
    }

    #[derive(PartialEq, Clone, Copy)]
    struct TestPlayer {
        pub sign: char,
    }

    impl Player for TestPlayer {
        fn get_sign(&self) -> char {
            self.sign
        }

        fn get_coordinate(&self) -> Result<(usize, usize), String> {
            Err("Not implemented".to_string())
        }
    }
}
