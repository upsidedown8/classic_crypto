use crate::{
    cipher::{Keyed, Solve, Symmetric},
    key::{
        BellasoSquare, Keyword, SetKey, VigSquare, {Key, StatefulKey},
    },
    lang::Language,
};

pub struct Bellaso {
    square: BellasoSquare,
    pub keyword: Keyword,
}

impl Bellaso {}

impl Symmetric for Bellaso {
    fn run(&self, language: &mut Language, msg: &str) -> String {
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
    fn new(language: &mut Language) -> Bellaso {
        Bellaso {
            square: BellasoSquare::new(language),
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

impl Solve for Bellaso {
    fn solve(&mut self, language: &mut Language, msg: &str) {
        let ciphertext = language.string_to_vec(msg);
        self.keyword.set_key(
            language,
            &crate::cipher::polyalph::vig_solve(
                &ciphertext,
                1,
                language,
                |cp, shift| self.square.encrypt(shift, cp),
                false,
            ),
        )
    }
}
