mod board;
mod players;
mod robot_player;
mod user_input;
mod win_condition;

use board::Board;
use players::Player;
use robot_player::get_robot_coordinate;
use user_input::get_coordinate_from_user;
use win_condition::is_winning_board;

fn main() {
    let player_one = Player {
        sign: 'O',
        get_coordinate: get_coordinate_from_user,
    };

    let player_two = Player {
        sign: 'X',
        get_coordinate: get_robot_coordinate,
    };

    println!("You are player '{}'", player_one.sign);
    let game = Game::new((player_one, player_two));

    game.play();
}

struct Game {
    board: Board,
    players: (Player, Player),
    current_player: Player,
    is_over: bool,
}

impl Game {
    fn new(players: (Player, Player)) -> Game {
        let board = Board::new();
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
            is_over: is_winning_board(self.board) || is_full_board(self.board),
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
            println!("{}", self.get_final_message());
        } else {
            match (self.current_player.get_coordinate)(self.current_player.sign, self.board) {
                Ok(coordinate) => self.place_choice(coordinate),
                Err(e) => self.print_error_and_play(e),
            }
        }
    }

    fn print_error_and_play(&self, error: String) {
        println!("{}", error);
        println!("{}", self.board);
        self.play();
    }

    fn place_choice(&self, coordinate: (usize, usize)) {
        match self.board.add_value(coordinate, self.current_player.sign) {
            Ok(b) => {
                let new_game = Game { board: b, ..*self };
                let new_game = new_game.next_turn();
                println!(
                    "\nBoard after player {}'s turn:\n",
                    self.current_player.sign
                );
                println!("{}", new_game.board);
                new_game.play();
            }
            Err(e) => self.print_error_and_play(e),
        }
    }

    fn get_final_message(&self) -> String {
        if is_winning_board(self.board) {
            let winner = if self.current_player.sign == 'O' {
                'X'
            } else {
                'O'
            };
            format!("Player {} won!", winner)
        } else {
            "It's a tie!".to_string()
        }
    }
}

fn is_full_board(board: Board) -> bool {
    !board
        .grid
        .iter()
        .flat_map(|r| r.iter())
        .collect::<Vec<&char>>()
        .contains(&&'-')
}

#[cfg(test)]
mod tests {
    use super::*;
    use board::Board;

    fn dummy_get_coordinate(_sign: char, _board: Board) -> Result<(usize, usize), String> {
        Err("Not implemented".to_string())
    }

    #[test]
    fn full_board_is_full() {
        let board = Board {
            grid: [['X'; 3], ['X'; 3], ['X'; 3]],
        };
        assert!(is_full_board(board));
    }

    #[test]
    fn empty_board_is_not_full() {
        let board = Board {
            grid: [['-'; 3], ['-'; 3], ['-'; 3]],
        };
        assert!(!is_full_board(board));
    }

    #[test]
    fn slightly_populated_board_is_not_full() {
        let board = Board {
            grid: [['-', 'X', 'O'], ['-', '-', '-'], ['O', '-', 'X']],
        };
        assert!(!is_full_board(board));
    }

    #[test]
    fn add_value_in_empty_field_adds_value() {
        let sign = 'X';
        let board = Board::new();
        let player = Player {
            sign,
            get_coordinate: dummy_get_coordinate,
        };
        let result_board = board.add_value((0, 0), player.sign).unwrap();
        assert_eq!(result_board.grid[0][0], sign);
    }

    #[test]
    fn add_value_with_custom_sign_uses_sign() {
        let sign = 'A';
        let board = Board::new();
        let player = Player {
            sign,
            get_coordinate: dummy_get_coordinate,
        };
        let result_board = board.add_value((0, 0), player.sign).unwrap();
        assert_eq!(result_board.grid[0][0], sign);
    }

    #[test]
    fn add_value_outside_of_bounds_is_invalid() {
        let board = Board::new();

        let player = Player {
            sign: 'X',
            get_coordinate: dummy_get_coordinate,
        };

        let result = board.add_value((3, 3), player.sign);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn add_value_to_existing_field_is_invalid() {
        let board = Board {
            grid: [['X'; 3], ['-'; 3], ['-'; 3]],
        };

        let player = Player {
            sign: 'X',
            get_coordinate: dummy_get_coordinate,
        };

        let result = board.add_value((0, 0), player.sign);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn play_game_with_game_over_does_not_panic() {
        let players = (
            Player {
                sign: 'O',
                get_coordinate: dummy_get_coordinate,
            },
            Player {
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
            Player {
                sign: 'O',
                get_coordinate: dummy_get_coordinate,
            },
            Player {
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
