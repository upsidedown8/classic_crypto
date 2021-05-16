use crate::{
    cipher::{Asymmetric, Keyed},
    key::{Alphabet, IdentityKey, IoKey, KeyedVigSquare, Keyword, StatefulKey, VigSquare},
    lang::Language,
};

pub struct KeyedVigenere {
    pub keyword: Keyword,
    pub alphabet: Alphabet,
}

impl Asymmetric for KeyedVigenere {
    fn encrypt(&self, language: &mut Language, msg: &str) -> String {
        let mut square = KeyedVigSquare::identity(language);
        square.set_alphabet(self.alphabet.as_slice());

        let mut count = 0;
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let new_cp = square.encrypt(
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
        let mut square = KeyedVigSquare::identity(language);
        square.set_alphabet(self.alphabet.as_slice());

        let mut count = 0;
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let new_cp = square.decrypt(
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

impl Keyed for KeyedVigenere {
    fn new(language: &mut Language) -> KeyedVigenere {
        let mut result = KeyedVigenere {
            keyword: Keyword::identity(language),
            alphabet: Alphabet::identity(language),
        };

        result.keyword.key_info_mut().set(
            "Keyword",
            "A string",
            "kw"
        );
        result.alphabet.key_info_mut().set(
            "Alphabet",
            "",
            "alph"
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
        vec![&self.keyword, &self.alphabet]
    }
    fn keys_mut(&mut self) -> Vec<&mut dyn IoKey> {
        vec![&mut self.keyword, &mut self.alphabet]
    }
}
