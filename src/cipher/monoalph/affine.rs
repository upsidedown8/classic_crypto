use rand::prelude::IteratorRandom;

use crate::{cipher::{Asymmetric, Keyed, Solve}, lang::ScoreSize};
use crate::lang::Language;
use crate::util;

pub struct Affine {
    pub a: i16,
    pub b: i16,
}

impl Affine {
    pub fn solve_known_pair(
        &mut self,
        language: &Language,
        plain0: i16,
        plain1: i16,
        cipher0: i16,
        cipher1: i16,
    ) {
        let d = util::modulo(plain0 - plain1, language.cp_count());
        let inv_d = util::mmi(d, language.cp_count()).expect("Failed to calculate modular inverse");
        self.a = util::modulo(inv_d * (cipher0 - cipher1), language.cp_count());
        self.b = util::modulo(
            inv_d * (plain0 * cipher1 - plain1 * cipher0),
            language.cp_count(),
        );
    }

    fn valid_a_values(language: &Language) -> Vec<i16> {
        (1..language.cp_count())
            .filter(|n| util::mmi(*n, language.cp_count()) != None)
            .collect()
    }

    fn decrypt_one(language: &Language, cp: i16, mmi: i16, b: i16) -> i16 {
        util::modulo(mmi * (cp - b), language.cp_count())
    }
}

impl Asymmetric for Affine {
    fn encrypt(&self, language: &mut Language, msg: &str) -> String {
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let mut cp = language.get_cp(&c);
                    cp = util::modulo(self.a * cp + self.b, language.cp_count());
                    language.update_cp(&c, cp)
                } else {
                    c
                }
            })
            .collect()
    }
    fn decrypt(&self, language: &mut Language, msg: &str) -> String {
        let mmi = util::mmi(self.a, language.cp_count())
            .expect("Alphabet length should be coprime to the value of A");
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let mut cp = language.get_cp(&c);
                    cp = Affine::decrypt_one(language, cp, mmi, self.b);
                    language.update_cp(&c, cp)
                } else {
                    c
                }
            })
            .collect()
    }
}

impl Keyed for Affine {
    fn new(_language: &mut Language) -> Affine {
        Affine { a: 1, b: 0 }
    }
    fn reset(&mut self, _language: &mut Language) {
        self.a = 1;
        self.b = 0;
    }
    fn randomize(&mut self, language: &mut Language, rng: &mut impl rand::Rng) {
        self.a = *Affine::valid_a_values(language)
            .iter()
            .choose(rng)
            .unwrap_or(&1);
        self.b = rng.gen_range(0..language.cp_count());
    }
    fn to_string(&self, _language: &mut Language) -> String {
        format!("A:{}, B:{}", self.a, self.b)
    }
}

impl Solve for Affine {
    fn solve(&mut self, language: &mut Language, msg: &str) {
        let mut best_score = f64::MIN;

        let ciphertext = language.string_to_vec(msg);

        for a in Affine::valid_a_values(language) {
            if let Some(mmi) = util::mmi(a, language.cp_count()) {
                for b in 0..language.cp_count() {
                    let plaintext: Vec<i16> = ciphertext
                        .iter()
                        .map(|&cp| Affine::decrypt_one(language, cp, mmi, b))
                        .collect();
                    let score = language.score(&plaintext, ScoreSize::Quadgrams);
    
                    if score > best_score {
                        best_score = score;
                        self.a = a;
                        self.b = b;
                    }    
                }
            }
        }
    }
}
