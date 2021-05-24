use crate::{
    cipher::{Asymmetric, Keyed, Solve},
    key::{Alphabet, IdentityKey, IoKey, Key},
    lang::{Language, ScoreSize},
    util,
};

pub struct SimpleSubstitution {
    pub alphabet: Alphabet,
}

impl Asymmetric for SimpleSubstitution {
    fn encrypt(&self, language: &mut Language, msg: &str) -> String {
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let mut cp = language.get_cp(&c);
                    cp = self.alphabet.encrypt(cp);
                    language.update_cp(&c, cp)
                } else {
                    c
                }
            })
            .collect()
    }
    fn decrypt(&self, language: &mut Language, msg: &str) -> String {
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let mut cp = language.get_cp(&c);
                    cp = self.alphabet.decrypt(cp);
                    language.update_cp(&c, cp)
                } else {
                    c
                }
            })
            .collect()
    }
}

impl Keyed for SimpleSubstitution {
    fn new(language: &mut Language) -> SimpleSubstitution {
        let mut result = SimpleSubstitution {
            alphabet: Alphabet::identity(language),
        };

        result.alphabet.info_mut().set("Alphabet", "alph");

        result
    }
    fn keys(&self) -> Vec<&dyn IoKey> {
        vec![&self.alphabet]
    }
    fn keys_mut(&mut self) -> Vec<&mut dyn IoKey> {
        vec![&mut self.alphabet]
    }
}

const MAX_ITERATIONS: usize = 1000;
const MAX_REPETITIONS: usize = 3;

impl Solve for SimpleSubstitution {
    fn solve(&mut self, language: &mut Language, msg: &str) {
        let ciphertext = language.string_to_vec(msg);

        let mut inv_key = (0..language.cp_count()).collect::<Vec<i16>>();
        let mut best_score: f64 = f64::MIN;
        let mut repetitions: usize = 0;

        for _ in 0..MAX_ITERATIONS {
            let mut local_best_score: f64 = f64::MIN;

            util::shuffle(&mut inv_key);

            // keep trying all possible swaps until there is no further improvement
            let mut improved = true;
            while improved {
                improved = false;

                for i in 0..language.alphabet_len() - 1 {
                    for j in i..language.alphabet_len() {
                        inv_key.swap(i, j);

                        let score = language.score_iter(
                            ciphertext.iter().map(|&x| inv_key[x as usize]),
                            ScoreSize::Quadgrams,
                        );

                        if score > local_best_score {
                            local_best_score = score;
                            improved = true;
                        } else {
                            inv_key.swap(i, j);
                        }
                    }
                }
            }

            // update repetition count
            if (local_best_score - best_score).abs() < 0.1 {
                repetitions += 1;
            } else {
                repetitions = 0;
            }

            best_score = best_score.max(local_best_score);

            if repetitions >= MAX_REPETITIONS {
                break;
            }
        }

        self.alphabet
            .set(language, util::invert(&inv_key).as_slice())
            .unwrap();
    }
}
