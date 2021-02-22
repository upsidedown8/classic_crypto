use crate::key::keyword::Keyword;
use crate::key::vigenere_square::classic_vig_square::ClassicVigSquare;
use crate::lang::Language;
use crate::{
    cipher::{Asymmetric, Keyed},
    key::{
        vigenere_square::VigSquare,
        {Key, StatefulKey},
    },
};

pub struct ClassicVigenere {
    square: ClassicVigSquare,
    pub keyword: Keyword,
}

impl ClassicVigenere {}

impl Asymmetric for ClassicVigenere {
    fn encrypt(&self, language: &Language, msg: &str) -> String {
        let mut count = 0;
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let new_cp = self.square.encrypt(
                        language.get_cp(&c),
                        self.keyword.at(count % self.keyword.len()),
                    );
                    count += 1;
                    language.update_cp(&c, new_cp)
                } else {
                    c
                }
            })
            .collect()
    }
    fn decrypt(&self, language: &Language, msg: &str) -> String {
        let mut count = 0;
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let new_cp = self.square.decrypt(
                        self.keyword.at(count % self.keyword.len()),
                        language.get_cp(&c),
                    );
                    count += 1;
                    language.update_cp(&c, new_cp)
                } else {
                    c
                }
            })
            .collect()
    }
}

impl Keyed for ClassicVigenere {
    fn new(language: &Language) -> ClassicVigenere {
        ClassicVigenere {
            square: ClassicVigSquare::new(language),
            keyword: Keyword::new(language),
        }
    }
    fn reset(&mut self, language: &Language) {
        self.keyword.reset(language);
    }
    fn randomize(&mut self, language: &Language, rng: &mut impl rand::Rng) {
        self.keyword.randomize(language, rng);
    }
    fn to_string(&self, language: &Language) -> String {
        format!("Keyword:{}", self.keyword.to_string(language))
    }
}
