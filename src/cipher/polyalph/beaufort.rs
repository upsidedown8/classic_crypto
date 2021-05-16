use crate::{
    cipher::{Keyed, Solve, Symmetric},
    key::{ClassicVigSquare, IdentityKey, IoKey, Key, Keyword, VigSquare},
    lang::Language,
};

pub struct Beaufort {
    square: ClassicVigSquare,
    pub keyword: Keyword,
}

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
        let mut result = Beaufort {
            square: ClassicVigSquare::identity(language),
            keyword: Keyword::identity(language),
        };

        result
            .keyword
            .key_info_mut()
            .set("Keyword", "A string", "kw");

        result
    }
    fn keys(&self) -> Vec<&dyn IoKey> {
        vec![&self.keyword]
    }
    fn keys_mut(&mut self) -> Vec<&mut dyn IoKey> {
        vec![&mut self.keyword]
    }
}

impl Solve for Beaufort {
    fn solve(&mut self, language: &mut Language, msg: &str) {
        let ciphertext = language.string_to_vec(msg);
        self.keyword
            .set(
                language,
                crate::cipher::polyalph::vig_solve(
                    &ciphertext,
                    1,
                    language,
                    |cp, shift| self.square.decrypt(cp, shift),
                    |key, idx, key_len, _| key[idx % key_len],
                )
                .as_slice(),
            )
            .unwrap();
    }
}
