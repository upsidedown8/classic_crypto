pub mod electromechanical;
pub mod monoalph;
pub mod polyalph;
pub mod polygraph;
pub mod transpos;

use crate::lang::Language;

/// Trait implemented by Symmetric ciphers (where encryption and decryption are identical).
pub trait Symmetric {
    /// Perform the encryption/decryption operation on `msg`. Since
    /// this cipher is symmetric, these operations are identical.
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded `Language`
    /// * `msg` The message to encrypt/decrypt
    ///
    fn run(&self, language: &Language, msg: &str) -> String;
}

/// Trait implemented by Asymmetric ciphers (where encryption and decryption are unique operations).
pub trait Asymmetric {
    /// Perform the encryption operation on `msg`.
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded `Language`
    /// * `msg` The message to encrypt
    ///
    fn encrypt(&self, language: &Language, msg: &str) -> String;

    /// Perform the decryption operation on `msg`.
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded `Language`
    /// * `msg` The message to decrypt
    ///
    fn decrypt(&self, language: &Language, msg: &str) -> String;
}

/// Trait implemented by ciphers that require a `Key`
pub trait Keyed {
    /// Create a new Keyed cipher, initialized with default state, which
    /// is, if possible, the identity form of each key.
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded `Language`
    ///
    fn new(language: &Language) -> Self;

    /// Reset the cipher state
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded `Language`
    ///
    fn reset(&mut self, language: &Language);

    /// Randomize the cipher state
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded `Language`
    /// * `rng` A rand::Rng implementation to generate random numbers
    ///
    fn randomize(&mut self, language: &Language, rng: &mut impl rand::Rng);

    /// Convert the cipher state to a string
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded `Language`
    ///
    fn to_string(&self, language: &Language) -> String;
}
