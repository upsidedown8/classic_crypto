use std::usize;

use key::{Key, StatefulKey};
use crate::{key::key, util};
use super::vig_square::{VigSquare, vig_square_to_string};

pub struct KeyedVigSquare {
    square: Vec<Vec<i16>>,
    inverse: Vec<Vec<i16>>
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

    fn encrypt(&self, x: i16, y: i16) -> i16 {
        self.square[x as usize][y as usize]
    }
    fn decrypt(&self, x: i16, y: i16) -> i16 {
        self.inverse[x as usize][y as usize]
    }
}

impl Key for KeyedVigSquare {
    fn to_string(&self) -> String {
        vig_square_to_string(&self.square, 26, 26)
    }
    fn new() -> KeyedVigSquare {
        let my_square = vec![vec![0; 26]; 26];
        let mut vig_square = KeyedVigSquare {
            square: my_square.clone(),
            inverse: my_square
        };
        vig_square.init_squares();
        vig_square
    }
}

impl StatefulKey for KeyedVigSquare {
    fn reset(&mut self) {
        self.init_squares();
    }
    fn randomize(&mut self, rnd: &mut impl rand::Rng) {
        let mut alphabet = vec![0; 26];
        util::fill_consecutive_vec(&mut alphabet, 0, 26);
        util::shuffle(&mut alphabet, rnd);
        for row in 0..26 {
            let mut idx: usize = 0;
            for col in row..26 {
                self.inverse[row][alphabet[col] as usize] = idx as i16;
                self.square[row][idx] = alphabet[col] as i16;
                idx += 1;
            }
            for col in 0..row {
                self.inverse[row][alphabet[col] as usize] = idx as i16;
                self.square[row][idx] = alphabet[col] as i16;
                idx += 1;
            }
        }
    }
}
