use crate::cipher::Symmetric;
use crate::lang::Language;

pub struct Atbash {}

impl Symmetric for Atbash {
    fn run(&self, language: &mut Language, msg: &str) -> String {
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let mut cp = language.get_cp(&c);
                    cp = language.cp_count() - 1 - cp;
                    language.update_cp(&c, cp)
                } else {
                    c
                }
            })
            .collect()
    }
}
