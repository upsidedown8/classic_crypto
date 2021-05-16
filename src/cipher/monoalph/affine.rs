use rand::{prelude::IteratorRandom, Rng};

use crate::{
    cipher::{Asymmetric, Keyed, Solve},
    key::{IoKey, Key, Number, StatefulKey},
    lang::{Language, ScoreSize},
    util,
};

pub struct Affine {
    pub a: Number,
    pub b: Number,
}

impl Affine {
    pub fn solve_known_pair(
        &mut self,
        language: &mut Language,
        plain0: i16,
        plain1: i16,
        cipher0: i16,
        cipher1: i16,
    ) {
        let d = util::modulo(plain0 - plain1, language.cp_count());
        let inv_d = util::mmi(d, language.cp_count()).expect("Failed to calculate modular inverse");
        self.a
            .set(
                language,
                util::modulo(inv_d * (cipher0 - cipher1), language.cp_count()),
            )
            .unwrap();
        self.b
            .set(
                language,
                util::modulo(
                    inv_d * (plain0 * cipher1 - plain1 * cipher0),
                    language.cp_count(),
                ),
            )
            .unwrap();
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
                    cp = util::modulo(self.a.get() * cp + self.b.get(), language.cp_count());
                    language.update_cp(&c, cp)
                } else {
                    c
                }
            })
            .collect()
    }
    fn decrypt(&self, language: &mut Language, msg: &str) -> String {
        let mmi = util::mmi(self.a.get(), language.cp_count())
            .expect("Alphabet length should be coprime to the value of A");
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let mut cp = language.get_cp(&c);
                    cp = Affine::decrypt_one(language, cp, mmi, self.b.get());
                    language.update_cp(&c, cp)
                } else {
                    c
                }
            })
            .collect()
    }
}

impl Keyed for Affine {
    fn new(language: &mut Language) -> Affine {
        let mut result = Affine {
            a: *Number::new(language, 1).unwrap(),
            b: *Number::new(language, 0).unwrap(),
        };

        result.a.set_legal_values(Affine::valid_a_values(language));
        result
            .b
            .set_legal_values((0..language.cp_count()).collect::<Vec<_>>());
        result.a.key_info_mut().set("A", "", "a");
        result.b.key_info_mut().set("B", "", "b");

        result
    }
    fn reset(&mut self, language: &mut Language) {
        self.a.reset(language);
        self.b.reset(language);
    }
    fn randomize(&mut self, language: &mut Language) {
        self.a
            .set(
                language,
                *Affine::valid_a_values(language)
                    .iter()
                    .choose(&mut rand::thread_rng())
                    .unwrap_or(&1),
            )
            .unwrap();
        self.b
            .set(
                language,
                rand::thread_rng().gen_range(0..language.cp_count()),
            )
            .unwrap();
    }
    fn keys(&self) -> Vec<&dyn IoKey> {
        vec![&self.a, &self.b]
    }
    fn keys_mut(&mut self) -> Vec<&mut dyn IoKey> {
        vec![&mut self.a, &mut self.b]
    }
}

impl Solve for Affine {
    fn solve(&mut self, language: &mut Language, msg: &str) {
        let mut best_score = f64::MIN;

        let ciphertext = language.string_to_vec(msg);

        for a in Affine::valid_a_values(language) {
            if let Some(mmi) = util::mmi(a, language.cp_count()) {
                for b in 0..language.cp_count() {
                    let score = language.score_iter(
                        ciphertext
                            .iter()
                            .map(|&cp| Affine::decrypt_one(language, cp, mmi, b)),
                        ScoreSize::Quadgrams,
                    );

                    if score > best_score {
                        best_score = score;
                        self.a.set(language, a).unwrap();
                        self.b.set(language, b).unwrap();
                    }
                }
            }
        }
    }
}
