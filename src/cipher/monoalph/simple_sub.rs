use crate::key::Alphabet;
use crate::lang::Language;
use crate::{
    cipher::{Asymmetric, Keyed},
    key::{Key, StatefulKey},
};

pub struct SimpleSubstitution {
    pub alphabet: Alphabet,
}

impl Asymmetric for SimpleSubstitution {
    fn encrypt(&self, language: &mut Language, msg: &str) -> String {
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let mut cp = language.get_cp(&c);
                    cp = self.alphabet.encrypt(cp);
                    language.update_cp(&c, cp)
                } else {
                    c
                }
            })
            .collect()
    }
    fn decrypt(&self, language: &mut Language, msg: &str) -> String {
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let mut cp = language.get_cp(&c);
                    cp = self.alphabet.decrypt(cp);
                    language.update_cp(&c, cp)
                } else {
                    c
                }
            })
            .collect()
    }
}

impl Keyed for SimpleSubstitution {
    fn new(language: &mut Language) -> SimpleSubstitution {
        SimpleSubstitution {
            alphabet: Alphabet::new(language),
        }
    }
    fn reset(&mut self, language: &mut Language) {
        self.alphabet.reset(language);
    }
    fn randomize(&mut self, language: &mut Language, rng: &mut impl rand::Rng) {
        self.alphabet.randomize(language, rng);
    }
    fn to_string(&self, language: &mut Language) -> String {
        format!("alphabet:{}", self.alphabet.to_string(language))
    }
}
