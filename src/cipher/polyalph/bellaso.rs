use crate::{
    cipher::{Keyed, Solve, Symmetric},
    key::{BellasoSquare, IdentityKey, IoKey, Key, Keyword, VigSquare},
    lang::Language,
};

pub struct Bellaso {
    square: BellasoSquare,
    pub keyword: Keyword,
}

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
        let mut result = Bellaso {
            square: BellasoSquare::identity(language),
            keyword: Keyword::identity(language),
        };

        result.keyword.info_mut().set("Keyword", "kw");

        result
    }
    fn keys(&self) -> Vec<&dyn IoKey> {
        vec![&self.keyword]
    }
    fn keys_mut(&mut self) -> Vec<&mut dyn IoKey> {
        vec![&mut self.keyword]
    }
}

impl Solve for Bellaso {
    fn solve(&mut self, language: &mut Language, msg: &str) {
        let ciphertext = language.string_to_vec(msg);
        self.keyword
            .set(
                language,
                crate::cipher::polyalph::vig_solve(
                    &ciphertext,
                    1,
                    language,
                    |cp, shift| self.square.encrypt(shift, cp),
                    |key, idx, key_len, _| key[idx % key_len],
                )
                .as_slice(),
            )
            .unwrap();
    }
}
