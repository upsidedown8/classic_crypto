use crate::{
    error::{Error, Result},
    key::{IoKey, Key, KeyInfo, StatefulKey},
    lang::Language,
    util,
};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

// Wiring details from: https://en.wikipedia.org/wiki/Enigma_rotor_details#Rotor_wiring_tables

/// Collection of all Enigma Rotor types, from the Enigma I, M3 Army Enigma,
/// M3 & M4 Naval Enigma, and M4 R2 Enigma models. All of these Enigma models
/// can be simulated using combinations of these rotors. (See Enigma cipher)
///
#[derive(Clone, Copy)]
#[allow(clippy::clippy::upper_case_acronyms)]
pub enum RotorType {
    I = 0,
    II = 1,
    III = 2,
    IV = 3,
    V = 4,
    VI = 5,
    VII = 6,
    VIII = 7,
    Beta = 8,
    Gamma = 9,
}

impl Distribution<RotorType> for Standard {
    // Beta and Gamma rotors are generated with a seperate RNG
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> RotorType {
        match rng.gen_range(0..8) {
            0 => RotorType::I,
            1 => RotorType::II,
            2 => RotorType::III,
            3 => RotorType::IV,
            4 => RotorType::V,
            5 => RotorType::VI,
            6 => RotorType::VII,
            _ => RotorType::VIII,
        }
    }
}

#[rustfmt::skip]
const WIRINGS: [[i16; 26]; 10] = [
    [ 4, 10, 12, 5, 11, 6, 3, 16, 21, 25, 13, 19, 14, 22, 24, 7, 23, 20, 18, 15, 0, 8, 1, 17, 2, 9, ],
    [ 0, 9, 3, 10, 18, 8, 17, 20, 23, 1, 11, 7, 22, 19, 12, 2, 16, 6, 25, 13, 15, 24, 5, 21, 14, 4, ],
    [ 1, 3, 5, 7, 9, 11, 2, 15, 17, 19, 23, 21, 25, 13, 24, 4, 8, 22, 6, 0, 10, 12, 20, 18, 16, 14, ],
    [ 4, 18, 14, 21, 15, 25, 9, 0, 24, 16, 20, 8, 17, 7, 23, 11, 13, 5, 19, 6, 10, 3, 2, 12, 22, 1, ],
    [ 21, 25, 1, 17, 6, 8, 19, 24, 20, 15, 18, 3, 13, 7, 11, 23, 0, 22, 12, 9, 16, 14, 5, 4, 2, 10, ],
    [ 9, 15, 6, 21, 14, 20, 12, 5, 24, 16, 1, 4, 13, 7, 25, 17, 3, 10, 0, 18, 23, 11, 8, 2, 19, 22, ],
    [ 13, 25, 9, 7, 6, 17, 2, 23, 12, 24, 18, 22, 1, 14, 20, 5, 0, 8, 21, 11, 15, 4, 10, 16, 3, 19, ],
    [ 5, 10, 16, 7, 19, 11, 23, 14, 2, 1, 9, 18, 15, 3, 25, 17, 0, 12, 4, 22, 13, 8, 20, 24, 6, 21, ],
    [ 11, 4, 24, 9, 21, 2, 13, 8, 23, 22, 15, 1, 16, 12, 3, 17, 19, 0, 10, 25, 6, 5, 20, 7, 14, 18, ],
    [ 5, 18, 14, 10, 0, 13, 20, 4, 17, 7, 12, 1, 19, 8, 24, 2, 22, 11, 16, 15, 25, 23, 21, 6, 9, 3, ],
];
#[rustfmt::skip]
const INV_WIRINGS: [[i16; 26]; 10] = [
    [ 20, 22, 24, 6, 0, 3, 5, 15, 21, 25, 1, 4, 2, 10, 12, 19, 7, 23, 18, 11, 17, 8, 13, 16, 14, 9, ],
    [ 0, 9, 15, 2, 25, 22, 17, 11, 5, 1, 3, 10, 14, 19, 24, 20, 16, 6, 4, 13, 7, 23, 12, 8, 21, 18, ],
    [ 19, 0, 6, 1, 15, 2, 18, 3, 16, 4, 20, 5, 21, 13, 25, 7, 24, 8, 23, 9, 22, 11, 17, 10, 14, 12, ],
    [ 7, 25, 22, 21, 0, 17, 19, 13, 11, 6, 20, 15, 23, 16, 2, 4, 9, 12, 1, 18, 10, 3, 24, 14, 8, 5, ],
    [ 16, 2, 24, 11, 23, 22, 4, 13, 5, 19, 25, 14, 18, 12, 21, 9, 20, 3, 10, 6, 8, 0, 17, 15, 7, 1, ],
    [ 18, 10, 23, 16, 11, 7, 2, 13, 22, 0, 17, 21, 6, 12, 4, 1, 9, 15, 19, 24, 5, 3, 25, 20, 8, 14, ],
    [ 16, 12, 6, 24, 21, 15, 4, 3, 17, 2, 22, 19, 8, 0, 13, 20, 23, 5, 10, 25, 14, 18, 11, 7, 9, 1, ],
    [ 16, 9, 8, 13, 18, 0, 24, 3, 21, 10, 1, 5, 17, 20, 7, 12, 2, 15, 11, 4, 22, 25, 19, 6, 23, 14, ],
    [ 17, 11, 5, 14, 1, 21, 20, 23, 7, 3, 18, 0, 13, 6, 24, 10, 12, 15, 25, 16, 22, 4, 9, 8, 2, 19, ],
    [ 4, 11, 15, 25, 7, 0, 23, 9, 13, 24, 3, 17, 10, 5, 2, 19, 18, 8, 1, 12, 6, 22, 16, 21, 14, 20, ],
];
const NOTCHES: [[i16; 2]; 10] = [
    [16, 16],
    [4, 4],
    [21, 21],
    [9, 9],
    [25, 25],
    [25, 12],
    [25, 12],
    [25, 12],
    [-1, -1], // Beta rotor does not turn others
    [-1, -1], // Gamma rotor does not turn others
];

/// Represents an Enigma Rotor (See Enigma cipher)
///
#[derive(Clone)]
pub struct Rotor {
    /// The internal wiring to use
    wiring_type: RotorType,

    /// The outer position (incremental)
    pub grund: i16,

    /// The inner position (fixed)
    pub rings: i16,

    info: KeyInfo,
}

impl Rotor {
    /// Sends a letter forward or backward through the rotor wiring,
    /// depending on the value of reverse.
    ///
    /// # Arguments
    ///
    /// * `letter` The letter to send through the wires
    /// * `reverse` Whether to send the letter through backwards
    ///
    pub fn input(&self, letter: i16, reverse: bool) -> i16 {
        let offset = self.grund - self.rings;
        let rotor_idx = self.wiring_type as usize;
        let pos = util::modulo(letter + offset, 26) as usize;

        let wirings = if reverse { &INV_WIRINGS } else { &WIRINGS };

        util::modulo(wirings[rotor_idx][pos] - offset, 26)
    }

    /// Is the rotor's current outer position (grund) at the notch point / one of
    /// the notch points for that rotor?
    ///
    pub fn is_on_notch(&self) -> bool {
        NOTCHES[self.wiring_type as usize].contains(&self.grund)
    }

    /// Advance the outer position (grund) by a single step (rotates the rotor).
    ///
    pub fn step(&mut self) {
        self.grund = util::modulo(self.grund + 1, 26);
    }

    /// Reset the outer (grund) and inner (rings) positions
    ///
    pub fn reset_positions(&mut self) {
        self.grund = 0;
        self.rings = 0;
    }
}

impl Key<RotorType> for Rotor {
    fn new(_language: &mut Language, arg: RotorType) -> Result<Box<Self>> {
        Ok(Box::new(Rotor {
            wiring_type: arg,
            grund: 0,
            rings: 0,
            info: KeyInfo::default(),
        }))
    }
    fn set(&mut self, _language: &mut Language, arg: RotorType) -> Result<()> {
        self.wiring_type = arg;
        Ok(())
    }
}
impl Key<&str> for Rotor {
    fn new(language: &mut Language, arg: &str) -> Result<Box<Self>> {
        let mut result = Rotor::new(language, RotorType::I)?;
        result.set(language, arg)?;
        Ok(result)
    }
    fn set(&mut self, _language: &mut Language, arg: &str) -> Result<()> {
        match arg.to_lowercase().as_str() {
            "i" => self.wiring_type = RotorType::I,
            "ii" => self.wiring_type = RotorType::II,
            "iii" => self.wiring_type = RotorType::III,
            "iv" => self.wiring_type = RotorType::IV,
            "v" => self.wiring_type = RotorType::V,
            "vi" => self.wiring_type = RotorType::VI,
            "vii" => self.wiring_type = RotorType::VII,
            "viii" => self.wiring_type = RotorType::VIII,
            "beta" | "b" => self.wiring_type = RotorType::Beta,
            "gamma" | "g" => self.wiring_type = RotorType::Gamma,
            _ => {
                return Err(Error::InvalidKeyFmt {
                    expected: "One of [i, ii, iii, iv, v, vi, vii, viii, beta, gamma]".to_string(),
                    actual: arg.to_string(),
                })
            }
        };
        Ok(())
    }
}

impl StatefulKey for Rotor {
    fn reset(&mut self, _language: &mut Language) {
        self.wiring_type = RotorType::I;
        self.reset_positions();
    }
    fn to_string(&self, _language: &mut Language) -> String {
        format!(
            "rotor:{}, grund:{}, rings:{}",
            match self.wiring_type {
                RotorType::I => "I",
                RotorType::II => "II",
                RotorType::III => "III",
                RotorType::IV => "IV",
                RotorType::V => "V",
                RotorType::VI => "VI",
                RotorType::VII => "VII",
                RotorType::VIII => "VIII",
                RotorType::Beta => "Beta",
                RotorType::Gamma => "Gamma",
            },
            self.grund,
            self.rings
        )
    }
    fn randomize(&mut self, _language: &mut Language) {
        let mut rng = rand::thread_rng();
        self.wiring_type = rng.gen();
        self.grund = rng.gen_range(0..26);
        self.rings = rng.gen_range(0..26);
    }
}

impl IoKey for Rotor {
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
