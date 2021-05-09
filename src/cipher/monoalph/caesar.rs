use crate::lang::Language;
use crate::util;
use crate::{
    cipher::{Asymmetric, Keyed, Solve},
    lang::ScoreSize,
};

pub struct Caesar {
    pub shift: i16,
}

impl Asymmetric for Caesar {
    fn encrypt(&self, language: &mut Language, msg: &str) -> String {
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let mut cp = language.get_cp(&c);
                    cp = util::modulo(cp + self.shift, language.cp_count());
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
                    cp = util::modulo(cp - self.shift, language.cp_count());
                    language.update_cp(&c, cp)
                } else {
                    c
                }
            })
            .collect()
    }
}

impl Keyed for Caesar {
    fn new(_language: &mut Language) -> Caesar {
        Caesar { shift: 0 }
    }
    fn reset(&mut self, _language: &mut Language) {
        self.shift = 0;
    }
    fn randomize(&mut self, language: &mut Language, rng: &mut impl rand::Rng) {
        self.shift = rng.gen_range(0..language.cp_count());
    }
    fn to_string(&self, _language: &mut Language) -> String {
        format!("shift:{}", self.shift)
    }
}

impl Solve for Caesar {
    fn solve(&mut self, language: &mut Language, msg: &str) {
        let mut best_score = f64::MIN;

        let ciphertext = language.string_to_vec(msg);

        for shift in 0..language.cp_count() {
            let score = language.score_iter(
                ciphertext
                    .iter()
                    .map(|&cp| util::modulo(cp - shift, language.cp_count())),
                ScoreSize::Quadgrams,
            );

            if score > best_score {
                best_score = score;
                self.shift = shift;
            }
        }
    }
}
