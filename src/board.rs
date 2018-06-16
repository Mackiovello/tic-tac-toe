// use players;

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

    pub fn transpose(&self) -> Board {
        Board {
            grid: transpose_grid(self.grid),
        }
    }

    pub fn add_value(&self, coordinate: (usize, usize), sign: char) -> Result<Board, String> {
        if coordinate.0 > 2 || coordinate.1 > 2 {
            return Err("The field is out of bounds".to_string());
        }

        if self.grid[coordinate.1][coordinate.0] != '-' {
            return Err("The field is already taken".to_string());
        }

        let mut new_board = self.clone();
        new_board.grid[coordinate.1][coordinate.0] = sign;
        Ok(new_board)
    }
}

fn transpose_grid(grid: [[char; 3]; 3]) -> [[char; 3]; 3] {
    let mut transposed = [['-'; 3]; 3];

    for i in 0..3 {
        for j in 0..3 {
            transposed[i][j] = grid[j][i];
        }
    }
    transposed
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
            "  0     1     2
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
