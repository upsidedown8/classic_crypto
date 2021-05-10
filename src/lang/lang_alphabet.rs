extern crate serde;
use std::collections::HashMap;

use crate::util;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LangAlphabet {
    /// The uppercase alphabet for this alphabet length.
    ///
    pub upper: String,

    /// The lowercase alphabet for this alphabet length.
    ///
    pub lower: String,

    /// The substitutions, in the form `vec!["ji", "ba", "xy"]`, to perform in order to transition from a
    /// standard alphabet with a greater number of letters. Lowercase transformations only.
    ///
    pub lower_substitutions: Vec<String>,

    /// The substitutions, in the form `vec!["JI", "BA", "XY"]`, to perform in order to transition from a
    /// standard alphabet with a greater number of letters. Uppercase transformations only.
    ///
    pub upper_substitutions: Vec<String>,

    /// Maps the items in the alphabet back to the standard alphabet, so that the standard scoring arrays
    /// can be indexed
    ///
    pub scoring_sub_table: Vec<i16>,

    /// Maps characters to code points
    ///
    #[serde(skip)]
    pub char_to_cp: HashMap<char, i16>,
}

impl LangAlphabet {
    /// Creates a new [`LangAlphabet`] instance. Each [`Language`] should have a number of [`LangAlphabet`]s
    /// for each smaller and bigger alphabet length it wants to support.
    ///
    /// # Arguments
    ///
    /// * `upper` The uppercase full alphabet
    /// * `lower` The lowercase full alphabet
    /// * `substitutions` Map one letter (not in the alphabet) to another letter within the alphabet. (Case sensative)
    ///
    pub fn new(
        upper: String,
        lower: String,
        lower_substitutions: Vec<String>,
        upper_substitutions: Vec<String>,
        scoring_sub_table: Vec<i16>,
    ) -> Result<LangAlphabet, &'static str> {
        let mut result = LangAlphabet {
            upper,
            lower,
            lower_substitutions,
            upper_substitutions,
            scoring_sub_table,
            char_to_cp: HashMap::new(),
        };

        result.init()?;

        Ok(result)
    }

    pub fn init(&mut self) -> Result<(), &'static str> {
        self.char_to_cp.clear();

        if self.upper.chars().count() != self.lower.chars().count() {
            Err("Upper and Lower alphabets must have equal length")
        } else if self.upper.chars().count() != self.scoring_sub_table.len() {
            Err("scoring_sub_table must be of equal length to the alphabet")
        } else if self.upper.chars().count() > crate::lang::language::MAX_ALPHABET_LEN {
            Err("Maximum alphabet length is 32")
        } else if !util::is_unique(&self.upper) {
            Err("Upper alphabet has repeated letters")
        } else if !util::is_unique(&self.lower) {
            Err("Lower alphabet has repeated letters")
        } else if self
            .lower_substitutions
            .iter()
            .any(|x| x.chars().count() != 2)
            || self
                .upper_substitutions
                .iter()
                .any(|x| x.chars().count() != 2)
        {
            Err("Substitutions must be pairs of letters")
        } else if !util::is_unique(
            self.lower_substitutions
                .iter()
                .fold(String::new(), |acc, x| acc + x)
                .as_str(),
        ) || !util::is_unique(
            self.upper_substitutions
                .iter()
                .fold(String::new(), |acc, x| acc + x)
                .as_str(),
        ) {
            Err("Substitutions must be unique")
        } else {
            let any_invalid_letters = self.lower_substitutions.iter().any(|sub| {
                let mut iter = sub.chars();

                if let Some(char2) = iter.nth(1) {
                    // check whether the target char exists in the alphabet
                    !self.lower.contains(char2)
                } else {
                    false
                }
            }) || self.upper_substitutions.iter().any(|sub| {
                let mut iter = sub.chars();

                if let Some(char2) = iter.nth(1) {
                    // check whether the target char exists in the alphabet
                    !self.upper.contains(char2)
                } else {
                    false
                }
            });

            if any_invalid_letters {
                Err("Some letters in the substitutions were not present in the alphabet")
            } else {
                // insert all upper & lower chars
                for i in 0..self.length() {
                    self.char_to_cp
                        .insert(self.upper.chars().nth(i).unwrap(), i as i16);
                    self.char_to_cp
                        .insert(self.lower.chars().nth(i).unwrap(), i as i16);
                }

                // insert substitutions
                for &list in &[&self.lower_substitutions, &self.upper_substitutions] {
                    for sub in list {
                        let mut iter = sub.chars();

                        // find the pair of chars, then add both upper & lower alternatives to char_to_cp
                        if let Some(char1) = iter.next() {
                            if let Some(char2) = iter.next() {
                                // add the pair
                                self.char_to_cp.insert(char1, self.char_to_cp[&char2]);
                            }
                        }
                    }
                }

                Ok(())
            }
        }
    }

    /// Returns the length of this alphabet
    ///
    pub fn length(&self) -> usize {
        self.upper.chars().count()
    }
}
