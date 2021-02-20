use crate::{cipher::{Asymmetric, Keyed}, key::{Key, StatefulKey}};
use crate::lang::Language;
use crate::key::alphabet::Alphabet;

pub struct SimpleSubstitution {
    pub alphabet: Alphabet
}

impl Asymmetric for SimpleSubstitution {
    fn encrypt(&self, language: &Language, msg: &String) -> String {
        msg
            .chars()
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
    fn decrypt(&self, language: &Language, msg: &String) -> String {
        msg
            .chars()
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
    fn new(language: &Language) -> SimpleSubstitution {
        SimpleSubstitution {
            alphabet: Alphabet::new(language)
        }
    }
    fn reset(&mut self, language: &Language) {
        self.alphabet.reset(language);
    }
    fn randomize(&mut self, language: &Language, rng: &mut impl rand::Rng) {
        self.alphabet.randomize(language, rng);
    }
    fn to_string(&self, language: &Language) -> String {
        format!("alphabet:{}", self.alphabet.to_string(language))
    }
}