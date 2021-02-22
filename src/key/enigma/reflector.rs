use crate::{key::{Key, KeyFrom, SetKey, StatefulKey}, lang::Language};
use rand::{
    distributions::{Distribution, Standard},
    Rng
};

#[derive(Clone, Copy)]
pub enum ReflectorType {
    A = 0,
    B = 1,
    C = 2,
    BThin = 3,
    CThin = 4
}

impl Distribution<ReflectorType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ReflectorType {
        match rng.gen_range(0..5) {
            0 => ReflectorType::A,
            1 => ReflectorType::B,
            2 => ReflectorType::C,
            3 => ReflectorType::BThin,
            _ => ReflectorType::CThin
        }
    }
}

const WIRINGS: [[i16; 26]; 5] = [
    [  4,  9, 12, 25,  0, 11, 24, 23, 21,  1, 22,  5,  2, 17, 16, 20, 14, 13, 19, 18, 15,  8, 10,  7,  6,  3 ],
    [ 24, 17, 20,  7, 16, 18, 11,  3, 15, 23, 13,  6, 14, 10, 12,  8,  4,  1,  5, 25,  2, 22, 21,  9,  0, 19 ],
    [  5, 21, 15,  9,  8,  0, 14, 24,  4,  3, 17, 25, 23, 22,  6,  2, 19, 10, 20, 16, 18,  1, 13, 12,  7, 11 ],
    [  4, 13, 10, 16,  0, 20, 24, 22,  9,  8,  2, 14, 15,  1, 11, 12,  3, 23, 25, 21,  5, 19,  7, 17,  6, 18 ],
    [ 17,  3, 14,  1,  9, 13, 19, 10, 21,  4,  7, 12, 11,  5,  2, 22, 25,  0, 23,  6, 24,  8, 15, 18, 20, 16 ]
];

#[derive(Clone, Copy)]
pub struct Reflector {
    wiring_type: ReflectorType
}

impl Reflector {
    pub fn input(&self, letter: i16) -> i16 {
        WIRINGS[self.wiring_type as usize][letter as usize]
    }
}

impl KeyFrom<ReflectorType> for Reflector {
    fn create_from(_language: &Language, wiring: ReflectorType) -> Reflector {
        Reflector {
            wiring_type: wiring
        }
    }
}

impl SetKey<ReflectorType> for Reflector {
    fn set_key(&mut self, _language: &Language, wiring: ReflectorType) {
        self.wiring_type = wiring;
    }
}

impl Key for Reflector {
    fn to_string(&self, _language: &Language) -> String {
        format!("Reflector:{}", match self.wiring_type {
            ReflectorType::A => "A",
            ReflectorType::B => "B",
            ReflectorType::C => "C",
            ReflectorType::BThin => "B thin",
            ReflectorType::CThin => "C thin"
        })
    }

    fn new(_language: &Language) -> Reflector {
        Reflector {
            wiring_type: ReflectorType::B
        }
    }
}

impl StatefulKey for Reflector {
    fn reset(&mut self, _language: &Language) {
        self.wiring_type = ReflectorType::B;
    }
    fn randomize(&mut self, _language: &Language, rng: &mut impl Rng) {
        self.wiring_type = rng.gen();
    }
}
