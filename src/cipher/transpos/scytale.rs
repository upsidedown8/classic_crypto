use crate::lang::Language;
use crate::{
    cipher::{Asymmetric, Keyed, Solve},
    lang::ScoreSize,
};

pub struct Scytale {
    pub num_faces: usize,
}

impl Scytale {
    fn encrypt_indexes(num_faces: usize, len: usize) -> Vec<usize> {
        let mut result = vec![0; len];

        let mut idx = 0;
        for row in 0..num_faces {
            for i in (row..len).step_by(num_faces) {
                result[i] = idx;
                idx += 1;
            }
        }

        result
    }
    fn decrypt_indexes(num_faces: usize, len: usize) -> Vec<usize> {
        let mut result = vec![0; len];

        let mut idx = 0;
        for row in 0..num_faces {
            for i in (row..len).step_by(num_faces) {
                result[idx] = i;
                idx += 1;
            }
        }

        result
    }
}

impl Asymmetric for Scytale {
    fn encrypt(&self, language: &mut Language, msg: &str) -> String {
        let plaintext = language.string_to_vec(msg);
        let mut iter = Scytale::encrypt_indexes(self.num_faces, plaintext.len()).into_iter();
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
        let mut iter = Scytale::decrypt_indexes(self.num_faces, ciphertext.len()).into_iter();
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

impl Keyed for Scytale {
    fn new(_language: &mut Language) -> Scytale {
        Scytale { num_faces: 2 }
    }
    fn reset(&mut self, _language: &mut Language) {
        self.num_faces = 2;
    }
    fn randomize(&mut self, _language: &mut Language, rng: &mut impl rand::Rng) {
        self.num_faces = rng.gen_range(2..15);
    }
    fn to_string(&self, _language: &mut Language) -> String {
        format!("Faces:{}", self.num_faces)
    }
}

impl Solve for Scytale {
    fn solve(&mut self, language: &mut Language, msg: &str) {
        let mut best_score = f64::MIN;

        let ciphertext = language.string_to_vec(msg);

        for num_faces in 2..50 {
            let score = language.score_iter(
                Scytale::decrypt_indexes(num_faces, ciphertext.len())
                    .iter()
                    .map(|&old_idx| ciphertext[old_idx]),
                ScoreSize::Quadgrams,
            );

            if score > best_score {
                best_score = score;
                self.num_faces = num_faces;
            }
        }
    }
}
