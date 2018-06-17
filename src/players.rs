use board::Board;

#[derive(Clone, Copy)]
pub struct Player {
    pub sign: char,
    pub get_coordinate: fn(char, Board) -> Result<(usize, usize), String>,
}

impl PartialEq for Player {
    fn eq(&self, other: &Player) -> bool {
        self.sign == other.sign
    }
}
