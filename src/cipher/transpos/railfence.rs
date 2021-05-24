use crate::{
    cipher::{Asymmetric, Keyed, Solve},
    key::{IoKey, Key, Number},
    lang::{Language, ScoreSize},
};

pub struct Railfence {
    pub num_rails: Number,
}

impl Railfence {
    fn encrypt_indexes(num_rails: usize, len: usize) -> Vec<usize> {
        let mut matrix = vec![-1i32; num_rails * len];
        let mut result = Vec::with_capacity(len);

        let num_rails = num_rails as i32;
        let len = len as i32;

        let mut row = 0i32;
        let mut increment = -1i32;

        for col in 0..len {
            matrix[(row * len + col) as usize] = col;
            if row == 0 || row == num_rails - 1 {
                increment *= -1;
            }
            row += increment;
        }

        for row in 0..num_rails {
            for col in 0..len {
                let pos = (row * len + col) as usize;
                if matrix[pos] != -1 {
                    result.push(matrix[pos] as usize);
                }
            }
        }

        result
    }
    fn decrypt_indexes(num_rails: usize, len: usize) -> Vec<usize> {
        let mut matrix = vec![-1i32; num_rails * len];
        let mut result = Vec::with_capacity(len);

        let num_rails = num_rails as i32;
        let len = len as i32;

        let mut row = 0i32;
        let mut increment = -1i32;

        for col in 0..len {
            matrix[(row * len + col) as usize] = -2;
            if row == 0 || row == num_rails - 1 {
                increment *= -1;
            }
            row += increment;
        }

        let mut idx = 0;
        for row in 0..num_rails {
            for col in 0..len {
                let pos = (row * len + col) as usize;
                if matrix[pos] == -2 {
                    matrix[pos] = idx;
                    idx += 1;
                }
            }
        }

        row = 0;
        increment = -1;

        for col in 0..len {
            result.push(matrix[(row * len + col) as usize] as usize);
            if row == 0 || row == num_rails - 1 {
                increment *= -1;
            }
            row += increment;
        }

        result
    }
}

impl Asymmetric for Railfence {
    fn encrypt(&self, language: &mut Language, msg: &str) -> String {
        let plaintext = language.string_to_vec(msg);
        let mut iter =
            Railfence::encrypt_indexes(self.num_rails.get() as usize, plaintext.len()).into_iter();
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
        let mut iter =
            Railfence::decrypt_indexes(self.num_rails.get() as usize, ciphertext.len()).into_iter();
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

impl Keyed for Railfence {
    fn new(language: &mut Language) -> Railfence {
        let mut result = Railfence {
            num_rails: *Number::new(language, 1).unwrap(),
        };

        result.num_rails.set_legal_values((1..50).collect());
        result.num_rails.info_mut().set("Number of rails", "rails");

        result
    }
    fn keys(&self) -> Vec<&dyn IoKey> {
        vec![&self.num_rails]
    }
    fn keys_mut(&mut self) -> Vec<&mut dyn IoKey> {
        vec![&mut self.num_rails]
    }
}

impl Solve for Railfence {
    fn solve(&mut self, language: &mut Language, msg: &str) {
        let mut best_score = f64::MIN;

        let ciphertext = language.string_to_vec(msg);

        for num_rails in 2..50 {
            let score = language.score_iter(
                Railfence::decrypt_indexes(num_rails, ciphertext.len())
                    .iter()
                    .map(|&old_idx| ciphertext[old_idx]),
                ScoreSize::Quadgrams,
            );

            if score > best_score {
                best_score = score;
                self.num_rails.set(language, num_rails as i16).unwrap();
            }
        }
    }
}
