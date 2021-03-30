//!
//! All key representations reside in this module, even if said key is only used
//! for a single cipher. All keys implement the `Key` trait, and any keys that have
//! state implement `StatefulKey` (lookup tables such as `ClassicVigSquare` do not
//! need to implement this trait). Keys typically implement `KeyFrom<T>` and `SetKey<T>`
//! for initialization and set operations.
//!

use crate::lang::Language;
use rand::Rng;

mod alphabet;
mod cards;
mod enigma;
mod keyword;
mod matrix;
mod number;
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
pub use number::Number;
pub use vigenere_square::bellaso_square::BellasoSquare;
pub use vigenere_square::classic_vig_square::ClassicVigSquare;
pub use vigenere_square::keyed_vig_square::KeyedVigSquare;
pub use vigenere_square::porta_square::PortaSquare;
pub use vigenere_square::VigSquare;
// pub use polybius_square::PolybiusSquare;
// pub use straddle_checkerboard::StraddleCheckerboard;

/// Trait to initialize a new key with a value of type `T`
pub trait KeyFrom<T> {
    /// Creates a new `Key` implementation using a value of type `T`. Methods on individual
    /// `Key` implementations may offer more control over the parameters of a key.
    ///
    /// # Arguments
    ///
    /// * [`Language`] A borrowed instance of the currently loaded [`Language`]
    /// * `_` The argument with which to initialize the `Key`
    ///
    fn create_from(language: &Language, _: T) -> Self;
}

/// Trait to set the state of a key with a value of type `T`
pub trait SetKey<T> {
    /// Sets the state of a `Key` implementation using a value of type `T`. Methods on individual
    /// `Key` implementations may offer more control over the parameters of a key.
    ///
    /// # Arguments
    ///
    /// * [`Language`] A borrowed instance of the currently loaded [`Language`]
    /// * `_` The argument with which to set the `Key` state
    ///
    fn set_key(&mut self, language: &Language, _: T);
}

/// Trait implemented by all cipher keys
pub trait Key {
    /// String representation of the current key state (or of the stateless key in the case of
    /// Vigenere squares).
    ///
    /// # Arguments
    ///
    /// * [`Language`] A borrowed instance of the currently loaded [`Language`]
    ///
    fn to_string(&self, language: &Language) -> String;

    /// Initializes a new instance of the `Key` implementation with default values, which may be
    /// influenced by the [`Language`] config.
    ///
    /// # Arguments
    ///
    /// * [`Language`] A borrowed instance of the currently loaded [`Language`]
    ///
    fn new(language: &Language) -> Self;
}

/// Trait implemented by `Key` implementations with state
pub trait StatefulKey {
    /// Resets the state to the default values. Where possible this will result in an identity key
    /// (A key that results in no encryption).
    ///
    /// # Arguments
    ///
    /// * [`Language`] A borrowed instance of the currently loaded [`Language`]
    ///
    fn reset(&mut self, language: &Language);

    /// Randomizes the state of the key.
    ///
    /// # Arguments
    ///
    /// * [`Language`] A borrowed instance of the currently loaded [`Language`]
    /// * `rng` A rand::Rng implementation to generate random numbers
    ///
    fn randomize(&mut self, language: &Language, rng: &mut impl Rng);
}
