use crate::{
    key::{Key, StatefulKey},
    lang::Language,
    util,
};
use rand::Rng;

/// Represents an Enigma Plugboard (See Enigma cipher)
///
#[derive(Clone)]
pub struct Plugboard {
    substitution: Vec<i16>,
}

impl Plugboard {
    /// Creates a new plug between two letters, provided that
    /// the plug is valid
    ///
    /// # Arguments
    ///
    /// * `letter1` The first letter of the plug
    /// * `letter2` The second letter of the plug
    ///
    pub fn add_plug(&mut self, letter1: i16, letter2: i16) {
        debug_assert!(self.is_valid_plug(letter1, letter2));

        self.substitution[letter1 as usize] = letter2;
        self.substitution[letter2 as usize] = letter1;
    }

    /// Removes a plug between two letters, provided that the plug exists
    ///
    /// # Arguments
    ///
    /// * `letter1` The first letter of the plug
    /// * `letter2` The second letter of the plug
    ///
    pub fn del_plug(&mut self, letter1: i16, letter2: i16) {
        debug_assert!(self.is_existing_plug(letter1, letter2));

        self.substitution[letter1 as usize] = letter1;
        self.substitution[letter2 as usize] = letter2;
    }

    /// Sends a letter through the plugboard and returns the output
    /// letter. If the letter is not plugged, returns that letter.
    ///
    /// # Arguments
    ///
    /// * `letter` The letter to input to the plug
    ///
    pub fn input(&self, letter: i16) -> i16 {
        self.substitution[letter as usize]
    }

    /// Checks whether creating a plug between letter1 and letter2 is allowed.
    ///
    /// # Arguments
    ///
    /// * `letter1` The first letter of the plug
    /// * `letter2` The second letter of the plug
    ///
    pub fn is_valid_plug(&self, letter1: i16, letter2: i16) -> bool {
        let idx1 = letter1 as usize;
        let idx2 = letter2 as usize;

        self.substitution[idx1] == letter1 && self.substitution[idx2] == letter2 && idx1 != idx2
    }

    /// Checks whether two letters are plugged together
    ///
    /// # Arguments
    ///
    /// * `letter1` The first letter of the plug
    /// * `letter2` The second letter of the plug
    ///
    pub fn is_existing_plug(&self, letter1: i16, letter2: i16) -> bool {
        let idx1 = letter1 as usize;
        let idx2 = letter2 as usize;

        self.substitution[idx1] == letter2 && self.substitution[idx2] == letter1 && idx1 != idx2
    }

    /// Checks whether a letter is used in a plug
    ///
    /// # Arguments
    ///
    /// * `letter` The letter to check
    ///
    pub fn is_letter_used(&self, letter: i16) -> bool {
        self.substitution[letter as usize] != letter
    }
}

impl Key for Plugboard {
    fn to_string(&self, language: &mut Language) -> String {
        let mut tmp = self.substitution.clone();
        let mut data = String::new();
        for i in 0..26 {
            if tmp[i as usize] != i {
                data.push(language.cp_to_upper(i));
                data.push(language.cp_to_upper(tmp[i as usize]));
                data.push(' ');

                // prevent plug from being counted twice
                let t = tmp[i as usize];
                tmp[i as usize] = i;
                tmp[t as usize] = t;
            }
        }
        format!("Plugboard: {}", data.trim())
    }

    fn new(language: &mut Language) -> Plugboard {
        let mut result = Plugboard {
            substitution: vec![0; 26],
        };
        result.reset(language);
        result
    }
}

impl StatefulKey for Plugboard {
    fn reset(&mut self, _language: &mut Language) {
        util::fill_consecutive_vec(&mut self.substitution, 0, 26);
    }
    fn randomize(&mut self, language: &mut Language, rng: &mut impl Rng) {
        self.reset(language);
        let mut values = vec![0; 26];
        util::fill_consecutive_vec(&mut values, 0, 26);
        util::shuffle(&mut values, rng);
        let num_plugs = rng.gen_range(5..13);
        for i in 0..num_plugs {
            self.add_plug(values[i * 2], values[i * 2 + 1]);
        }
    }
}
