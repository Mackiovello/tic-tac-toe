use board::Board;

pub trait Player: PartialEq + Copy {
    fn get_sign(&self) -> char;

    fn get_coordinate(&self, board: Board) -> Result<(usize, usize), String>;
}
