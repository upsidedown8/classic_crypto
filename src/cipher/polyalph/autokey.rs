use crate::{cipher::{Asymmetric, Keyed, Solve}, key::{{Key, StatefulKey}, ClassicVigSquare, Keyword, SetKey, VigSquare}, lang::Language};

pub struct Autokey {
    square: ClassicVigSquare,
    pub keyword: Keyword,
}

impl Autokey {}

impl Asymmetric for Autokey {
    fn encrypt(&self, language: &mut Language, msg: &str) -> String {
        let mut count = 0;
        let mut pt_vec = vec![0; self.keyword.len()];
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let idx = count % self.keyword.len();
                    let cp = language.get_cp(&c);
                    let new_cp = self.square.encrypt(
                        cp,
                        if count < self.keyword.len() {
                            self.keyword.at(count)
                        } else {
                            pt_vec[idx]
                        },
                    );
                    pt_vec[idx] = cp;
                    count += 1;
                    language.update_cp(&c, new_cp)
                } else {
                    c
                }
            })
            .collect()
    }
    fn decrypt(&self, language: &mut Language, msg: &str) -> String {
        let mut count = 0;
        let mut pt_vec = vec![0; self.keyword.len()];
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let idx = count % self.keyword.len();
                    pt_vec[idx] = self.square.decrypt(
                        if count < self.keyword.len() {
                            self.keyword.at(count)
                        } else {
                            pt_vec[idx]
                        },
                        language.get_cp(&c),
                    );
                    count += 1;
                    language.update_cp(&c, pt_vec[idx])
                } else {
                    c
                }
            })
            .collect()
    }
}

impl Keyed for Autokey {
    fn new(language: &mut Language) -> Autokey {
        Autokey {
            square: ClassicVigSquare::new(language),
            keyword: Keyword::new(language),
        }
    }
    fn reset(&mut self, language: &mut Language) {
        self.keyword.reset(language);
    }
    fn randomize(&mut self, language: &mut Language, rng: &mut impl rand::Rng) {
        self.keyword.randomize(language, rng);
    }
    fn to_string(&self, language: &mut Language) -> String {
        format!("Keyword:{}", self.keyword.to_string(language))
    }
}

impl Solve for Autokey {
    fn solve(&mut self, language: &mut Language, msg: &str) {
        let ciphertext = language.string_to_vec(msg);
        self.keyword.set_key(
            language,
            &crate::cipher::polyalph::vig_solve(&ciphertext, 1, language, |cp, shift| {
                self.square.decrypt(shift, cp)
            }, true),
        )
    }
}
