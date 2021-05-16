use super::VigSquare;
use crate::{
    key::IdentityKey,
    lang::Language,
};

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

    #[inline(always)]
    fn encrypt(&self, x: i16, y: i16) -> i16 {
        self.square[x as usize][y as usize]
    }
    #[inline(always)]
    fn decrypt(&self, x: i16, y: i16) -> i16 {
        self.square[x as usize][y as usize]
    }
}

impl IdentityKey for PortaSquare {
    fn identity(_language: &mut Language) -> Self {
        let mut vig_square = PortaSquare {
            square: vec![vec![0; 26]; 26],
        };
        vig_square.init_squares();
        vig_square
    }
}
