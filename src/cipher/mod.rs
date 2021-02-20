pub mod monoalph;
pub mod polyalph;

use crate::lang::Language;

pub trait Symmetric {
    fn run(&self, language: &Language, msg: &String) -> String;
}

pub trait Asymmetric {
    fn encrypt(&self, language: &Language, msg: &String) -> String;
    fn decrypt(&self, language: &Language, msg: &String) -> String;
}

pub trait Keyed {
    fn new(language: &Language) -> Self;
    fn reset(&mut self, language: &Language);
    fn randomize(&mut self, language: &Language, rng: &mut impl rand::Rng);
    fn to_string(&self, language: &Language) -> String;
}