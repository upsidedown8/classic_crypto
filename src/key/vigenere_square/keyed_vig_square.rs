use super::VigSquare;
use crate::{
    key::{IdentityKey, StatefulKey},
    lang::Language,
    util,
};

/// Represents a Keyed Vigenere tableau. (See Keyed Vigenere cipher)
///
pub struct KeyedVigSquare {
    square: Vec<Vec<i16>>,
    inverse: Vec<Vec<i16>>,
}

impl KeyedVigSquare {
    pub fn set_alphabet(&mut self, alphabet: &[i16]) {
        for row in 0..26 {
            let mut idx: usize = 0;
            for letter in &alphabet[row..26] {
                self.inverse[row][*letter as usize] = idx as i16;
                self.square[row][idx] = *letter;
                idx += 1;
            }
            for letter in &alphabet[0..row] {
                self.inverse[row][*letter as usize] = idx as i16;
                self.square[row][idx] = *letter;
                idx += 1;
            }
        }
    }
}

impl VigSquare for KeyedVigSquare {
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

impl IdentityKey for KeyedVigSquare {
    fn identity(_language: &mut Language) -> Self {
        let my_square = vec![vec![0; 26]; 26];
        let mut vig_square = KeyedVigSquare {
            square: my_square.clone(),
            inverse: my_square,
        };
        vig_square.init_squares();
        vig_square
    }
}

impl StatefulKey for KeyedVigSquare {
    fn reset(&mut self, _language: &mut Language) {
        self.init_squares();
    }
    fn to_string(&self, language: &mut Language) -> String {
        Self::vig_square_to_string(language, &self.square, 26, 26)
    }
    fn randomize(&mut self, _language: &mut Language) {
        let mut alphabet = vec![0; 26];
        util::fill_consecutive_vec(&mut alphabet, 0, 26);
        util::shuffle(&mut alphabet);
        for row in 0..26 {
            let mut idx: usize = 0;
            for letter in &alphabet[row..26] {
                self.inverse[row][*letter as usize] = idx as i16;
                self.square[row][idx] = *letter;
                idx += 1;
            }
            for letter in &alphabet[0..row] {
                self.inverse[row][*letter as usize] = idx as i16;
                self.square[row][idx] = *letter;
                idx += 1;
            }
        }
    }
}
