use key::{Key, SetKey, StatefulKey};

use crate::convert;
use crate::util;
use crate::key::key;

pub struct Alphabet {
    value: Vec<i16>,
    inverse: Vec<i16>
}

impl Alphabet {
    fn update_inverse(&mut self) {
        self.inverse = util::invert(&self.value);
    }

    pub fn encrypt(&self, letter: i16) -> i16 {
        self.value[letter as usize]
    }
    pub fn decrypt(&self, letter: i16) -> i16 {
        self.inverse[letter as usize]
    }
}

impl From<&str> for Alphabet {
    fn from(string: &str) -> Alphabet {
        Alphabet::from(&String::from(string))
    }
}
impl From<&String> for Alphabet {
    fn from(string: &String) -> Alphabet {
        let alphabet: Vec<i16> = convert::from_string(&string);
        let my_value = util::fill_alphabet_from_start(&alphabet, 26);
        let my_inverse = util::invert(&my_value);
        Alphabet {
            value: my_value,
            inverse: my_inverse
        }
    }
}
impl From<&Vec<i16>> for Alphabet {
    fn from(vec: &Vec<i16>) -> Alphabet {
        Alphabet {
            value: vec.clone(),
            inverse: util::invert(vec)
        }
    }
}

impl SetKey<&String> for Alphabet {
    fn set(&mut self, string: &String) {
        let alphabet: Vec<i16> = convert::from_string(&string);
        self.value = util::fill_alphabet_from_start(&alphabet, 26);
        self.update_inverse();
    }
}
impl SetKey<&Vec<i16>> for Alphabet {
    fn set(&mut self, vec: &Vec<i16>) {
        self.value = vec.clone();
        self.update_inverse();
    }
}

impl Key for Alphabet {
    fn to_string(&self) -> String {
        convert::to_string(&self.value)
    }
    fn new() -> Alphabet {
        let mut alphabet = vec![0; 26];
        util::fill_consecutive_vec(&mut alphabet, 0, 26);
        Alphabet {
            value: alphabet.clone(),
            inverse: alphabet
        }
    }
}

impl StatefulKey for Alphabet {
    fn reset(&mut self) {
        self.value = vec![0; 26];
        self.update_inverse();
    }
    fn randomize(&mut self, rnd: &mut impl rand::Rng) {
        util::shuffle(&mut self.value, rnd);
        self.update_inverse();
    }
}