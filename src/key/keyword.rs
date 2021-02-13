use key::{Key, SetKey};

use crate::convert;
use crate::util;
use crate::key::key;

pub struct Keyword {
    value: Vec<u16>
}

impl Keyword {
    pub fn len(&self) -> usize {
        self.value.len()
    }
}

impl From<&str> for Keyword {
    fn from(string: &str) -> Keyword {
        Keyword::from(&String::from(string))
    }
}
impl From<&String> for Keyword {
    fn from(string: &String) -> Keyword {
        Keyword {
            value: convert::from_string(&string)
        }
    }
}

impl SetKey<&String> for Keyword {
    fn set(&mut self, string: &String) {
        self.value = convert::from_string(&string);
    }
}
impl SetKey<&Vec<u16>> for Keyword {
    fn set(&mut self, key: &Vec<u16>) {
        self.value = key.clone();
    }
}

impl Key for Keyword {
    fn to_string(&self) -> String {
        convert::to_string(&self.value)
    }

    fn reset(&mut self) {
        self.value = vec![0];
    }
    fn randomize(&mut self, rnd: &mut impl rand::Rng) {
        let length = rnd.gen_range(3..12);
        self.value.resize(length, 0);
        util::fill_random_array(&mut self.value, rnd, 26);
    }

    fn new() -> Keyword {
        Keyword {
            value: vec![0]
        }
    }
}