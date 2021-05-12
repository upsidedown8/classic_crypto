extern crate serde;
use std::collections::HashMap;

use crate::error::{Error, Result};
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

    /// The expected value for index of coincedence
    ///
    pub expected_ioc: f64,

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
    /// * `upper_substitutions` Map one uppercase letter (not in the alphabet) to another letter within the alphabet
    /// * `lower_substitutions` Map one lowercase letter (not in the alphabet) to another letter within the alphabet
    /// * `scoring_sub_table` Map (0..length) to (0..principle_length)
    ///
    pub fn new(
        upper: String,
        lower: String,
        upper_substitutions: Vec<String>,
        lower_substitutions: Vec<String>,
        scoring_sub_table: Vec<i16>,
    ) -> Result<LangAlphabet> {
        let mut result = LangAlphabet {
            upper,
            lower,
            lower_substitutions,
            upper_substitutions,
            scoring_sub_table,
            char_to_cp: HashMap::new(),
            expected_ioc: 0.0,
        };

        result.init()?;

        Ok(result)
    }

    /// Checks the deserialized [`LangAlphabet`] and fills in unserialized fields
    ///
    pub fn init(&mut self) -> Result<()> {
        self.char_to_cp.clear();

        let upper_len = self.upper.chars().count();
        let lower_len = self.lower.chars().count();

        if upper_len != lower_len {
            Err(Error::AlphabetLenDifference {
                upper_len,
                lower_len,
            })
        } else if upper_len != self.scoring_sub_table.len() {
            Err(Error::ScoringSubTableLen {
                alphabet_len: upper_len,
                table_len: self.scoring_sub_table.len(),
            })
        } else if self.upper.chars().count() > crate::lang::language::MAX_ALPHABET_LEN {
            Err(Error::MaxAlphabetLenExceeded {
                alphabet_len: upper_len,
            })
        } else if !util::is_unique(&self.upper) {
            Err(Error::RepeatCharUpperAlph {
                upper: self.upper.clone(),
            })
        } else if !util::is_unique(&self.lower) {
            Err(Error::RepeatCharLowerAlph {
                lower: self.lower.clone(),
            })
        } else if self
            .lower_substitutions
            .iter()
            .any(|x| x.chars().count() != 2)
            || self
                .upper_substitutions
                .iter()
                .any(|x| x.chars().count() != 2)
        {
            Err(Error::SubstitutionsNotPairs {
                subs: self.upper_substitutions.clone(),
            })
        } else {
            let lower_is_unique = util::is_unique(
                self.lower_substitutions
                    .iter()
                    .fold(String::new(), |acc, x| acc + x)
                    .as_str(),
            );
            let upper_is_unique = util::is_unique(
                self.upper_substitutions
                    .iter()
                    .fold(String::new(), |acc, x| acc + x)
                    .as_str(),
            );

            if !upper_is_unique {
                Err(Error::SubstitutionsNotUnique {
                    subs: self.upper_substitutions.clone(),
                })
            } else if !lower_is_unique {
                Err(Error::SubstitutionsNotUnique {
                    subs: self.lower_substitutions.clone(),
                })
            } else {
                let invalid_lower = self.lower_substitutions.iter().any(|sub| {
                    let mut iter = sub.chars();

                    if let Some(char2) = iter.nth(1) {
                        // check whether the target char exists in the alphabet
                        !self.lower.contains(char2)
                    } else {
                        false
                    }
                });
                let invalid_upper = self.upper_substitutions.iter().any(|sub| {
                    let mut iter = sub.chars();

                    if let Some(char2) = iter.nth(1) {
                        // check whether the target char exists in the alphabet
                        !self.upper.contains(char2)
                    } else {
                        false
                    }
                });

                if invalid_lower {
                    Err(Error::InvalidCharsInSubstitutions {
                        subs: self.lower_substitutions.clone(),
                    })
                } else if invalid_upper {
                    Err(Error::InvalidCharsInSubstitutions {
                        subs: self.upper_substitutions.clone(),
                    })
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
    }

    /// Returns the length of this alphabet
    ///
    pub fn length(&self) -> usize {
        self.upper.chars().count()
    }
}
