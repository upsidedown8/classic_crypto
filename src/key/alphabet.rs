use key::{Key, KeyFrom, SetKey, StatefulKey};

use crate::key;
use crate::lang::Language;
use crate::util;

pub struct Alphabet {
    value: Vec<i16>,
    inverse: Vec<i16>,
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
    fn create_from(language: &Language, string: &String) -> Alphabet {
        let alphabet: Vec<i16> = language.string_to_vec(&string);
        let my_value = util::fill_alphabet_from_start(&alphabet, language.alphabet_len());
        let my_inverse = util::invert(&my_value);
        Alphabet {
            value: my_value,
            inverse: my_inverse,
        }
    }
}
impl KeyFrom<&Vec<i16>> for Alphabet {
    fn create_from(_language: &Language, vec: &Vec<i16>) -> Alphabet {
        Alphabet {
            value: vec.clone(),
            inverse: util::invert(vec),
        }
    }
}

impl SetKey<&String> for Alphabet {
    fn set_key(&mut self, language: &Language, string: &String) {
        let alphabet: Vec<i16> = language.string_to_vec(&string);
        self.value = util::fill_alphabet_from_start(&alphabet, language.alphabet_len() as usize);
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
    fn new(language: &Language) -> Alphabet {
        let mut alphabet = vec![0; language.alphabet_len()];
        util::fill_consecutive_vec(&mut alphabet, 0, language.cp_count());
        Alphabet {
            value: alphabet.clone(),
            inverse: alphabet,
        }
    }
}

impl StatefulKey for Alphabet {
    fn reset(&mut self, language: &Language) {
        self.value = vec![0; language.alphabet_len()];
        self.update_inverse();
    }
    fn randomize(&mut self, _language: &Language, rng: &mut impl rand::Rng) {
        util::shuffle(&mut self.value, rng);
        self.update_inverse();
    }
}
