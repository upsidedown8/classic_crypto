use crate::{
    cipher::{Keyed, Symmetric},
    key::{
        ClassicVigSquare, Keyword, VigSquare, {Key, StatefulKey},
    },
    lang::Language,
};

pub struct Beaufort {
    square: ClassicVigSquare,
    pub keyword: Keyword,
}

impl Beaufort {}

impl Symmetric for Beaufort {
    fn run(&self, language: &Language, msg: &str) -> String {
        let mut count = 0;
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let new_cp = self.square.decrypt(
                        language.get_cp(&c),
                        self.keyword.at(count % self.keyword.len()),
                    );
                    count += 1;
                    language.update_cp(&c, new_cp)
                } else {
                    c
                }
            })
            .collect()
    }
}

impl Keyed for Beaufort {
    fn new(language: &Language) -> Beaufort {
        Beaufort {
            square: ClassicVigSquare::new(language),
            keyword: Keyword::new(language),
        }
    }
    fn reset(&mut self, language: &Language) {
        self.keyword.reset(language);
    }
    fn randomize(&mut self, language: &Language, rng: &mut impl rand::Rng) {
        self.keyword.randomize(language, rng);
    }
    fn to_string(&self, language: &Language) -> String {
        format!("Keyword:{}", self.keyword.to_string(language))
    }
}
