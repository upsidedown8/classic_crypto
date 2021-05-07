use std::collections::HashMap;

extern crate serde;
use serde::{Deserialize, Serialize};
use crate::lang::LangAlphabet;

/// Provides compatability with different cipher alphabet lengths, performs conversions
/// from letters to code points (from `0..alphabet_size`), scores plaintext data (Coming soon),
/// recognises letters/punctuation and can be configured from a single `JSON` file.
///
#[derive(Serialize, Deserialize, Default)]
pub struct Language {
    /// The name of the `Language`
    ///
    pub name: String,

    /// Length of the standard alphabet
    ///
    pub alphabet_len: i16,

    /// Stores all supported alphabets
    /// 
    pub alphabets: Vec<LangAlphabet>,

    /// Maps diacritics -> standard letter(s)
    ///
    pub substitution_table: HashMap<char, String>,

    #[serde(skip)]
    selected_alph_idx: usize,
}

impl Language {
    /// Gets the currently selected alphabet
    /// 
    fn alph(&self) -> &LangAlphabet {
        &self.alphabets[self.selected_alph_idx]
    }

    /// Sets the current length of the alphabet. Returns true if the operation suceeded.
    /// 
    /// # Arguments
    /// 
    /// * len The desired length of the alphabet
    /// 
    pub fn set_alph_len(&mut self, len: usize) -> bool {
        if let Some(idx) = self.alphabets.iter().position(|x| x.length() == len) {
            self.selected_alph_idx = idx;
            true
        } else {
            false
        }
    }

    /* -------------------------------------------------------------------------- */
    /*                             Current Alphabet                               */
    /* -------------------------------------------------------------------------- */

    /// Converts a letter to its code point
    /// 
    /// # Arguments
    /// 
    /// * letter The letter to convert
    /// 
    /// # Panics
    /// 
    /// If the char provided does not satisfy `self.is_letter()`
    /// 
    pub fn get_cp(&self, letter: &char) -> i16 {
        assert!(self.is_letter(letter));

        self.alph().char_to_cp[letter]
    }

    /// Updates the code point of a letter, whilst keeping the correct case
    /// 
    /// # Arguments
    /// 
    /// * old_letter A letter occupying the same space in the string, in the past.
    /// * new_cp The new code point to change the letter to
    /// 
    /// # Panics
    /// 
    /// If the char provided is not a letter (`self.is_letter()`) or `new_cp` does not satisfy `self.valid_cp()`.
    /// 
    pub fn update_cp(&self, old_letter: &char, new_cp: i16) -> char {
        assert!(self.valid_cp(new_cp));
        assert!(self.is_letter(old_letter));

        if self.is_upper(old_letter) {
            self.cp_to_upper(new_cp)
        } else {
            self.cp_to_lower(new_cp)
        }
    }

    /// Converts from a code point to its uppercase equivalant
    /// 
    /// # Arguments
    /// 
    /// * cp The code point to convert
    /// 
    pub fn cp_to_upper(&self, cp: i16) -> char {
        assert!(self.valid_cp(cp));

        self.alph().upper.chars().nth(cp as usize).unwrap()
    }

    /// Converts from a code point to its lowercase equivalant
    /// 
    /// # Arguments
    /// 
    /// * cp The code point to convert
    /// 
    pub fn cp_to_lower(&self, cp: i16) -> char {
        assert!(self.valid_cp(cp));

        self.alph().lower.chars().nth(cp as usize).unwrap()
    }

    /// Converts a letter to lowercase
    /// 
    /// # Arguments
    /// 
    /// * letter The letter to convert to lowercase
    ///
    pub fn to_lower(&self, letter: &char) -> char {
        if self.is_upper(letter) {
            self.cp_to_lower(self.get_cp(letter))
        } else {
            *letter
        }
    }

    /// Converts a letter to uppercase
    /// 
    /// # Arguments
    /// 
    /// * letter The letter to convert to uppercase
    ///
    pub fn to_upper(&self, letter: &char) -> char {
        if self.is_lower(letter) {
            self.cp_to_upper(self.get_cp(letter))
        } else {
            *letter
        }
    }

    /// Is a particular char a letter?
    /// 
    /// # Arguments
    /// 
    /// * letter The char to check
    /// 
    pub fn is_letter(&self, letter: &char) -> bool {
        self.alph().char_to_cp.contains_key(&letter)
    }

    /// Is a particular letter punctuation (not a letter)?
    /// 
    /// # Arguments
    /// 
    /// * letter The letter to check
    /// 
    pub fn is_punct(&self, letter: &char) -> bool {
        !self.is_letter(letter)
    }

    /// Is a particular letter uppercaes?
    /// 
    /// # Arguments
    /// 
    /// * letter The letter to check
    /// 
    pub fn is_upper(&self, letter: &char) -> bool {
        self.alph().upper.contains(*letter) ||
        self.alph().upper_substitutions.iter().any(|x| x.chars().next().unwrap() == *letter)
    }

    /// Is a particular letter lowercase?
    /// 
    /// # Arguments
    /// 
    /// * letter The letter to check
    /// 
    pub fn is_lower(&self, letter: &char) -> bool {
        self.alph().lower.contains(*letter) ||
        self.alph().lower_substitutions.iter().any(|x| x.chars().next().unwrap() == *letter)
    }

    /// Is the code point valid?
    /// 
    /// # Arguments
    /// 
    /// * cp The code point to validate
    /// 
    pub fn valid_cp(&self, cp: i16) -> bool {
        0 <= cp && cp <= self.max_cp()
    }

    /// Returns the max value of a code point
    /// 
    pub fn max_cp(&self) -> i16 {
        self.cp_count() - 1
    }

    /// Returns the number of distinct code points
    /// 
    pub fn cp_count(&self) -> i16 {
        self.alphabet_len() as i16
    }

    /// Returns the length of this alphabet variant
    ///
    pub fn alphabet_len(&self) -> usize {
        self.alph().length()
    }


    /* -------------------------------------------------------------------------- */
    /*                             String conversions                             */
    /* -------------------------------------------------------------------------- */
    /// Converts a borrowed slice of code points to a string. This calls `self.alph().cp_to_upper()` on each
    /// element of the slice.
    ///
    /// # Arguments
    ///
    /// * `arr` The slice to convert
    ///
    pub fn vec_to_string(&self, arr: &[i16]) -> String {
        arr.iter().map(|&i| self.cp_to_upper(i)).collect()
    }

    /// Converts a borrowed slice of a string. This calls cp_to_upper() on each
    /// element of the slice.
    ///
    /// # Arguments
    ///
    /// * `arr` The slice to convert
    ///
    pub fn string_to_vec(&self, string: &str) -> Vec<i16> {
        string
            .chars()
            .filter(|i| self.is_letter(i))
            .map(|i| self.get_cp(&i))
            .collect()
    }

    /* -------------------------------------------------------------------------- */
    /*                               Alphabet setup                               */
    /* -------------------------------------------------------------------------- */
    /// Adds an alphabet to the [`Language`]. For more information on alphabets, see [`LangAlphabet`].
    /// Returns Err if an alphabet with the same length already exists.
    /// 
    /// # Arguments
    /// 
    /// * alphabet The alphabet to add
    /// 
    pub fn add_alphabet(&mut self, alphabet: LangAlphabet) -> Result<(), &str> {
        if self.alphabets.iter().any(|x| x.length() == alphabet.length()) {
            Err("Alphabet with given length already exists")
        } else {
            self.alphabets.push(alphabet);
            Ok(())
        }
    }

    /// Removes an alphabet from the [`Language`]. For more information on alphabets, see [`LangAlphabet`].
    /// No errors occur if there is no such alphabet in the [`Language`].
    /// 
    /// # Arguments
    /// 
    /// * alphabet_len The length of alphabet to remove
    /// 
    pub fn del_alphabet(&mut self, alphabet_len: usize) {
        self.alphabets.retain(|x| x.length() != alphabet_len);
    }

    /* -------------------------------------------------------------------------- */
    /*                          Substitution table setup                          */
    /* -------------------------------------------------------------------------- */
    /// Adds a substitution to the self contained substitution table. Used optionally in preprocessing
    /// for strings before they are sent to ciphers. Use case: expanding ligatures i.e "Æ" -> "AE". Since
    /// the substitution can result in multiple letters, `char`s are mapped to `Strings`, which interfere
    /// with the single character based operations of all ciphers. Hence, substitutions are applied in
    /// a preprocessing stage.
    ///
    /// # Arguments
    ///
    /// * `from` The `char` (ligature) to substitute from
    /// * `to` The string to replace the ligature with
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// language.add_substitution(&'Æ', "AE");
    /// assert_eq!(String::from("AE"), language.substitute_string("Æ"));
    /// ```
    ///
    pub fn add_substitution(&mut self, from: &char, to: &str) {
        assert!(!self.substitution_table.contains_key(from));

        self.substitution_table.insert(*from, String::from(to));
    }

    /// Removes a substitution from the internal substitution table. `from` must already
    /// be a key in the substitution table.
    ///
    /// # Arguments
    ///
    /// * `from` The character that you no longer want to substitute
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// language.add_substitution(&'Æ', "AE");
    /// language.del_substitution(&'Æ');
    /// assert_eq!(String::from("Æ"), language.substitute_string("Æ"));
    /// ```
    ///
    pub fn del_substitution(&mut self, from: &char) {
        assert!(self.substitution_table.contains_key(from));

        self.substitution_table.remove(from);
    }

    /// Clears all ligature substitutions from the internal substitution table
    ///
    pub fn clear_substitutions(&mut self) {
        self.substitution_table.clear();
    }

    /* -------------------------------------------------------------------------- */
    /*                             Substitution table                             */
    /* -------------------------------------------------------------------------- */
    /// Performs the ligature substitutions stored in the internal substitution table to a string.
    /// See `Language::add_substitution()` for more details.
    ///
    /// # Arguments
    ///
    /// * `string` The string to substitute
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// language.add_substitution(&'Æ', "AE");
    /// assert_eq!(String::from("AE"), language.substitute_string("Æ"));
    /// ```
    ///
    pub fn substitute_string(&self, string: &str) -> String {
        let mut result: String = String::new();
        string.chars().for_each(|c| {
            if self.substitution_table.contains_key(&c) {
                result.push_str(&self.substitution_table[&c]);
            } else {
                result.push(c);
            }
        });
        result
    }
}
