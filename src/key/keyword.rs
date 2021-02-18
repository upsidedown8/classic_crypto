use key::{Key, SetKey, StatefulKey, KeyFrom};

use crate::lang::Language;
use crate::util;
use crate::key::key;

pub struct Keyword {
    value: Vec<i16>
}

impl Keyword {
    pub fn len(&self) -> usize {
        self.value.len()
    }
}

impl KeyFrom<&String> for Keyword {
    fn from(language: &Language, string: &String) -> Keyword {
        Keyword {
            value: language.string_to_vec(&string)
        }
    }
}

impl SetKey<&String> for Keyword {
    fn set_key(&mut self, language: &Language, string: &String) {
        self.value = language.string_to_vec(&string);
    }
}
impl SetKey<&Vec<i16>> for Keyword {
    fn set_key(&mut self, _language: &Language, key: &Vec<i16>) {
        self.value = key.clone();
    }
}

impl Key for Keyword {
    fn to_string(&self, language: &Language) -> String {
        language.vec_to_string(&self.value)
    }
    fn new() -> Keyword {
        Keyword {
            value: vec![0]
        }
    }
}

impl StatefulKey for Keyword {
    fn reset(&mut self) {
        self.value = vec![0];
    }
    fn randomize(&mut self, rnd: &mut impl rand::Rng) {
        let length = rnd.gen_range(3..12);
        self.value.resize(length, 0);
        util::fill_random_array(&mut self.value, rnd, 26);
    }
}