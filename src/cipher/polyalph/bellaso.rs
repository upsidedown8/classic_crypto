use crate::key::{keyword::Keyword, vigenere_square::VigSquare};
use crate::lang::Language;
use crate::{
    cipher::{Keyed, Symmetric},
    key::{
        vigenere_square::bellaso_square::BellasoSquare,
        {Key, StatefulKey},
    },
};

pub struct Bellaso {
    square: BellasoSquare,
    pub keyword: Keyword,
}

impl Bellaso {}

impl Symmetric for Bellaso {
    fn run(&self, language: &Language, msg: &String) -> String {
        let mut count = 0;
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let new_cp = self.square.encrypt(
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

impl Keyed for Bellaso {
    fn new(language: &Language) -> Bellaso {
        Bellaso {
            square: BellasoSquare::new(language),
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
