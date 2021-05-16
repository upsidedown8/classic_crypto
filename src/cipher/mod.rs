pub mod electromechanical;
pub mod monoalph;
pub mod polyalph;
pub mod polygraph;
pub mod stream;
pub mod transpos;

use crate::{key::IoKey, lang::Language};

/// Trait implemented by Symmetric ciphers (where encryption and decryption are identical).
pub trait Symmetric {
    /// Perform the encryption/decryption operation on `msg`. Since
    /// this cipher is symmetric, these operations are identical.
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded [`Language`]
    /// * `msg` The message to encrypt/decrypt
    ///
    fn run(&self, language: &mut Language, msg: &str) -> String;
}

/// Trait implemented by Asymmetric ciphers (where encryption and decryption are unique operations).
pub trait Asymmetric {
    /// Perform the encryption operation on `msg`.
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded [`Language`]
    /// * `msg` The message to encrypt
    ///
    fn encrypt(&self, language: &mut Language, msg: &str) -> String;

    /// Perform the decryption operation on `msg`.
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded [`Language`]
    /// * `msg` The message to decrypt
    ///
    fn decrypt(&self, language: &mut Language, msg: &str) -> String;
}

/// Trait implemented by ciphers that require a `Key`
pub trait Keyed {
    /// Create a new Keyed cipher, initialized with default state, which
    /// is, if possible, the identity form of each key.
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded [`Language`]
    ///
    fn new(language: &mut Language) -> Self;

    /// Get a vec of keys
    ///
    fn keys(&self) -> Vec<&dyn IoKey>;

    /// Get a vec of mutable keys
    ///
    fn keys_mut(&mut self) -> Vec<&mut dyn IoKey>;

    /// Reset the cipher state
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded [`Language`]
    ///
    fn reset(&mut self, language: &mut Language) {
        self.keys_mut()
            .iter_mut()
            .for_each(|key| key.reset(language));
    }

    /// Randomize the cipher state
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded [`Language`]
    ///
    fn randomize(&mut self, language: &mut Language) {
        self.keys_mut()
            .iter_mut()
            .for_each(|key| key.randomize(language));
    }

    /// Convert the cipher state to a string
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded [`Language`]
    ///
    fn to_string(&self, language: &mut Language) -> String {
        let mut result = String::new();

        let keys = self.keys();

        (0..keys.len()).for_each(|i| {
            result.push_str(&format!(
                "{}: {}",
                keys[i].key_info().name,
                keys[i].to_string(language),
            ));
            result.push('\n');
        });

        result
    }
}

/// Trait implemented by ciphers which can be automatically solved
pub trait Solve {
    /// Solve the ciphertext given in msg, the cipher instance will be updated with the
    /// key of the best solution.
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded [`Language`]
    /// * `msg` The message to solve
    ///
    fn solve(&mut self, language: &mut Language, msg: &str);
}
