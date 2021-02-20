
use rand::Rng;
use crate::lang::Language;

pub mod alphabet;
pub mod cards;
pub mod keyword;
pub mod matrix;
pub mod polybius_square;
pub mod straddle_checkerboard;
pub mod vigenere_square;

pub trait KeyFrom<T> {
    fn create_from(language: &Language, _: T) -> Self;
}

pub trait SetKey<T> {
    fn set_key(&mut self, language: &Language, _: T);
}

pub trait Key {
    fn to_string(&self, language: &Language) -> String;

    fn new(language: &Language) -> Self;
}

pub trait StatefulKey {
    fn reset(&mut self, language: &Language);
    fn randomize(&mut self, language: &Language, rng: &mut impl Rng);
}