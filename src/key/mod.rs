use crate::lang::Language;
use rand::Rng;

mod alphabet;
mod cards;
mod enigma;
mod keyword;
mod matrix;
mod polybius_square;
mod straddle_checkerboard;
mod vigenere_square;

pub use alphabet::Alphabet;
pub use cards::Cards;
pub use enigma::plugboard::Plugboard;
pub use enigma::reflector::*;
pub use enigma::rotor::*;
pub use keyword::Keyword;
pub use matrix::Matrix;
pub use vigenere_square::bellaso_square::BellasoSquare;
pub use vigenere_square::classic_vig_square::ClassicVigSquare;
pub use vigenere_square::keyed_vig_square::KeyedVigSquare;
pub use vigenere_square::porta_square::PortaSquare;
pub use vigenere_square::VigSquare;
// pub use polybius_square::PolybiusSquare;
// pub use straddle_checkerboard::StraddleCheckerboard;

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
