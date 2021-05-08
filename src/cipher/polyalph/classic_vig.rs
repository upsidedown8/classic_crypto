use crate::{
    cipher::{Asymmetric, Keyed, Solve},
    key::{
        ClassicVigSquare, Keyword, SetKey, VigSquare, {Key, StatefulKey},
    },
    lang::Language,
};

pub struct ClassicVigenere {
    square: ClassicVigSquare,
    pub keyword: Keyword,
}

impl ClassicVigenere {}

impl Asymmetric for ClassicVigenere {
    fn encrypt(&self, language: &mut Language, msg: &str) -> String {
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
    fn decrypt(&self, language: &mut Language, msg: &str) -> String {
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
    fn new(language: &mut Language) -> ClassicVigenere {
        ClassicVigenere {
            square: ClassicVigSquare::new(language),
            keyword: Keyword::new(language),
        }
    }
    fn reset(&mut self, language: &mut Language) {
        self.keyword.reset(language);
    }
    fn randomize(&mut self, language: &mut Language, rng: &mut impl rand::Rng) {
        self.keyword.randomize(language, rng);
    }
    fn to_string(&self, language: &mut Language) -> String {
        format!("Keyword:{}", self.keyword.to_string(language))
    }
}

impl Solve for ClassicVigenere {
    fn solve(&mut self, language: &mut Language, msg: &str) {
        let ciphertext = language.string_to_vec(msg);
        self.keyword.set_key(
            language,
            &crate::cipher::polyalph::vig_solve(&ciphertext, 1, language, |cp, shift| {
                self.square.decrypt(shift, cp)
            }, false),
        )
    }
}
