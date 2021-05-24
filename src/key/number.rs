use crate::{
    error::{Error, Result},
    key::{IdentityKey, IoKey, Key, KeyInfo, StatefulKey},
    lang::Language,
};

/// Represents a Number (See Affine Cipher)
///
pub struct Number {
    value: i16,
    legal_values: Vec<i16>,
    info: KeyInfo,
    desc: String,
}

impl Number {
    /// Gets the value
    ///
    pub fn get(&self) -> i16 {
        self.value
    }

    /// Sets the range of legal values for the number.
    ///
    /// # Arguments
    ///
    /// * `legal_values` The legal range of numbers
    ///
    pub fn set_legal_values(&mut self, range: Vec<i16>) {
        self.legal_values = range.clone();
        self.desc = format!("<integer: {:?}>", range);
    }

    fn parse(arg: &str) -> Result<i16> {
        let num = arg.parse::<i16>().map_err(|_| Error::InvalidKeyFmt {
            expected: "An integer".to_string(),
            actual: arg.to_string(),
        })?;
        Ok(num)
    }
    fn check_val(&self, arg: i16) -> Result<i16> {
        if self.legal_values.contains(&arg) {
            Ok(arg)
        } else {
            Err(Error::InvalidKeyFmt {
                expected: format!("Number should be in range {:?}", self.legal_values),
                actual: format!("{}", arg),
            })
        }
    }
}

impl Key<i16> for Number {
    fn new(language: &mut Language, arg: i16) -> Result<Box<Self>> {
        let result = Number {
            value: arg,
            legal_values: (0..language.cp_count()).collect(),
            info: KeyInfo::default(),
            desc: "<integer>".to_string(),
        };

        result.check_val(arg)?;

        Ok(Box::new(result))
    }
    fn set(&mut self, _language: &mut Language, arg: i16) -> Result<()> {
        self.value = self.check_val(arg)?;
        Ok(())
    }
}
impl Key<&str> for Number {
    fn new(language: &mut Language, arg: &str) -> Result<Box<Self>> {
        let num = Self::parse(arg)?;
        Number::new(language, num)
    }
    fn set(&mut self, language: &mut Language, arg: &str) -> Result<()> {
        let num = Self::parse(arg)?;
        self.set(language, num)
    }
}

impl IdentityKey for Number {
    fn identity(language: &mut Language) -> Self {
        Self {
            value: 0,
            legal_values: (0..language.cp_count()).collect(),
            info: KeyInfo::default(),
            desc: "<integer>".to_string(),
        }
    }
}

impl StatefulKey for Number {
    fn reset(&mut self, _language: &mut Language) {
        self.value = self.legal_values[0];
    }
    fn to_string(&self, _language: &mut Language) -> String {
        format!("{}", self.value)
    }
    fn randomize(&mut self, _language: &mut Language) {
        self.value = self.legal_values[fastrand::usize(0..self.legal_values.len())];
    }
}

impl IoKey for Number {
    fn set_key_str(&mut self, language: &mut Language, arg: &str) -> Result<()> {
        self.set(language, arg)
    }
    fn info(&self) -> &KeyInfo {
        &self.info
    }
    fn info_mut(&mut self) -> &mut KeyInfo {
        &mut self.info
    }
    fn desc(&self) -> String {
        self.desc.clone()
    }
}
