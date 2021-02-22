use crate::key::{keyword::Keyword, vigenere_square::VigSquare};
use crate::lang::Language;
use crate::{
    cipher::{Keyed, Symmetric},
    key::{
        vigenere_square::porta_square::PortaSquare,
        {Key, StatefulKey},
    },
};

pub struct Porta {
    square: PortaSquare,
    pub keyword: Keyword,
}

impl Porta {}

impl Symmetric for Porta {
    fn run(&self, language: &Language, msg: &str) -> String {
        let mut count = 0;
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let new_cp = self.square.encrypt(
                        self.keyword.at(count % self.keyword.len()) / 2,
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

impl Keyed for Porta {
    fn new(language: &Language) -> Porta {
        Porta {
            square: PortaSquare::new(language),
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
