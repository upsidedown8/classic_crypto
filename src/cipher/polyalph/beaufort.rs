use crate::{
    cipher::{Keyed, Solve, Symmetric},
    key::{
        ClassicVigSquare, Keyword, SetKey, VigSquare, {Key, StatefulKey},
    },
    lang::Language,
};

pub struct Beaufort {
    square: ClassicVigSquare,
    pub keyword: Keyword,
}

impl Beaufort {}

impl Symmetric for Beaufort {
    fn run(&self, language: &mut Language, msg: &str) -> String {
        let mut count = 0;
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let new_cp = self.square.decrypt(
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
}

impl Keyed for Beaufort {
    fn new(language: &mut Language) -> Beaufort {
        Beaufort {
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

impl Solve for Beaufort {
    fn solve(&mut self, language: &mut Language, msg: &str) {
        let ciphertext = language.string_to_vec(msg);
        self.keyword.set_key(
            language,
            &crate::cipher::polyalph::vig_solve(
                &ciphertext,
                1,
                language,
                |cp, shift| self.square.decrypt(cp, shift),
                |key, idx, key_len, _| key[idx % key_len],
            ),
        )
    }
}
