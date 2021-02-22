use std::collections::HashMap;

extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Language {
    /* Current language details */
    pub name: String,

    /* characters -> code points */
    char_to_cp: HashMap<char, i16>,

    /* alphabets */
    upper_alphabet: Vec<char>,
    lower_alphabet: Vec<char>,
    alphabet_len: i16,

    /* diacritics -> standard letter(s) */
    substitution_table: HashMap<char, String>,
}

impl Language {
    pub fn new() -> Language {
        Language {
            name: String::new(),
            char_to_cp: HashMap::new(),
            upper_alphabet: Vec::new(),
            lower_alphabet: Vec::new(),
            alphabet_len: 0,
            substitution_table: HashMap::new(),
        }
    }

    /* Alphabet setup */
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
    pub fn edit_cp(&mut self, cp: i16, upper: &char, lower: &char) {
        assert!(self.valid_cp(cp));

        self.char_to_cp.retain(|_, v| *v != cp);
        self.char_to_cp.insert(*upper, cp);
        self.char_to_cp.insert(*lower, cp);
        self.upper_alphabet[cp as usize] = *upper;
        self.lower_alphabet[cp as usize] = *lower;
    }
    pub fn add_cp_alias(&mut self, cp: i16, alias: &char) {
        assert!(!self.char_to_cp.contains_key(alias));
        assert!(self.valid_cp(cp));

        self.char_to_cp.insert(*alias, cp);
    }
    pub fn pop_cp(&mut self) -> i16 {
        let cp = self.max_cp();

        assert!(cp >= 0);

        self.upper_alphabet.pop();
        self.lower_alphabet.pop();
        self.char_to_cp.retain(|_, v| *v != cp);
        self.alphabet_len -= 1;

        cp
    }
    pub fn clear_all_cp(&mut self) {
        self.alphabet_len = 0;
        self.upper_alphabet.clear();
        self.lower_alphabet.clear();
        self.char_to_cp.clear();
    }

    /* Substitution table setup */
    pub fn add_substitution(&mut self, from: &char, to: &String) {
        assert!(!self.substitution_table.contains_key(from));

        self.substitution_table.insert(*from, to.clone());
    }
    pub fn del_substitution(&mut self, from: &char) {
        self.substitution_table.remove(from);
    }
    pub fn clear_substitutions(&mut self) {
        self.substitution_table.clear();
    }

    /* Single letter conversions */
    pub fn get_cp(&self, letter: &char) -> i16 {
        assert!(self.is_letter(letter));

        self.char_to_cp[letter]
    }
    pub fn update_cp(&self, old_letter: &char, new_cp: i16) -> char {
        assert!(self.valid_cp(new_cp));
        assert!(self.is_letter(old_letter));

        if self.is_upper(old_letter) {
            self.cp_to_upper(new_cp)
        } else {
            self.cp_to_lower(new_cp)
        }
    }
    pub fn cp_to_upper(&self, cp: i16) -> char {
        self.upper_alphabet[cp as usize]
    }
    pub fn cp_to_lower(&self, cp: i16) -> char {
        self.lower_alphabet[cp as usize]
    }

    /* String conversions */
    pub fn vec_to_string(&self, arr: &Vec<i16>) -> String {
        arr.iter().map(|&i| self.cp_to_upper(i)).collect()
    }
    pub fn str_to_vec(&self, string: &str) -> Vec<i16> {
        self.string_to_vec(&String::from(string))
    }
    pub fn string_to_vec(&self, string: &String) -> Vec<i16> {
        string
            .chars()
            .filter(|i| self.is_letter(i))
            .map(|i| self.get_cp(&i))
            .collect()
    }

    /* Substitution table */
    pub fn substitute_string(&self, string: &String) -> String {
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

    /* Char conversions */
    pub fn to_lower(&self, letter: &char) -> char {
        if self.is_upper(letter) {
            self.cp_to_lower(self.get_cp(letter))
        } else {
            *letter
        }
    }
    pub fn to_upper(&self, letter: &char) -> char {
        if self.is_lower(letter) {
            self.cp_to_upper(self.get_cp(letter))
        } else {
            *letter
        }
    }

    /* Validation */
    pub fn is_letter(&self, letter: &char) -> bool {
        self.char_to_cp.contains_key(&letter)
    }
    pub fn is_punct(&self, letter: &char) -> bool {
        !self.is_letter(letter)
    }
    pub fn is_upper(&self, letter: &char) -> bool {
        self.is_letter(letter) && self.upper_alphabet.contains(letter)
    }
    pub fn is_lower(&self, letter: &char) -> bool {
        self.is_letter(letter) && self.lower_alphabet.contains(letter)
    }
    pub fn valid_cp(&self, cp: i16) -> bool {
        0 <= cp && cp <= self.max_cp()
    }
    pub fn max_cp(&self) -> i16 {
        self.cp_count() - 1
    }
    pub fn cp_count(&self) -> i16 {
        self.alphabet_len() as i16
    }
    pub fn alphabet_len(&self) -> usize {
        self.alphabet_len as usize
    }
}
