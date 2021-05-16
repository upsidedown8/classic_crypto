use super::VigSquare;
use crate::{
    key::IdentityKey,
    lang::Language,
};

/// Represents a Vigenere tableau. (See Classic Vigenere cipher)
///
pub struct ClassicVigSquare {
    square: Vec<Vec<i16>>,
    inverse: Vec<Vec<i16>>,
}

impl VigSquare for ClassicVigSquare {
    fn init_squares(&mut self) {
        for row in 0..26 {
            let mut idx: usize = 0;
            for col in row..26 {
                self.inverse[row][col] = idx as i16;
                self.square[row][idx] = col as i16;
                idx += 1;
            }
            for col in 0..row {
                self.inverse[row][col] = idx as i16;
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
        self.inverse[x as usize][y as usize]
    }
}

impl IdentityKey for ClassicVigSquare {
    fn identity(_language: &mut Language) -> Self {
        let my_square = vec![vec![0; 26]; 26];
        let mut vig_square = ClassicVigSquare {
            square: my_square.clone(),
            inverse: my_square,
        };
        vig_square.init_squares();
        vig_square
    }
}
