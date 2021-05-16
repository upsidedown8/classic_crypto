use crate::{
    cipher::{Asymmetric, Keyed, Solve},
    key::{ClassicVigSquare, IdentityKey, IoKey, Key, Keyword, StatefulKey, VigSquare},
    lang::Language,
};

pub struct Autokey {
    square: ClassicVigSquare,
    pub keyword: Keyword,
}

impl Asymmetric for Autokey {
    fn encrypt(&self, language: &mut Language, msg: &str) -> String {
        let mut count = 0;
        let mut pt_vec = vec![0; self.keyword.len()];
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let idx = count % self.keyword.len();
                    let cp = language.get_cp(&c);
                    let new_cp = self.square.encrypt(
                        cp,
                        if count < self.keyword.len() {
                            self.keyword.at(count)
                        } else {
                            pt_vec[idx]
                        },
                    );
                    pt_vec[idx] = cp;
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
        let mut pt_vec = vec![0; self.keyword.len()];
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let idx = count % self.keyword.len();
                    pt_vec[idx] = self.square.decrypt(
                        if count < self.keyword.len() {
                            self.keyword.at(count)
                        } else {
                            pt_vec[idx]
                        },
                        language.get_cp(&c),
                    );
                    count += 1;
                    language.update_cp(&c, pt_vec[idx])
                } else {
                    c
                }
            })
            .collect()
    }
}

impl Keyed for Autokey {
    fn new(language: &mut Language) -> Autokey {
        let mut result = Autokey {
            square: ClassicVigSquare::identity(language),
            keyword: Keyword::identity(language),
        };

        result.keyword.key_info_mut().set(
            "Keyword",
            "A string",
            "kw"
        );

        result
    }
    fn reset(&mut self, language: &mut Language) {
        self.keyword.reset(language);
    }
    fn randomize(&mut self, language: &mut Language) {
        self.keyword.randomize(language);
    }
    fn keys(&self) -> Vec<&dyn IoKey> {
        vec![&self.keyword]
    }
    fn keys_mut(&mut self) -> Vec<&mut dyn IoKey> {
        vec![&mut self.keyword]
    }
}

impl Solve for Autokey {
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
                    |key, idx, key_len, plaintext| {
                        if idx < key_len {
                            key[idx]
                        } else {
                            plaintext[idx % key_len]
                        }
                    },
                )
                .as_slice(),
            )
            .unwrap();
    }
}
