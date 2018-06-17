use board::Board;

pub trait Player: PartialEq + Copy {
    fn get_sign(&self) -> char;

    fn get_coordinate(&self, board: Board) -> Result<(usize, usize), String>;
}

#[derive(Clone, Copy)]
pub struct Player2 {
    pub sign: char,
    pub get_coordinate: fn(Player2, Board) -> Result<(usize, usize), String>,
}

impl PartialEq for Player2 {
    fn eq(&self, other: &Player2) -> bool {
        self.sign == other.sign
    }
}
