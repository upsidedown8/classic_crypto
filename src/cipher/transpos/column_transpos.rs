use crate::{
    cipher::{Asymmetric, Keyed, Solve},
    key::{IdentityKey, IoKey, Key, Keyword},
    lang::Language,
};

pub struct ColumnTransposition {
    pub keyword: Keyword,
}

impl ColumnTransposition {
    fn encrypt_indexes(len: usize, key_order: Vec<usize>) -> Vec<usize> {
        let mut result = vec![0; len];
        let column_length = len / key_order.len();
        for col in 0..key_order.len() {
            for row in 0..column_length {
                result[key_order[col] * column_length + row] = row * key_order.len() + col;
            }
        }

        result
    }
    fn decrypt_indexes(len: usize, key_order: Vec<usize>) -> Vec<usize> {
        let mut result = vec![0; len];
        let column_length = len / key_order.len();
        for col in 0..key_order.len() {
            for row in 0..column_length {
                result[row * key_order.len() + col] = key_order[col] * column_length + row;
            }
        }

        result
    }
}

impl Asymmetric for ColumnTransposition {
    fn encrypt(&self, language: &mut Language, msg: &str) -> String {
        let plaintext = language.string_to_vec(msg);
        let key_order = self.keyword.find_order();
        let mut iter = ColumnTransposition::encrypt_indexes(plaintext.len(), key_order).into_iter();
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
        let mut iter =
            ColumnTransposition::decrypt_indexes(ciphertext.len(), key_order).into_iter();
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

impl Keyed for ColumnTransposition {
    fn new(language: &mut Language) -> ColumnTransposition {
        let mut result = ColumnTransposition {
            keyword: Keyword::identity(language),
        };

        result
            .keyword
            .key_info_mut()
            .set("Keyword", "<string>", "kw");

        result
    }
    fn keys(&self) -> Vec<&dyn IoKey> {
        vec![&self.keyword]
    }
    fn keys_mut(&mut self) -> Vec<&mut dyn IoKey> {
        vec![&mut self.keyword]
    }
}

impl Solve for ColumnTransposition {
    fn solve(&mut self, language: &mut Language, msg: &str) {
        let ciphertext = language.string_to_vec(msg);
        let key = super::transposition_solve(
            &ciphertext,
            language,
            ColumnTransposition::decrypt_indexes,
            |row, col, _, num_rows| col * num_rows + row,
        );
        self.keyword.set(language, key.as_slice()).unwrap();
    }
}
