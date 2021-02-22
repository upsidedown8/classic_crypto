use crate::cipher::{Keyed, Symmetric};
use crate::key::{
    Key, KeyFrom, Plugboard, Reflector, ReflectorType, Rotor, RotorType, StatefulKey,
};
use crate::lang::Language;

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
        assert!(self.rotors.len() >= 3);

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
    fn run(&self, language: &Language, msg: &str) -> String {
        let mut enigma_mut = self.clone();
        enigma_mut.run_mut(language, msg)
    }
}

impl Keyed for Enigma {
    fn new(language: &Language) -> Enigma {
        Enigma {
            plugboard: Plugboard::new(language),
            rotors: vec![
                Rotor::create_from(language, RotorType::I),
                Rotor::create_from(language, RotorType::II),
                Rotor::create_from(language, RotorType::III),
            ],
            reflector: Reflector::create_from(language, ReflectorType::B),
        }
    }
    fn reset(&mut self, language: &Language) {
        self.plugboard.reset(language);
        self.rotors.iter_mut().for_each(|r| r.reset(language));
        self.reflector.reset(language);
    }
    fn randomize(&mut self, language: &Language, rng: &mut impl rand::Rng) {
        self.plugboard.randomize(language, rng);
        self.rotors
            .iter_mut()
            .for_each(|r| r.randomize(language, rng));
        self.reflector.randomize(language, rng);
    }
    fn to_string(&self, language: &Language) -> String {
        format!(
            "{}\n{}\n{}",
            self.plugboard.to_string(language),
            self.reflector.to_string(language),
            self.rotors
                .iter()
                .map(|&r| r.to_string(language))
                .fold(String::new(), |mut acc, x| {
                    acc.push_str(&x);
                    acc.push('\n');
                    acc
                }),
        )
    }
}
