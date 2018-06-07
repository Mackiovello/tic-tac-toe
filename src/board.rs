use players;

use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub grid: [[char; 3]; 3],
}

impl Board {
    pub fn new() -> Board {
        Board {
            grid: [['-'; 3]; 3],
        }
    }

    pub fn add_value<T: players::Player>(
        &self,
        coordinate: (usize, usize),
        player: T,
    ) -> Result<Board, String> {
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

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let values = self.grid
            .iter()
            .flat_map(|a| a.iter())
            .cloned()
            .collect::<Vec<char>>();

        write!(
            f,
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
            values[8]
        )
    }
}
