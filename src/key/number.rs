use rand::Rng;
use std::ops::Range;

use crate::key::{Key, KeyFrom, SetKey, StatefulKey};
use crate::lang::Language;

/// Represents a Number (See Affine Cipher)
///
pub struct Number {
    value: i16,
    range: Range<i16>,
}

impl Number {
    /// Gets the value
    ///
    pub fn get(&self) -> i16 {
        self.value
    }

    /// Sets the range of legal values for the number.
    ///
    /// # Arguments
    ///
    /// * `range` The legal range of numbers
    ///
    pub fn set_range(&mut self, range: Range<i16>) {
        self.range = range;
    }

    /// Is the current value within the legal range?
    ///
    pub fn validate(&self) -> bool {
        self.range.contains(&self.value)
    }
}

impl KeyFrom<i16> for Number {
    fn create_from(language: &mut Language, val: i16) -> Number {
        Number {
            value: val,
            range: 0..language.cp_count(),
        }
    }
}
impl KeyFrom<&String> for Number {
    fn create_from(language: &mut Language, string: &String) -> Number {
        let num: i16 = string.parse().expect("Expected a number");
        Number::create_from(language, num)
    }
}

impl SetKey<i16> for Number {
    fn set_key(&mut self, _language: &mut Language, num: i16) {
        assert!(self.range.contains(&num));

        self.value = num;
    }
}
impl SetKey<&String> for Number {
    fn set_key(&mut self, language: &mut Language, string: &String) {
        let num: i16 = string.parse().expect("Expected a number");
        self.set_key(language, num);
    }
}

impl Key for Number {
    fn new(language: &mut Language) -> Number {
        Number {
            value: 0,
            range: 0..language.cp_count(),
        }
    }
    fn to_string(&self, _language: &mut Language) -> String {
        format!("{}", self.value)
    }
}

impl StatefulKey for Number {
    fn reset(&mut self, _language: &mut Language) {
        self.value = self.range.start;
    }
    fn randomize(&mut self, _language: &mut Language, rng: &mut impl Rng) {
        self.value = rng.gen_range(self.range.clone());
    }
}
