pub mod bellaso_square;
pub mod classic_vig_square;
pub mod keyed_vig_square;
pub mod porta_square;

use crate::lang::Language;

/// Trait implemented by vigenere square type `Key`s
pub trait VigSquare {
    /// All vigenere squares need a method to initialize the table
    ///
    fn init_squares(&mut self);

    /// Encrypts a letter `x` using key `y` typically by looking up on the `square`
    /// member of the struct.
    ///
    /// # Arguments
    ///
    /// * `x` The letter (code point) to encrypt
    /// * `y` The key (code point) to encrypt `x` with
    ///
    fn encrypt(&self, x: i16, y: i16) -> i16;

    /// Decrypts a letter `x` using key `y` typically by looking up on the `inverse`
    /// member of the struct. For `Symmetrical` ciphers, the `VigSquare` implementation
    /// calls `self.encrypt(x, y)` from the `decrypt` method.
    ///
    /// # Arguments
    ///
    /// * `x` The letter (code point) to decrypt
    /// * `y` The key (code point) to decrypt `x` with
    ///
    fn decrypt(&self, x: i16, y: i16) -> i16;

    /// Single method used by all `VigSquare` implementations that converts the square to a `String`
    /// representation.
    ///
    /// # Arguments
    ///
    /// * `language` A borrowed instance of the currently loaded [`Language`]
    /// * `square` A borrowed slice of vectors representing the square
    /// * `max_y` The vertical maximum bound (exclusive)
    /// * `max_x` The horizontal maximum bound (exclusive)
    ///  
    fn vig_square_to_string(
        language: &Language,
        square: &[Vec<i16>],
        max_y: usize,
        max_x: usize,
    ) -> String {
        assert_eq!(language.alphabet_len(), 26);
        let mut square_as_string = String::new();

        let is_porta = max_y == 13;

        // left padding
        square_as_string.push_str(if is_porta { "    | " } else { "  | " });

        // print headers
        for i in 0..26 {
            square_as_string.push(language.cp_to_upper(i));
            square_as_string.push(' ');
        }
        if is_porta {
            square_as_string
                .push_str("\n====|====================================================\n");
        } else {
            square_as_string
                .push_str("\n==|====================================================\n");
        }

        // main square
        for y in 0..max_y as i16 {
            if is_porta {
                square_as_string.push(language.cp_to_upper(y / 2));
                square_as_string.push(',');
                square_as_string.push(language.cp_to_upper(y / 2 + 1));
            } else {
                square_as_string.push(language.cp_to_upper(y));
            }
            square_as_string.push_str(" | ");
            for x in 0..max_x {
                square_as_string.push(language.cp_to_upper(square[y as usize][x]));
                square_as_string.push(' ');
            }
            square_as_string.push('\n')
        }

        square_as_string
    }
}
