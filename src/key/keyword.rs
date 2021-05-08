use key::{Key, KeyFrom, SetKey, StatefulKey};

use crate::key;
use crate::lang::Language;
use crate::util;

/// Represents a Keyword (See Vigenere ciphers)
///
pub struct Keyword {
    value: Vec<i16>,
}

impl Keyword {
    /// Returns the length of the keyword
    ///
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Is the keyword empty?
    ///
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Get the value of the keyword at `idx` as a code point
    ///
    /// # Arguments
    ///
    /// * `idx` The index to get
    ///
    pub fn at(&self, idx: usize) -> i16 {
        assert!(idx < self.value.len());
        self.value[idx]
    }
}

impl KeyFrom<&String> for Keyword {
    fn create_from(language: &mut Language, string: &String) -> Keyword {
        Keyword {
            value: language.string_to_vec(&string),
        }
    }
}

impl SetKey<&String> for Keyword {
    fn set_key(&mut self, language: &mut Language, string: &String) {
        self.value = language.string_to_vec(&string);
    }
}
impl SetKey<&Vec<i16>> for Keyword {
    fn set_key(&mut self, _language: &mut Language, key: &Vec<i16>) {
        self.value = key.clone();
    }
}

impl Key for Keyword {
    fn to_string(&self, language: &mut Language) -> String {
        language.vec_to_string(&self.value)
    }
    fn new(_language: &mut Language) -> Keyword {
        Keyword { value: vec![0] }
    }
}

impl StatefulKey for Keyword {
    fn reset(&mut self, _language: &mut Language) {
        self.value = vec![0];
    }
    fn randomize(&mut self, language: &mut Language, rng: &mut impl rand::Rng) {
        let length = rng.gen_range(3..12);
        self.value.resize(length, 0);
        util::fill_random_array(&mut self.value, rng, language.cp_count());
    }
}
