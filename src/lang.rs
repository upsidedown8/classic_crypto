//!
//! Module containing structs for generalizing cipher algorithms, so that individual
//! ciphers are only aware of the requirements of the cipher alphabet, such as the
//! number of letters needed.
//!

use std::collections::HashMap;

extern crate serde;
use serde::{Deserialize, Serialize};

/// Provides compatability with different cipher alphabet lengths, performs conversions
/// from letters to code points (from `0..alphabet_size`), scores plaintext data (Coming soon),
/// recognises letters/punctuation and can be configured from a single `JSON` file.
///
#[derive(Serialize, Deserialize, Default)]
pub struct Language {
    /// The name of the `Language`
    ///
    pub name: String,

    /// Maps characters -> code points
    ///  
    char_to_cp: HashMap<char, i16>,

    /* alphabets */
    /// Uppercase alphabet
    ///
    upper_alphabet: Vec<char>,

    /// Lowercase alphabet
    ///
    lower_alphabet: Vec<char>,

    /// Length of the standard alphabet
    ///
    alphabet_len: i16,

    /// Maps diacritics -> standard letter(s)
    ///
    substitution_table: HashMap<char, String>,
}

impl Language {
    /* -------------------------------------------------------------------------- */
    /*                               Alphabet setup                               */
    /* -------------------------------------------------------------------------- */

    /// Adds a code point to the alphabet. Each code point requires an uppercase letter
    /// and a lowercase letter, which can be the same. Neither letter should have been added
    /// to any other code point before. Since code points must be consecutive, the value of the
    /// code point is not required. It is simply pushed to all existing collections.
    ///
    /// # Detail
    ///
    /// Returns the value of the code point.
    /// The code points should be added in the order that they naturally occur in the alphabet,
    /// so that shift ciphers such as `Caesar` can achieve a logical progression through the
    /// chosen alphabet. The actual Unicode values of the characters are ignored, so the ordering
    /// of code points is essential. When the code point is added, the alphabet_len is also incremented.
    ///
    /// # Arguments
    ///
    /// * `upper` The uppercase equivalent of the code point
    /// * `lower` The lowercase equivalent of the code point
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    /// assert_eq!(language.get_cp(&'A'), 0);
    /// assert_eq!(language.get_cp(&'a'), 0);
    /// assert_eq!(language.get_cp(&'Z'), 25);
    /// assert_eq!(language.get_cp(&'z'), 25);
    /// ```
    ///
    pub fn new_cp(&mut self, upper: &char, lower: &char) -> i16 {
        assert!(!self.char_to_cp.contains_key(upper));
        assert!(!self.char_to_cp.contains_key(lower));
        assert!(!self.upper_alphabet.contains(upper));
        assert!(!self.lower_alphabet.contains(lower));

        let cp = self.max_cp() + 1;

        self.char_to_cp.insert(*upper, cp);
        self.char_to_cp.insert(*lower, cp);

        self.upper_alphabet.push(*upper);
        self.lower_alphabet.push(*lower);
        self.alphabet_len += 1;

        cp
    }

    /// Edit the upper and lower equivalents of an existing code point, `cp`. When this
    /// method is run **all existing alternatives** for that code point will be deleted,
    /// and overridden by the new `upper` and `lower` values. The `cp` provided must be valid.
    ///
    /// # Arguments
    ///
    /// * `cp` The value of the code point to update. This must be valid
    /// * `upper` The new uppercase equivalent of the code point
    /// * `lower` The new lowercase equivalent of the code point
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    /// let cp_of_letter_c = language.get_cp(&'c');
    /// language.edit_cp(cp_of_letter_c, &'Ç', &'ç');
    ///
    /// assert_eq!(language.get_cp(&'Ç'), cp_of_letter_c);
    /// assert_eq!(language.get_cp(&'ç'), cp_of_letter_c);
    /// ```
    ///
    pub fn edit_cp(&mut self, cp: i16, upper: &char, lower: &char) {
        assert!(self.valid_cp(cp));

        self.char_to_cp.retain(|_, v| *v != cp);
        self.char_to_cp.insert(*upper, cp);
        self.char_to_cp.insert(*lower, cp);
        self.upper_alphabet[cp as usize] = *upper;
        self.lower_alphabet[cp as usize] = *lower;
    }

    /// Adds an alias to an existing (*and valid*) code point `cp`. Use case is when you want
    /// to support multiple Unicode representations of the same letter, for example "ĆćÇçĉĈċĊ"
    /// are all different characters that could be considered as an english "C". An alias can be
    /// added for each of these letters so they all point to the same code point.
    ///
    /// # Detail
    ///
    /// The alias must not already have been added for any other code point, and the code point
    /// itself must be valid.
    ///
    /// # Arguments
    ///
    /// * `cp` The code point which the new alias will point to
    /// * `alias` The `char` to map to the existing code point
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    /// let cp_of_letter_c = language.get_cp(&'c');
    /// for alias in "ĆćÇçĉĈċĊ".chars() {
    ///     language.add_cp_alias(cp_of_letter_c, &alias);    
    /// }
    ///
    /// assert_eq!(language.get_cp(&'Ć'), cp_of_letter_c);
    /// assert_eq!(language.get_cp(&'ć'), cp_of_letter_c);
    /// assert_eq!(language.get_cp(&'Ç'), cp_of_letter_c);
    /// assert_eq!(language.get_cp(&'ç'), cp_of_letter_c);
    /// assert_eq!(language.get_cp(&'ĉ'), cp_of_letter_c);
    /// assert_eq!(language.get_cp(&'Ĉ'), cp_of_letter_c);
    /// assert_eq!(language.get_cp(&'ċ'), cp_of_letter_c);
    /// assert_eq!(language.get_cp(&'Ċ'), cp_of_letter_c);
    /// ```
    ///
    pub fn add_cp_alias(&mut self, cp: i16, alias: &char) {
        assert!(!self.char_to_cp.contains_key(alias));
        assert!(self.valid_cp(cp));

        self.char_to_cp.insert(*alias, cp);
    }

    /// Removes an alias from all existing code point `cp`.
    ///
    /// # Detail
    ///
    /// The alias must have been added for a code point.
    ///
    /// # Arguments
    ///
    /// * `alias` The `char` to remove from an existing code point
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    /// let cp_of_letter_c = language.get_cp(&'c');
    /// language.add_cp_alias(cp_of_letter_c, &'Ć');    
    ///
    /// assert!(language.is_letter(&'Ć'));
    ///
    /// language.del_cp_alias(&'Ć');
    ///
    /// assert!(!language.is_letter(&'Ć'));
    /// ```
    ///
    pub fn del_cp_alias(&mut self, alias: &char) {
        assert!(self.char_to_cp.contains_key(alias));

        self.char_to_cp.retain(|&k, _| k != *alias);
    }

    /// Removes the greatest code point (`self.max_cp()`) from the upper and lower alphabets. Also removes
    /// all aliases of that letter, and decrements the alphabet length. Returns the value of the removed code point.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    ///
    /// language.pop_cp();
    ///
    /// assert!(!language.is_letter(&'Z'));
    /// assert!(!language.is_letter(&'z'));
    /// ```
    ///
    pub fn pop_cp(&mut self) -> i16 {
        let cp = self.max_cp();

        assert!(cp >= 0);

        self.upper_alphabet.pop();
        self.lower_alphabet.pop();
        self.char_to_cp.retain(|_, v| *v != cp);
        self.alphabet_len -= 1;

        cp
    }

    /// Removes all existing code points from the language, clearing the upper and lower alphabets,
    /// and all aliases. The alphabet length is set to zero.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    ///
    /// language.clear_all_cp();
    ///
    /// for i in 0..upper_alph.len() {
    ///     assert!(!language.is_letter(&upper_alph.chars().nth(i).unwrap()));
    ///     assert!(!language.is_letter(&lower_alph.chars().nth(i).unwrap()));
    /// }
    /// ```
    ///
    pub fn clear_all_cp(&mut self) {
        self.alphabet_len = 0;
        self.upper_alphabet.clear();
        self.lower_alphabet.clear();
        self.char_to_cp.clear();
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
    /*                          Single letter conversions                         */
    /* -------------------------------------------------------------------------- */
    /// Returns the code point from a letter. The provided character must be a letter
    /// as defined by `Language::is_letter()`.
    ///
    /// # Arguments
    ///
    /// * `letter` The character to convert. Can be any uppercase,lowercase or alias configured for the `Language`
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    ///
    /// assert_eq!(language.get_cp(&'A'), 0);
    /// assert_eq!(language.get_cp(&'z'), 25);
    /// ```
    ///
    pub fn get_cp(&self, letter: &char) -> i16 {
        assert!(self.is_letter(letter));

        self.char_to_cp[letter]
    }

    /// Not to be confused with `Language::edit_cp()`. Returns a character that maintains the current
    /// case (upper/lower) as defined by the uppercase and lowercase alphabets, but has an updated
    /// code point i.e. the letter is first identified as uppercase or lowercase, then that alphabet
    /// is indexed by the `new_cp` to find the resulting letter
    ///
    /// # Arguments
    ///
    /// * `old_letter` The original letter. Used to determine upper/lower case
    /// * `new_cp` The new code point to apply to the letter
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    ///
    /// let my_letter = 'A';
    /// let my_cp = language.get_cp(&my_letter);
    /// let my_new_cp = my_cp + 1;
    ///
    /// assert_eq!(language.update_cp(&my_letter, my_new_cp), 'B');
    /// assert_eq!(language.update_cp(&language.to_lower(&my_letter), my_new_cp), 'b');
    /// ```
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

    /// Converts a valid code point to its uppercase equivalent
    ///
    /// # Arguments
    ///
    /// * `cp` The valid code point to convert
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    /// assert_eq!(language.cp_to_upper(0), 'A');
    /// assert_eq!(language.cp_to_upper(1), 'B');
    /// assert_eq!(language.cp_to_upper(25), 'Z');
    /// ```
    ///
    pub fn cp_to_upper(&self, cp: i16) -> char {
        assert!(self.valid_cp(cp));

        self.upper_alphabet[cp as usize]
    }

    /// Converts a valid code point to its lowercase equivalent
    ///
    /// # Arguments
    ///
    /// * `cp` The valid code point to convert
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    /// assert_eq!(language.cp_to_lower(0), 'a');
    /// assert_eq!(language.cp_to_lower(1), 'b');
    /// assert_eq!(language.cp_to_lower(25), 'z');
    /// ```
    ///
    pub fn cp_to_lower(&self, cp: i16) -> char {
        assert!(self.valid_cp(cp));

        self.lower_alphabet[cp as usize]
    }

    /* -------------------------------------------------------------------------- */
    /*                             String conversions                             */
    /* -------------------------------------------------------------------------- */
    /// Converts a borrowed slice of code points to a string. This calls cp_to_upper() on each
    /// element of the slice.
    ///
    /// # Arguments
    ///
    /// * `arr` The slice to convert
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    /// let my_vec = vec![7, 4, 11, 11, 14];
    /// assert_eq!(language.vec_to_string(&my_vec), String::from("HELLO"));
    /// ```
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
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    /// let my_msg = "hello";
    /// assert_eq!(language.string_to_vec(&my_msg), vec![7, 4, 11, 11, 14]);
    /// ```
    ///
    pub fn string_to_vec(&self, string: &str) -> Vec<i16> {
        string
            .chars()
            .filter(|i| self.is_letter(i))
            .map(|i| self.get_cp(&i))
            .collect()
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

    /* -------------------------------------------------------------------------- */
    /*                              Char conversions                              */
    /* -------------------------------------------------------------------------- */
    /// Converts a letter to its lowercase equivalent, or if `letter` is punctuation,
    /// leaves the letter as it is.
    ///
    /// # Arguments
    ///
    /// * `letter` The letter to convert
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    /// assert_eq!(language.to_lower(&'X'), 'x');
    /// assert_eq!(language.to_lower(&'x'), 'x');
    /// assert_eq!(language.to_lower(&'?'), '?');
    /// ```
    ///
    pub fn to_lower(&self, letter: &char) -> char {
        if self.is_upper(letter) {
            self.cp_to_lower(self.get_cp(letter))
        } else {
            *letter
        }
    }

    /// Converts a letter to its uppercase equivalent, or if `letter` is punctuation,
    /// leaves the letter as it is.
    ///
    /// # Arguments
    ///
    /// * `letter` The letter to convert
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    /// assert_eq!(language.to_upper(&'X'), 'X');
    /// assert_eq!(language.to_upper(&'x'), 'X');
    /// assert_eq!(language.to_upper(&'?'), '?');
    /// ```
    ///
    pub fn to_upper(&self, letter: &char) -> char {
        if self.is_lower(letter) {
            self.cp_to_upper(self.get_cp(letter))
        } else {
            *letter
        }
    }

    /* -------------------------------------------------------------------------- */
    /*                                 Validation                                 */
    /* -------------------------------------------------------------------------- */
    /// Checks whether `letter` is contained in the uppercase, lowercase, or alias collections.
    ///
    /// # Arguments
    ///
    /// * `letter` The letter to check
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    /// assert!(language.is_letter(&'X'));
    /// assert!(language.is_letter(&'x'));
    /// assert!(!language.is_letter(&'?'));
    /// ```
    ///
    pub fn is_letter(&self, letter: &char) -> bool {
        self.char_to_cp.contains_key(&letter)
    }

    /// Checks whether `letter` is NOT contained in the uppercase, lowercase, or alias collections.
    /// Equivalent to `!self.is_letter(letter)`.
    ///
    /// # Arguments
    ///
    /// * `letter` The letter to check
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    /// assert!(!language.is_punct(&'X'));
    /// assert!(!language.is_punct(&'x'));
    /// assert!(language.is_punct(&'?'));
    /// ```
    ///
    pub fn is_punct(&self, letter: &char) -> bool {
        !self.is_letter(letter)
    }

    /// Checks whether `letter` is contained in the uppercase alphabet.
    ///
    /// # Arguments
    ///
    /// * `letter` The letter to check
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    /// assert!(language.is_upper(&'X'));
    /// assert!(!language.is_upper(&'x'));
    /// assert!(!language.is_upper(&'?'));
    /// ```
    ///
    pub fn is_upper(&self, letter: &char) -> bool {
        self.upper_alphabet.contains(letter)
    }

    /// Checks whether `letter` is contained in the lowercase alphabet.
    ///
    /// # Arguments
    ///
    /// * `letter` The letter to check
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    /// assert!(!language.is_lower(&'X'));
    /// assert!(language.is_lower(&'x'));
    /// assert!(!language.is_lower(&'?'));
    /// ```
    ///
    pub fn is_lower(&self, letter: &char) -> bool {
        self.lower_alphabet.contains(letter)
    }

    /// Checks whether `cp` is within the valid range `0..max_cp()`.
    ///
    /// # Arguments
    ///
    /// * `cp` The code point to check
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    /// assert!(language.valid_cp(0));
    /// assert!(language.valid_cp(25));
    /// assert!(!language.valid_cp(-1));
    /// assert!(!language.valid_cp(26));
    /// ```
    ///
    pub fn valid_cp(&self, cp: i16) -> bool {
        0 <= cp && cp <= self.max_cp()
    }

    /// Returns the maximum (inclusive) value of a code point. One less than the alphabet length.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    /// assert_eq!(language.max_cp(), 25);
    /// ```
    ///
    pub fn max_cp(&self) -> i16 {
        self.cp_count() - 1
    }

    /// Returns the maximum (exclusive) value of a code point. Equal to the alphabet length.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    /// assert_eq!(language.cp_count(), 26_i16);
    /// ```
    ///
    pub fn cp_count(&self) -> i16 {
        self.alphabet_len() as i16
    }

    /// Returns the length of the alphabet.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use classic_crypto::lang::Language;
    /// let mut language = Language::default();
    /// let upper_alph = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    /// let lower_alph = upper_alph.to_lowercase();
    ///
    /// for i in 0..upper_alph.len() {
    ///     language.new_cp(
    ///         &upper_alph.chars().nth(i).unwrap(),
    ///         &lower_alph.chars().nth(i).unwrap()
    ///     );
    /// }
    /// assert_eq!(language.alphabet_len(), 26 as usize);
    /// ```
    ///
    pub fn alphabet_len(&self) -> usize {
        self.alphabet_len as usize
    }
}
