use crate::cipher::{Asymmetric, Keyed};
use crate::lang::Language;
use crate::util;

pub struct Caesar {
    pub shift: i16,
}

impl Asymmetric for Caesar {
    fn encrypt(&self, language: &Language, msg: &str) -> String {
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
    fn decrypt(&self, language: &Language, msg: &str) -> String {
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
    fn new(_language: &Language) -> Caesar {
        Caesar { shift: 0 }
    }
    fn reset(&mut self, _language: &Language) {
        self.shift = 0;
    }
    fn randomize(&mut self, language: &Language, rng: &mut impl rand::Rng) {
        self.shift = rng.gen_range(0..language.cp_count());
    }
    fn to_string(&self, _language: &Language) -> String {
        format!("shift:{}", self.shift)
    }
}
