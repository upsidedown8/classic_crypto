pub mod electromechanical;
pub mod monoalph;
pub mod polyalph;
// pub mod polygraph;
// pub mod transpos;

use crate::lang::Language;

pub trait Symmetric {
    fn run(&self, language: &Language, msg: &str) -> String;
}

pub trait Asymmetric {
    fn encrypt(&self, language: &Language, msg: &str) -> String;
    fn decrypt(&self, language: &Language, msg: &str) -> String;
}

pub trait Keyed {
    fn new(language: &Language) -> Self;
    fn reset(&mut self, language: &Language);
    fn randomize(&mut self, language: &Language, rng: &mut impl rand::Rng);
    fn to_string(&self, language: &Language) -> String;
}
