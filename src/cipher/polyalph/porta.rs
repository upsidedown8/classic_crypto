use crate::{
    cipher::{Keyed, Solve, Symmetric},
    key::{
        Keyword, PortaSquare, SetKey, VigSquare, {Key, StatefulKey},
    },
    lang::Language,
};

pub struct Porta {
    square: PortaSquare,
    pub keyword: Keyword,
}

impl Porta {}

impl Symmetric for Porta {
    fn run(&self, language: &mut Language, msg: &str) -> String {
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
    fn new(language: &mut Language) -> Porta {
        Porta {
            square: PortaSquare::new(language),
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

impl Solve for Porta {
    fn solve(&mut self, language: &mut Language, msg: &str) {
        let ciphertext = language.string_to_vec(msg);
        self.keyword.set_key(
            language,
            &crate::cipher::polyalph::vig_solve(
                &ciphertext,
                2,
                language,
                |cp, shift| self.square.encrypt(shift / 2, cp),
                |key, idx, key_len, _| key[idx % key_len],
            ),
        )
    }
}
