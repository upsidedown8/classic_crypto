use key::Key;
use crate::key::key;
use super::vig_square::{VigSquare, vig_square_to_string};

pub struct BellasoSquare {
    square: Vec<Vec<i16>>
}

impl VigSquare for BellasoSquare {
    fn init_squares(&mut self) {
        for row in 0..13 {
            let mut idx = 0;
            for col in 0..row {
                self.square[row][idx] = (26 + col - row) as i16;
                idx += 1;
            }
            for col in row..13 {
                self.square[row][idx] = (13 + col - row) as i16;
                idx += 1;
            }
            for col in 0..13-row {
                self.square[row][idx] = (col + row) as i16;
                idx += 1;
            }
            for col in 0..row {
                self.square[row][idx] = col as i16;
                idx += 1;
            }
        }
        for row in 0..13 {
            for col in 0..26 {
                self.square[row + 13][25 - col] = self.square[row][col];
            }
        }
    }

    fn encrypt(&self, x: i16, y: i16) -> i16 {
        self.square[x as usize][y as usize]
    }
    fn decrypt(&self, x: i16, y: i16) -> i16 {
        self.square[x as usize][y as usize]
    }
}

impl Key for BellasoSquare {
    fn to_string(&self) -> String {
        vig_square_to_string(&self.square, 26, 26)
    }
    fn new() -> BellasoSquare {
        let mut vig_square = BellasoSquare {
            square: vec![vec![0; 26]; 26]
        };
        vig_square.init_squares();
        vig_square
    }
}
