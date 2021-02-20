use crate::{key::{Key, StatefulKey}, lang::Language, util};
use rand::Rng;

pub struct Plugboard {
    substitution: Vec<i16>
}

impl Plugboard {
    pub fn add_plug(&mut self, letter1: i16, letter2: i16) {
        assert!(self.is_valid_plug(letter1, letter2));

        self.substitution[letter1 as usize] = letter2;
        self.substitution[letter2 as usize] = letter1;
    }
    pub fn del_plug(&mut self, letter1: i16, letter2: i16) {
        assert!(self.is_existing_plug(letter1, letter2));

        self.substitution[letter1 as usize] = letter1;
        self.substitution[letter2 as usize] = letter2;
    }

    pub fn input(&self, letter: i16) -> i16 {
        self.substitution[letter as usize]
    }

    pub fn is_valid_plug(&self, letter1: i16, letter2: i16) -> bool {
        let idx1 = letter1 as usize;
        let idx2 = letter2 as usize;

        self.substitution[idx1] == letter1 &&
        self.substitution[idx2] == letter2 &&
        idx1 != idx2
    }
    pub fn is_existing_plug(&self, letter1: i16, letter2: i16) -> bool {
        let idx1 = letter1 as usize;
        let idx2 = letter2 as usize;

        self.substitution[idx1] == letter2 &&
        self.substitution[idx2] == letter1 &&
        idx1 != idx2
    }
    pub fn is_letter_used(&self, letter: i16) -> bool {
        self.substitution[letter as usize] != letter
    }
}

impl Key for Plugboard {
    fn to_string(&self, language: &Language) -> String {
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
        data.trim().to_string()
    }

    fn new(_language: &Language) -> Plugboard {
        let mut result = Plugboard {
            substitution: vec![0; 26]
        };
        result.reset(_language);
        result
    }
}

impl StatefulKey for Plugboard {
    fn reset(&mut self, _language: &Language) {
        util::fill_consecutive_vec(&mut self.substitution, 0, 26);
    }
    fn randomize(&mut self, _language: &Language, rng: &mut impl Rng) {
        self.reset(_language);
        let mut values = vec![0; 26];
        util::fill_consecutive_vec(&mut values, 0, 26);
        util::shuffle(&mut values, rng);
        let num_plugs = rng.gen_range(5..13);
        for i in 0..num_plugs {
            self.add_plug(values[i*2], values[i*2 + 1]);
        }
    }
}