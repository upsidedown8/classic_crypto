//!
//! All key representations reside in this module, even if said key is only used
//! for a single cipher. All keys implement the `Key` trait, and any keys that have
//! state implement `StatefulKey` (lookup tables such as `ClassicVigSquare` do not
//! need to implement this trait). Keys typically implement `KeyFrom<T>` and `SetKey<T>`
//! for initialization and set operations.
//!

use crate::{error::Result, lang::Language};

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
pub use matrix::{Matrix, MatrixDimSize};
pub use number::Number;
pub use vigenere_square::bellaso_square::BellasoSquare;
pub use vigenere_square::classic_vig_square::ClassicVigSquare;
pub use vigenere_square::keyed_vig_square::KeyedVigSquare;
pub use vigenere_square::porta_square::PortaSquare;
pub use vigenere_square::VigSquare;
// pub use polybius_square::PolybiusSquare;
// pub use straddle_checkerboard::StraddleCheckerboard;

#[derive(Default, Clone)]
pub struct KeyInfo {
    pub name: String,
    pub desc: String,
    pub short_name: String,
}

impl KeyInfo {
    pub fn set(&mut self, name: &str, desc: &str, short_name: &str) {
        self.name = name.to_string();
        self.desc = desc.to_string();
        self.short_name = short_name.to_string();
    }
}

/// Trait implemented by all cipher keys
///
/// `T` is a datatype to create a key from
///
pub trait Key<T> {
    /// Creates a new `Key` implementation using a value of type `T`. Methods on individual
    /// `Key` implementations may offer more control over the parameters of a key.
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded [`Language`]
    /// * `arg` The argument with which to initialize the `Key`
    ///
    fn new(language: &mut Language, arg: T) -> Result<Box<Self>>;

    /// Sets the state of a `Key` implementation using a value of type `T`. Methods on individual
    /// `Key` implementations may offer more control over the parameters of a key.
    ///
    /// # Arguments
    ///
    /// * `language` An instance of the currently loaded [`Language`]
    /// * `arg` The argument with which to set the `Key` state
    ///
    fn set(&mut self, language: &mut Language, arg: T) -> Result<()>;
}

/// Trait implemented by cipher keys that have an identity version
pub trait IdentityKey {
    /// Creates a default/identity version of the key
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded [`Language`]
    ///
    fn identity(language: &mut Language) -> Self;
}

/// Trait implemented by [`Key`] implementations with state
pub trait StatefulKey {
    /// Resets the state to the default values. Where possible this will result in an identity key
    /// (A key that results in no encryption).
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded [`Language`]
    ///
    fn reset(&mut self, language: &mut Language);

    /// Converts the key to a string
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded [`Language`]
    ///
    fn to_string(&self, language: &mut Language) -> String;

    /// Randomizes the state of the key.
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded [`Language`]
    ///
    fn randomize(&mut self, language: &mut Language);
}

/// Trait implemented by [`Key`] implementations that want
/// to communicate with the CLI through a text based system.
pub trait IoKey: StatefulKey {
    /// Sets the value of the key using input provided in a string
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded [`Language`]
    /// * `data` The string with which to set the key
    ///
    fn set_key_str(&mut self, language: &mut Language, arg: &str) -> Result<()>;

    /// Get the key info for this key type
    ///
    fn key_info(&self) -> &KeyInfo;

    /// Get the mutable key info for this key type
    ///
    fn key_info_mut(&mut self) -> &mut KeyInfo;
}
