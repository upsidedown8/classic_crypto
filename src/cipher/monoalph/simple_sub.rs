use crate::{key::{Alphabet, SetKey}, lang::ScoreSize, util};
use crate::lang::Language;
use crate::{
    cipher::{Asymmetric, Keyed, Solve},
    key::{Key, StatefulKey},
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
        SimpleSubstitution {
            alphabet: Alphabet::new(language),
        }
    }
    fn reset(&mut self, language: &mut Language) {
        self.alphabet.reset(language);
    }
    fn randomize(&mut self, language: &mut Language, rng: &mut impl rand::Rng) {
        self.alphabet.randomize(language, rng);
    }
    fn to_string(&self, language: &mut Language) -> String {
        format!("alphabet:{}", self.alphabet.to_string(language))
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

            util::shuffle(&mut inv_key, &mut rand::thread_rng());

            // keep trying all possible swaps until there is no further improvement
            let mut improved = true;
            while improved {
                improved = false;

                for i in 0..language.alphabet_len()-1 {
                    for j in i..language.alphabet_len() {
                        inv_key.swap(i, j);

                        let score = language.score_iter(
                            ciphertext
                                .iter()
                                .map(|&x| inv_key[x as usize]),
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

        self.alphabet.set_key(language, &util::invert(&inv_key));
    }
}
