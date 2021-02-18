use rand::prelude::SliceRandom;

use crate::cipher::cipher::{Asymmetric, Keyed};
use crate::lang::Language;
use crate::util;

const A_VALUES: [i16; 12] = [ 1, 3, 5, 7, 9, 11, 15, 17, 19, 21, 23, 25];

pub struct Affine {
    a: i16,
    b: i16
}

impl Affine {
    pub fn solve_known_pair(&mut self, language: &Language, plain0: i16, plain1: i16, cipher0: i16, cipher1: i16) {
        let d = util::modulo(plain0 - plain1, language.cp_count());
        let inv_d = util::mmi(d, language.cp_count())
            .expect("Failed to calculate modular inverse");
        self.a = util::modulo(inv_d * (cipher0 - cipher1), language.cp_count());
        self.b = util::modulo(inv_d * (plain0*cipher1 - plain1*cipher0), language.cp_count());
    }
}

impl Asymmetric for Affine {
    fn encrypt(&self, language: &Language, msg: &String) -> String {
        msg
            .chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let mut cp = language.get_cp(&c);
                    cp = util::modulo(self.a*cp + self.b, language.cp_count());
                    language.update_cp(&c, cp)
                } else {
                    c
                }
            })
            .collect()
    }
    fn decrypt(&self, language: &Language, msg: &String) -> String {
        let mmi = util::mmi(self.a, language.cp_count())
            .expect("Alphabet length should be coprime to the value of A");
        msg
            .chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let mut cp = language.get_cp(&c);
                    cp = util::modulo(mmi * (cp - self.b), language.cp_count());
                    language.update_cp(&c, cp)
                } else {
                    c
                }
            })
            .collect()
    }
}

impl Keyed for Affine {
    fn reset(&mut self, _language: &Language) {
        self.a = 1;
        self.b = 0;
    }
    fn randomize(&mut self, language: &Language, rng: &mut impl rand::Rng) {
        self.a = *A_VALUES.choose(rng).unwrap_or(&1);
        self.b = rng.gen_range(0..language.cp_count());
    }
    fn to_string(&self, _language: &Language) -> String {
        format!("Affine: A({}) B({})", self.a, self.b)
    }
}