use std::fmt;

pub fn player_to_sign(player: Player) -> String {
    match player {
        Player::One => "O",
        Player::Two => "X",
        Player::Empty => "-"
    }.to_string()
}

pub fn switch_player(current_player: Player) -> Player {
    match current_player {
        Player::One => Player::Two,
        Player::Two => Player::One,
        _ => panic!("That option is not possible")
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    One,
    Two,
    Empty,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let to_print = match self {
            &Player::One => "player one",
            &Player::Two => "player two",
            &Player::Empty => panic!("Can't print for no player")
        };
        write!(f, "{}", to_print)
    }
}
