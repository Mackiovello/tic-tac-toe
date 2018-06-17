mod players;
mod win_condition;
mod board;
mod human_player;
mod robot_player;

use win_condition::is_winning_grid;
use players::{Player, Player2};
use human_player::{get_coordinate, HumanPlayer};
use robot_player::{get_robot_coordinate, RobotPlayer};

fn main() {
    let player_one = Player2 {
        sign: 'O',
        get_coordinate: get_coordinate,
    };

    let player_two = Player2 {
        sign: 'X',
        get_coordinate: get_robot_coordinate,
    };

    let game = Game::new((player_one, player_two));

    game.play();
}

struct Game {
    board: board::Board,
    players: (Player2, Player2),
    current_player: Player2,
    is_over: bool,
}

impl Game {
    fn new(players: (Player2, Player2)) -> Game {
        let board = board::Board::new();
        let current_player = players.0;
        let players = players;
        let is_over = false;
        Game {
            board,
            current_player,
            players,
            is_over,
        }
    }

    fn next_turn(&self) -> Self {
        Game {
            board: self.board,
            is_over: is_winning_grid(self.board),
            players: self.players,
            current_player: if self.current_player == self.players.0 {
                self.players.1
            } else {
                self.players.0
            },
        }
    }

    fn play(&self) {
        if self.is_over {
            println!("Game over");
        } else {
            match (self.current_player.get_coordinate)(self.current_player, self.board) {
                Ok(coordinate) => {
                    match self.board.add_value(coordinate, self.current_player.sign) {
                        Ok(b) => {
                            let new_game = Game {
                                board: b,
                                players: self.players,
                                is_over: self.is_over,
                                current_player: self.current_player,
                            };
                            let new_game = new_game.next_turn();
                            println!("{}", new_game.board);
                            new_game.play();
                        }
                        Err(e) => {
                            println!("{}", e);
                            println!("{}", self.board);
                            self.play();
                        }
                    }
                }
                Err(e) => {
                    println!("{}", e);
                    println!("{}", self.board);
                    self.play();
                }
            }
        }
    }
}

#[cfg(test)]
mod game_tests {
    use super::*;
    use board::Board;

    fn dummy_get_coordinate(_player: Player2, _board: Board) -> Result<(usize, usize), String> {
        Err("Not implemented".to_string())
    }

    #[test]
    fn add_value_in_empty_field_adds_value() {
        let sign = 'X';
        let board = board::Board::new();
        let player = Player2 {
            sign,
            get_coordinate: dummy_get_coordinate,
        };
        let result_board = board.add_value((0, 0), player.sign).unwrap();
        assert_eq!(result_board.grid[0][0], sign);
    }

    #[test]
    fn add_value_with_custom_sign_uses_sign() {
        let sign = 'A';
        let board = board::Board::new();
        let player = Player2 {
            sign,
            get_coordinate: dummy_get_coordinate,
        };
        let result_board = board.add_value((0, 0), player.sign).unwrap();
        assert_eq!(result_board.grid[0][0], sign);
    }

    #[test]
    fn add_value_outside_of_bounds_is_invalid() {
        let board = board::Board::new();

        let player = Player2 {
            sign: 'X',
            get_coordinate: dummy_get_coordinate,
        };

        let result = board.add_value((3, 3), player.sign);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn add_value_to_existing_field_is_invalid() {
        let board = board::Board {
            grid: [['X'; 3], ['-'; 3], ['-'; 3]],
        };

        let player = Player2 {
            sign: 'X',
            get_coordinate: dummy_get_coordinate,
        };

        let result = board.add_value((0, 0), player.sign);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn play_game_with_game_over_does_not_panic() {
        let players = (
            Player2 {
                sign: 'O',
                get_coordinate: dummy_get_coordinate,
            },
            Player2 {
                sign: 'X',
                get_coordinate: dummy_get_coordinate,
            },
        );
        let mut game = Game::new(players);
        game.is_over = true;

        game.play();
    }

    #[test]
    fn next_turn_switches_current_player() {
        let players = (
            Player2 {
                sign: 'O',
                get_coordinate: dummy_get_coordinate,
            },
            Player2 {
                sign: 'X',
                get_coordinate: dummy_get_coordinate,
            },
        );
        let game = Game::new(players);
        let initial_player = game.current_player;

        let game_after_turn = game.next_turn();
        let players_are_same = initial_player == game_after_turn.current_player;
        assert_eq!(players_are_same, false)
    }
}
