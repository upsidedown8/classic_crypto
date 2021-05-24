use crate::key::{Key, Plugboard, Reflector, ReflectorType, Rotor, RotorType};
use crate::lang::Language;
use crate::{
    cipher::{Keyed, Symmetric},
    key::{IdentityKey, IoKey},
};

#[derive(Clone)]
pub struct Enigma {
    pub plugboard: Plugboard,
    // 0=Rightmost ... 2/3=Leftmost rotor
    pub rotors: Vec<Rotor>,
    pub reflector: Reflector,
}

impl Enigma {
    pub fn run_mut(&mut self, language: &Language, msg: &str) -> String {
        msg.chars()
            .map(|c| {
                if language.is_letter(&c) {
                    let mut cp = language.get_cp(&c);
                    self.step_rotors();
                    cp = self.plugboard.input(cp);
                    cp = self.rotor_pass(cp, false);
                    cp = self.reflector.input(cp);
                    cp = self.rotor_pass(cp, true);
                    cp = self.plugboard.input(cp);
                    language.update_cp(&c, cp)
                } else {
                    c
                }
            })
            .collect()
    }

    fn step_rotors(&mut self) {
        debug_assert!(self.rotors.len() >= 3);

        if self.rotors[1].is_on_notch() {
            self.rotors[1].step();
            self.rotors[2].step();
        } else if self.rotors[0].is_on_notch() {
            self.rotors[1].step();
        }
        self.rotors[0].step();
    }

    fn rotor_pass(&self, letter: i16, reverse: bool) -> i16 {
        let mut out_letter = letter;

        for i in 0..self.rotors.len() {
            let rotor_idx = if reverse {
                self.rotors.len() - 1 - i
            } else {
                i
            };
            out_letter = self.rotors[rotor_idx].input(out_letter, reverse);
        }

        out_letter
    }
}

impl Symmetric for Enigma {
    fn run(&self, language: &mut Language, msg: &str) -> String {
        let mut enigma_mut = self.clone();
        enigma_mut.run_mut(language, msg)
    }
}

impl Keyed for Enigma {
    fn new(language: &mut Language) -> Enigma {
        let mut result = Enigma {
            plugboard: Plugboard::identity(language),
            rotors: vec![
                *Rotor::new(language, RotorType::III).unwrap(),
                *Rotor::new(language, RotorType::II).unwrap(),
                *Rotor::new(language, RotorType::I).unwrap(),
            ],
            reflector: *Reflector::new(language, ReflectorType::B).unwrap(),
        };

        result.plugboard.info_mut().set("Plugboard", "plug");
        result.rotors[0].info_mut().set("Right rotor", "r0");
        result.rotors[1].info_mut().set("Middle rotor", "r1");
        result.rotors[2].info_mut().set("Left rotor", "r2");
        result.reflector.info_mut().set("Reflector", "ref");

        result
    }
    fn keys(&self) -> Vec<&dyn IoKey> {
        let mut result: Vec<&dyn IoKey> = vec![&self.plugboard, &self.reflector];

        self.rotors.iter().for_each(|rotor| result.push(rotor));

        result
    }
    fn keys_mut(&mut self) -> Vec<&mut dyn IoKey> {
        let mut result: Vec<&mut dyn IoKey> = vec![&mut self.plugboard, &mut self.reflector];

        self.rotors.iter_mut().for_each(|rotor| result.push(rotor));

        result
    }
}
