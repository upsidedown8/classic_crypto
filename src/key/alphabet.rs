use crate::{
    error::Result,
    key::{IdentityKey, IoKey, Key, KeyInfo, StatefulKey},
    lang::Language,
    util,
};

/// Represents a substitution alphabet (See Simple Substitution cipher)
///
pub struct Alphabet {
    value: Vec<i16>,
    inverse: Vec<i16>,
    info: KeyInfo,
}

impl Alphabet {
    /// Calculates the inverse of `value`, then assigns to `inverse`
    ///  
    fn update_inverse(&mut self) {
        self.inverse = util::invert(&self.value);
    }

    /// Encrypts `letter` using the alphabet
    ///
    /// # Arguments
    ///
    /// * `letter` The letter to encrypt
    ///
    #[inline(always)]
    pub fn encrypt(&self, letter: i16) -> i16 {
        self.value[letter as usize]
    }

    /// Decrypts `letter` using the alphabet
    ///
    /// # Arguments
    ///
    /// * `letter` The letter to decrypt
    ///
    #[inline(always)]
    pub fn decrypt(&self, letter: i16) -> i16 {
        self.inverse[letter as usize]
    }

    /// Gets the value of the substitution as a slice
    ///
    pub fn as_slice(&self) -> &[i16] {
        &self.value
    }
}

impl Key<&str> for Alphabet {
    fn new(language: &mut Language, arg: &str) -> Result<Box<Self>> {
        let alphabet: Vec<i16> = language.string_to_vec(arg);
        let my_value = util::fill_alphabet_from_start(&alphabet, language.alphabet_len());
        let my_inverse = util::invert(&my_value);
        Ok(Box::new(Self {
            value: my_value,
            inverse: my_inverse,
            info: KeyInfo::default(),
        }))
    }
    fn set(&mut self, language: &mut Language, arg: &str) -> Result<()> {
        let alphabet: Vec<i16> = language.string_to_vec(arg);
        self.value = util::fill_alphabet_from_start(&alphabet, language.alphabet_len());
        self.update_inverse();
        Ok(())
    }
}
impl Key<&[i16]> for Alphabet {
    fn new(language: &mut Language, arg: &[i16]) -> Result<Box<Self>> {
        let my_value = util::fill_alphabet_from_start(arg, language.alphabet_len());
        let my_inverse = util::invert(&my_value);
        Ok(Box::new(Self {
            value: my_value,
            inverse: my_inverse,
            info: KeyInfo::default(),
        }))
    }
    fn set(&mut self, language: &mut Language, arg: &[i16]) -> Result<()> {
        self.value = util::fill_alphabet_from_start(arg, language.alphabet_len());
        self.update_inverse();
        Ok(())
    }
}

impl IdentityKey for Alphabet {
    fn identity(language: &mut Language) -> Self {
        Self {
            value: (0..language.cp_count()).collect(),
            inverse: (0..language.cp_count()).collect(),
            info: KeyInfo::default(),
        }
    }
}

impl StatefulKey for Alphabet {
    fn reset(&mut self, language: &mut Language) {
        self.value = vec![0; language.alphabet_len()];
        self.update_inverse();
    }
    fn to_string(&self, language: &mut Language) -> String {
        language.vec_to_string(&self.value)
    }
    fn randomize(&mut self, _language: &mut Language) {
        util::shuffle(&mut self.value);
        self.update_inverse();
    }
}

impl IoKey for Alphabet {
    fn set_key_str(&mut self, language: &mut Language, arg: &str) -> crate::error::Result<()> {
        self.set(language, arg)
    }
    fn key_info(&self) -> &KeyInfo {
        &self.info
    }
    fn key_info_mut(&mut self) -> &mut KeyInfo {
        &mut self.info
    }
}
