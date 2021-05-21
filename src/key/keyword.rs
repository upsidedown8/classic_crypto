use rand::Rng;

use crate::{
    error::Result,
    key::{IdentityKey, IoKey, Key, KeyInfo, StatefulKey},
    lang::Language,
    util,
};

/// Represents a Keyword (See Vigenere ciphers)
///
pub struct Keyword {
    value: Vec<i16>,
    info: KeyInfo,
}

impl Keyword {
    /// Returns the length of the keyword
    ///
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Is the keyword empty?
    ///
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Get the value of the keyword at `idx` as a code point
    ///
    /// # Arguments
    ///
    /// * `idx` The index to get
    ///
    #[inline(always)]
    pub fn at(&self, idx: usize) -> i16 {
        debug_assert!(idx < self.value.len());
        self.value[idx]
    }

    /// Finds the indexes of the items in the keyword
    ///
    pub fn find_order(&self) -> Vec<usize> {
        util::find_order(&self.value)
    }
}

impl Key<&str> for Keyword {
    fn new(language: &mut Language, arg: &str) -> Result<Box<Self>> {
        let mut result = Box::new(Self::identity(language));
        result.set(language, arg)?;
        Ok(result)
    }
    fn set(&mut self, language: &mut Language, arg: &str) -> Result<()> {
        self.value = language.string_to_vec(arg);
        // ensure not empty
        if self.value.is_empty() {
            self.value.push(0);
        }
        Ok(())
    }
}
impl Key<&[i16]> for Keyword {
    fn new(_language: &mut Language, arg: &[i16]) -> Result<Box<Self>> {
        Ok(Box::new(Self {
            value: Vec::from(arg),
            info: KeyInfo::default(),
        }))
    }
    fn set(&mut self, _language: &mut Language, arg: &[i16]) -> Result<()> {
        self.value = Vec::from(arg);
        // ensure not empty
        if self.value.is_empty() {
            self.value.push(0);
        }
        Ok(())
    }
}

impl IdentityKey for Keyword {
    fn identity(_language: &mut Language) -> Self {
        Self {
            value: vec![0],
            info: KeyInfo::default(),
        }
    }
}

impl StatefulKey for Keyword {
    fn reset(&mut self, _language: &mut Language) {
        self.value = vec![0];
    }
    fn to_string(&self, language: &mut Language) -> String {
        language.vec_to_string(&self.value)
    }
    fn randomize(&mut self, language: &mut Language) {
        let length = rand::thread_rng().gen_range(3..12);
        self.value.resize(length, 0);
        util::fill_random_array(&mut self.value, language.cp_count());
    }
}

impl IoKey for Keyword {
    fn set_key_str(&mut self, language: &mut Language, arg: &str) -> Result<()> {
        self.set(language, arg)
    }
    fn key_info(&self) -> &KeyInfo {
        &self.info
    }
    fn key_info_mut(&mut self) -> &mut KeyInfo {
        &mut self.info
    }
}
