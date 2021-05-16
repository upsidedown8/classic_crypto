use crate::{
    cipher::{Asymmetric, Keyed, Solve},
    key::{IdentityKey, IoKey, Key, Keyword, StatefulKey},
    lang::Language,
};

pub struct BlockTransposition {
    pub keyword: Keyword,
}

impl BlockTransposition {
    fn encrypt_indexes(len: usize, key_order: Vec<usize>) -> Vec<usize> {
        let mut result = vec![0; len];
        let column_length = len / key_order.len();
        for col in 0..key_order.len() {
            for row in 0..column_length {
                result[row * key_order.len() + key_order[col]] = row * key_order.len() + col;
            }
        }

        result
    }
    fn decrypt_indexes(len: usize, key_order: Vec<usize>) -> Vec<usize> {
        let mut result = vec![0; len];
        let column_length = len / key_order.len();
        for col in 0..key_order.len() {
            for row in 0..column_length {
                result[row * key_order.len() + col] = row * key_order.len() + key_order[col];
            }
        }

        result
    }
}

impl Asymmetric for BlockTransposition {
    fn encrypt(&self, language: &mut Language, msg: &str) -> String {
        let plaintext = language.string_to_vec(msg);
        let key_order = self.keyword.find_order();
        let mut iter = BlockTransposition::encrypt_indexes(plaintext.len(), key_order).into_iter();
        msg.chars()
            .filter_map(|ch| {
                if language.is_letter(&ch) {
                    Some(language.update_cp(&ch, plaintext[iter.next()?]))
                } else {
                    Some(ch)
                }
            })
            .collect()
    }
    fn decrypt(&self, language: &mut Language, msg: &str) -> String {
        let ciphertext = language.string_to_vec(msg);
        let key_order = self.keyword.find_order();
        let mut iter = BlockTransposition::decrypt_indexes(ciphertext.len(), key_order).into_iter();
        msg.chars()
            .filter_map(|ch| {
                if language.is_letter(&ch) {
                    Some(language.update_cp(&ch, ciphertext[iter.next()?]))
                } else {
                    Some(ch)
                }
            })
            .collect()
    }
}

impl Keyed for BlockTransposition {
    fn new(language: &mut Language) -> BlockTransposition {
        let mut result = BlockTransposition {
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

impl Solve for BlockTransposition {
    fn solve(&mut self, language: &mut Language, msg: &str) {
        let ciphertext = language.string_to_vec(msg);
        let key = super::transposition_solve(
            &ciphertext,
            language,
            BlockTransposition::decrypt_indexes,
            |row, col, key_len, _| row * key_len + col,
        );
        self.keyword.set(language, key.as_slice()).unwrap();
    }
}
