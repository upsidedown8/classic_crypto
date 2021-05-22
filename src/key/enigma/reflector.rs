use crate::{
    error::{Error, Result},
    key::{IoKey, Key, KeyInfo, StatefulKey},
    lang::Language,
};

// Wiring details from: https://en.wikipedia.org/wiki/Enigma_rotor_details#Rotor_wiring_tables

/// Collection of all Enigma Reflector types from the M3 and M4 Enigma
/// machines. (See Enigma cipher)
///
#[derive(Clone, Copy)]
pub enum ReflectorType {
    A = 0,
    B = 1,
    C = 2,
    BThin = 3, // M4
    CThin = 4, // M4
}

impl From<usize> for ReflectorType {
    fn from(arg: usize) -> Self {
        match arg {
            0 => ReflectorType::A,
            1 => ReflectorType::B,
            2 => ReflectorType::C,
            3 => ReflectorType::BThin,
            _ => ReflectorType::CThin,
        }
    }
}

#[rustfmt::skip]
const WIRINGS: [[i16; 26]; 5] = [
    [ 4, 9, 12, 25, 0, 11, 24, 23, 21, 1, 22, 5, 2, 17, 16, 20, 14, 13, 19, 18, 15, 8, 10, 7, 6, 3, ],
    [ 24, 17, 20, 7, 16, 18, 11, 3, 15, 23, 13, 6, 14, 10, 12, 8, 4, 1, 5, 25, 2, 22, 21, 9, 0, 19, ],
    [ 5, 21, 15, 9, 8, 0, 14, 24, 4, 3, 17, 25, 23, 22, 6, 2, 19, 10, 20, 16, 18, 1, 13, 12, 7, 11, ],
    [ 4, 13, 10, 16, 0, 20, 24, 22, 9, 8, 2, 14, 15, 1, 11, 12, 3, 23, 25, 21, 5, 19, 7, 17, 6, 18, ],
    [ 17, 3, 14, 1, 9, 13, 19, 10, 21, 4, 7, 12, 11, 5, 2, 22, 25, 0, 23, 6, 24, 8, 15, 18, 20, 16, ],
];

/// Represents an Enigma Reflector (See Enigma cipher)
///
#[derive(Clone)]
pub struct Reflector {
    /// The current internal wiring of the reflector
    wiring_type: ReflectorType,
    info: KeyInfo,
}

impl Reflector {
    /// Sends a letter through the wiring of the reflector and returns
    /// the output
    ///
    /// # Arguments
    ///
    /// * `letter` The letter to input to the reflector
    ///
    pub fn input(&self, letter: i16) -> i16 {
        WIRINGS[self.wiring_type as usize][letter as usize]
    }
}

impl Key<ReflectorType> for Reflector {
    fn new(_language: &mut Language, arg: ReflectorType) -> Result<Box<Self>> {
        Ok(Box::new(Reflector {
            wiring_type: arg,
            info: KeyInfo::default(),
        }))
    }
    fn set(&mut self, _language: &mut Language, arg: ReflectorType) -> Result<()> {
        self.wiring_type = arg;
        Ok(())
    }
}
impl Key<&str> for Reflector {
    fn new(language: &mut Language, arg: &str) -> Result<Box<Self>> {
        let mut result = Reflector::new(language, ReflectorType::B)?;
        result.set(language, arg)?;
        Ok(result)
    }
    fn set(&mut self, _language: &mut Language, arg: &str) -> Result<()> {
        match arg.to_lowercase().as_str() {
            "a" => {
                self.wiring_type = ReflectorType::A;
                Ok(())
            }
            "b" => {
                self.wiring_type = ReflectorType::B;
                Ok(())
            }
            "c" => {
                self.wiring_type = ReflectorType::C;
                Ok(())
            }
            "bthin" => {
                self.wiring_type = ReflectorType::BThin;
                Ok(())
            }
            "cthin" => {
                self.wiring_type = ReflectorType::CThin;
                Ok(())
            }
            _ => Err(Error::InvalidKeyFmt {
                expected: "One of [a, b, c, bthin, cthin]".to_string(),
                actual: arg.to_string(),
            }),
        }
    }
}

impl StatefulKey for Reflector {
    fn reset(&mut self, _language: &mut Language) {
        self.wiring_type = ReflectorType::B;
    }
    fn to_string(&self, _language: &mut Language) -> String {
        match self.wiring_type {
            ReflectorType::A => "A",
            ReflectorType::B => "B",
            ReflectorType::C => "C",
            ReflectorType::BThin => "B thin",
            ReflectorType::CThin => "C thin",
        }
        .to_string()
    }
    fn randomize(&mut self, _language: &mut Language) {
        self.wiring_type = ReflectorType::from(fastrand::usize(0..5));
    }
}

impl IoKey for Reflector {
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
