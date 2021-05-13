use structopt::{clap::arg_enum, StructOpt};
use crate::{Affine, Asymmetric, Caesar, Keyed, Solve, lang::Language};

arg_enum! {
    #[derive(StructOpt, Debug)]
    #[structopt(rename_all = "snake")]
    pub enum CipherOpt {
        Affine,
        Caesar,
    }
}

pub enum Cipher {
    Affine(Affine),
    Caesar(Caesar),
}

impl Cipher {
    pub fn new(cipher_opt: &CipherOpt, language: &mut Language) -> Cipher {
        match cipher_opt {
            CipherOpt::Affine => Cipher::Affine(Affine::new(language)),
            CipherOpt::Caesar => Cipher::Caesar(Caesar::new(language)),
        }
    }
    pub fn randomize(&mut self, language: &mut Language, rng: &mut impl rand::Rng) {
        match self {
            Cipher::Affine(ref mut affine) => affine.randomize(language, rng),
            Cipher::Caesar(ref mut caesar) => caesar.randomize(language, rng),
        };
    }
    pub fn reset(&mut self, language: &mut Language) {
        match self {
            Cipher::Affine(ref mut affine) => affine.reset(language),
            Cipher::Caesar(ref mut caesar) => caesar.reset(language),
        };
    }
    pub fn to_string(&self, language: &mut Language) -> String {
        match self {
            Cipher::Affine(ref affine) => affine.to_string(language),
            Cipher::Caesar(ref caesar) => caesar.to_string(language),
        }
    }
    pub fn encrypt(&self, language: &mut Language, msg: &str) -> String {
        match self {
            Cipher::Affine(ref affine) => affine.encrypt(language, msg),
            Cipher::Caesar(ref caesar) => caesar.encrypt(language, msg),
        }
    }
    pub fn decrypt(&self, language: &mut Language, msg: &str) -> String {
        match self {
            Cipher::Affine(ref affine) => affine.decrypt(language, msg),
            Cipher::Caesar(ref caesar) => caesar.decrypt(language, msg),
        }
    }
    pub fn solve(&mut self, language: &mut Language, msg: &str) {
        match self {
            Cipher::Affine(ref mut affine) => affine.solve(language, msg),
            Cipher::Caesar(ref mut caesar) => caesar.solve(language, msg),
        }
    }
}
