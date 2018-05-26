mod players;

extern crate itertools;

use std::fmt;
use itertools::Itertools;
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
        is_over: is_winning_board(game.board),
        players: game.players,
        current_player: if game.current_player == game.players.0 {
            game.players.1
        } else {
            game.players.0
        }
    }
}

fn is_winning_board(board: Board) -> bool {
    is_column_win(board) || is_row_win(board) || is_diagonal_win(board)    
}

fn is_diagonal_win(board: Board) -> bool {
    let grid = board.grid;

    let mut right_diagonal: Vec<char> = Vec::new();
    let mut left_diagonal: Vec<char> = Vec::new();
    for x in 0..3 {
        right_diagonal.push(grid[x][x]);
        left_diagonal.push(grid[x][2-x]);
    }

    unique_non_empty_row(right_diagonal) || unique_non_empty_row(left_diagonal)
}

fn unique_non_empty_row(row: Vec<char>) -> bool {
    row.clone()
        .into_iter()
        .unique()
        .count() == 1 && 
        row[0] != '-'
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
    board.grid
        .iter()
        .filter(|x| unique_non_empty_row(x.to_vec()))
        .count() > 0
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
        if self.grid[coordinate.0][coordinate.1] != '-' {
            return Err("The field is already taken".to_string());
        }

        let mut new_board = self.clone();
        new_board.grid[coordinate.0][coordinate.1] = player.get_sign();
        Ok(new_board)
    }
}

#[cfg(test)]
mod game_flow_tests {
    use super::*;

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
}

#[cfg(test)]
mod win_condition_tests {
    use super::*;

    #[test]
    fn empty_board_is_no_win() {
        let board = Board {
            grid: [['-'; 3]; 3]
        };
        assert_eq!(is_winning_board(board), false);
    }

    #[test]
    fn complete_row_is_win() {
        let board = Board {
            grid: [
                ['O'; 3], 
                ['-'; 3], 
                ['-'; 3]
            ]
        };
        assert_eq!(is_winning_board(board), true);
    }

    #[test]
    fn diagonal_is_win() {
        let board = Board {
            grid: [
                ['O', '-', '-'],
                ['-', 'O', '-'],
                ['-', '-', 'O'],                
            ]
        };
        assert_eq!(is_winning_board(board), true);
    }

    #[test]
    fn complete_column_is_win() {
        let board = Board {
            grid: [
                ['O', '-', '-'],
                ['O', '-', '-'],
                ['O', '-', '-'],
            ]
        };
        assert_eq!(is_winning_board(board), true);
    }

    #[test]
    fn combined_row_is_no_win() {
        let board = Board {
            grid: [
                ['O', 'X', 'X'],
                ['-', '-', '-'],
                ['-', '-', '-'],
            ]
        };
        assert_eq!(is_winning_board(board), false);
    }

    #[test]
    fn combined_column_is_no_win() {
        let board = Board {
            grid: [
                ['O', '-', '-'],
                ['X', '-', '-'],
                ['O', '-', '-'],
            ]
        };
        assert_eq!(is_winning_board(board), false);
    }

    #[test]
    fn combined_diagonal_is_no_win() {
        let board = Board {
            grid: [
                ['O', '-', '-'],
                ['-', 'X', '-'],
                ['-', '-', 'O'],
            ]
        };
        assert_eq!(is_winning_board(board), false);
    }
}
