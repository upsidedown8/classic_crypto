use crate::{
    cipher::{Asymmetric, Keyed, Solve},
    key::{IoKey, Key, Number},
    lang::{Language, ScoreSize},
};

pub struct Scytale {
    pub num_faces: Number,
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
        let mut iter =
            Scytale::encrypt_indexes(self.num_faces.get() as usize, plaintext.len()).into_iter();
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
            Scytale::decrypt_indexes(self.num_faces.get() as usize, ciphertext.len()).into_iter();
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
    fn new(language: &mut Language) -> Scytale {
        let mut result = Scytale {
            num_faces: *Number::new(language, 2).unwrap(),
        };

        result.num_faces.set_legal_values((2..15).collect());
        result
            .num_faces
            .key_info_mut()
            .set("Number of faces", "<integer>", "faces");

        result
    }
    fn keys(&self) -> Vec<&dyn IoKey> {
        vec![&self.num_faces]
    }
    fn keys_mut(&mut self) -> Vec<&mut dyn IoKey> {
        vec![&mut self.num_faces]
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
                self.num_faces.set(language, num_faces as i16).unwrap();
            }
        }
    }
}
