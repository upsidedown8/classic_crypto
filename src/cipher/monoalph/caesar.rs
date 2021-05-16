use crate::util;
use crate::{
    cipher::{Asymmetric, Keyed, Solve},
    lang::ScoreSize,
};
use crate::{
    key::{IoKey, Key, Number, StatefulKey},
    lang::Language,
};

pub struct Caesar {
    pub shift: Number,
}

impl Asymmetric for Caesar {
    fn encrypt(&self, language: &mut Language, msg: &str) -> String {
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let mut cp = language.get_cp(&c);
                    cp = util::modulo(cp + self.shift.get(), language.cp_count());
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
                    cp = util::modulo(cp - self.shift.get(), language.cp_count());
                    language.update_cp(&c, cp)
                } else {
                    c
                }
            })
            .collect()
    }
}

impl Keyed for Caesar {
    fn new(language: &mut Language) -> Caesar {
        let mut result = Caesar {
            shift: *Number::new(language, 0).unwrap(),
        };

        result.shift.key_info_mut().set(
            "Shift",
            "",
            "shift"
        );

        result
    }
    fn reset(&mut self, language: &mut Language) {
        self.shift.reset(language);
    }
    fn randomize(&mut self, language: &mut Language) {
        self.shift.randomize(language);
    }
    fn keys(&self) -> Vec<&dyn IoKey> {
        vec![&self.shift]
    }
    fn keys_mut(&mut self) -> Vec<&mut dyn IoKey> {
        vec![&mut self.shift]
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
                self.shift.set(language, shift).unwrap();
            }
        }
    }
}
