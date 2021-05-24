use crate::{
    cipher::{Asymmetric, Keyed, Solve},
    key::{ClassicVigSquare, IdentityKey, IoKey, Key, Keyword, VigSquare},
    lang::Language,
};

pub struct ClassicVigenere {
    square: ClassicVigSquare,
    pub keyword: Keyword,
}

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
        let mut result = ClassicVigenere {
            square: ClassicVigSquare::identity(language),
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

impl Solve for ClassicVigenere {
    fn solve(&mut self, language: &mut Language, msg: &str) {
        let ciphertext = language.string_to_vec(msg);
        self.keyword
            .set(
                language,
                crate::cipher::polyalph::vig_solve(
                    &ciphertext,
                    1,
                    language,
                    |cp, shift| self.square.decrypt(shift, cp),
                    |key, idx, key_len, _| key[idx % key_len],
                )
                .as_slice(),
            )
            .unwrap();
    }
}
