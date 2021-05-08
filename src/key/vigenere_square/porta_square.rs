use super::VigSquare;
use crate::{key, lang::Language};
use key::Key;

/// Represents a Porta tableau. (See Porta cipher)
///
pub struct PortaSquare {
    square: Vec<Vec<i16>>,
}

impl VigSquare for PortaSquare {
    fn init_squares(&mut self) {
        for row in 0..13 {
            let mut idx = 0;
            for col in row..13 {
                self.square[row][idx] = (col + 13) as i16;
                idx += 1;
            }
            for col in 0..row {
                self.square[row][idx] = (col + 13) as i16;
                idx += 1;
            }
            for col in 13 - row..13 {
                self.square[row][idx] = col as i16;
                idx += 1;
            }
            for col in 0..13 - row {
                self.square[row][idx] = col as i16;
                idx += 1;
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

impl Key for PortaSquare {
    fn to_string(&self, language: &mut Language) -> String {
        PortaSquare::vig_square_to_string(language, &self.square, 13, 26)
    }
    fn new(_language: &mut Language) -> PortaSquare {
        let mut vig_square = PortaSquare {
            square: vec![vec![0; 26]; 26],
        };
        vig_square.init_squares();
        vig_square
    }
}
