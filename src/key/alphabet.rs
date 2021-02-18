use key::{Key, SetKey, StatefulKey, KeyFrom};

use crate::lang::Language;
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

impl KeyFrom<&String> for Alphabet {
    fn from(language: &Language, string: &String) -> Alphabet {
        let alphabet: Vec<i16> = language.string_to_vec(&string);
        let my_value = util::fill_alphabet_from_start(&alphabet, 26);
        let my_inverse = util::invert(&my_value);
        Alphabet {
            value: my_value,
            inverse: my_inverse
        }
    }
}
impl KeyFrom<&Vec<i16>> for Alphabet {
    fn from(_language: &Language, vec: &Vec<i16>) -> Alphabet {
        Alphabet {
            value: vec.clone(),
            inverse: util::invert(vec)
        }
    }
}

impl SetKey<&String> for Alphabet {
    fn set_key(&mut self, language: &Language, string: &String) {
        let alphabet: Vec<i16> = language.string_to_vec(&string);
        self.value = util::fill_alphabet_from_start(&alphabet, 26);
        self.update_inverse();
    }
}
impl SetKey<&Vec<i16>> for Alphabet {
    fn set_key(&mut self, _language: &Language, vec: &Vec<i16>) {
        self.value = vec.clone();
        self.update_inverse();
    }
}

impl Key for Alphabet {
    fn to_string(&self, language: &Language) -> String {
        language.vec_to_string(&self.value)
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