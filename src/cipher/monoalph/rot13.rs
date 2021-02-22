use crate::cipher::Symmetric;
use crate::lang::Language;

pub struct ROT13 {}

impl Symmetric for ROT13 {
    fn run(&self, language: &Language, msg: &str) -> String {
        assert_eq!(language.cp_count() % 2, 0);
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let mut cp = language.get_cp(&c);
                    cp = language.cp_count() / 2 - cp;
                    language.update_cp(&c, cp)
                } else {
                    c
                }
            })
            .collect()
    }
}
