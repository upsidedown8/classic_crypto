use crate::{
    error::Result, lang::Language, Affine, Asymmetric, Atbash, Caesar, Keyed, Solve, Symmetric,
};
use structopt::{clap::arg_enum, StructOpt};

arg_enum! {
    #[derive(StructOpt, Debug)]
    #[structopt(rename_all = "snake")]
    pub enum CipherOpt {
        // ADFGVX,
        // ADFGX,
        Affine,
        Atbash,
        // Autokey,
        // Baconian,
        // Beaufort,
        // Bellaso,
        // Bifid,
        // BlockTransposition,
        Caesar,
        // Chaocipher,
        // ClassicVigenère,
        // Clock,
        // Chase,
        // ColumnTransposition,
        // Enigma,
        // Fialka,
        // FourSquare,
        // FractionatedMorse,
        // Hill,
        // HomophonicSubstitution,
        // KeyedVigenère,
        // Lorenz,
        // Morse,
        // MyszkowskiTransposition,
        // Playfair,
        // PolybiusSquare,
        // Porta,
        // Purple,
        // Railfence,
        // Rot13,
        // Scytale,
        // SimpleSubstitution,
        // Solitaire,
        // StraddleCheckerboard,
        // Trifid,
        // TwoSquare,
        // Typex,
        // VIC,
    }
}

pub enum Cipher {
    // ADFGVX(ADFGVX),
    // ADFGX(ADFGX),
    Affine(Affine),
    Atbash(Atbash),
    // Autokey(Autokey),
    // Baconian(Baconian),
    // Beaufort(Beaufort),
    // Bellaso(Bellaso),
    // Bifid(Bifid),
    // BlockTransposition(BlockTransposition),
    Caesar(Caesar),
    // Chaocipher(Chaocipher),
    // ClassicVigenère(ClassicVigenère),
    // Clock(Clock),
    // Chase(Chase),
    // ColumnTransposition(ColumnTransposition),
    // Enigma(Enigma),
    // Fialka(Fialka),
    // FourSquare(FourSquare),
    // FractionatedMorse(FractionatedMorse),
    // Hill(Hill),
    // HomophonicSubstitution(HomophonicSubstitution),
    // KeyedVigenère(KeyedVigenère),
    // Lorenz(Lorenz),
    // Morse(Morse),
    // MyszkowskiTransposition(MyszkowskiTransposition),
    // Playfair(Playfair),
    // PolybiusSquare(PolybiusSquare),
    // Porta(Porta),
    // Purple(Purple),
    // Railfence(Railfence),
    // Rot13(Rot13),
    // Scytale(Scytale),
    // SimpleSubstitution(SimpleSubstitution),
    // Solitaire(Solitaire),
    // StraddleCheckerboard(StraddleCheckerboard),
    // Trifid(Trifid),
    // TwoSquare(TwoSquare),
    // Typex(Typex),
    // VIC(VIC),
}

impl Cipher {
    pub fn new(cipher_opt: &CipherOpt, language: &mut Language) -> Cipher {
        match cipher_opt {
            CipherOpt::Affine => Cipher::Affine(Affine::new(language)),
            CipherOpt::Atbash => Cipher::Atbash(Atbash {}),
            CipherOpt::Caesar => Cipher::Caesar(Caesar::new(language)),
        }
    }
    pub fn randomize(&mut self, language: &mut Language, rng: &mut impl rand::Rng) {
        match self {
            Cipher::Affine(ref mut affine) => affine.randomize(language, rng),
            Cipher::Caesar(ref mut caesar) => caesar.randomize(language, rng),
            _ => {}
        };
    }
    pub fn reset(&mut self, language: &mut Language) {
        match self {
            Cipher::Affine(ref mut affine) => affine.reset(language),
            Cipher::Caesar(ref mut caesar) => caesar.reset(language),
            _ => {}
        };
    }
    pub fn to_string(&self, language: &mut Language) -> String {
        match self {
            Cipher::Affine(ref affine) => affine.to_string(language),
            Cipher::Caesar(ref caesar) => caesar.to_string(language),
            _ => String::new(),
        }
    }
    pub fn encrypt(&self, language: &mut Language, msg: &str) -> String {
        match self {
            Cipher::Affine(ref affine) => affine.encrypt(language, msg),
            Cipher::Atbash(ref atbash) => atbash.run(language, msg),
            Cipher::Caesar(ref caesar) => caesar.encrypt(language, msg),
        }
    }
    pub fn decrypt(&self, language: &mut Language, msg: &str) -> String {
        match self {
            Cipher::Affine(ref affine) => affine.decrypt(language, msg),
            Cipher::Atbash(ref atbash) => atbash.run(language, msg),
            Cipher::Caesar(ref caesar) => caesar.decrypt(language, msg),
        }
    }
    pub fn solve(&mut self, language: &mut Language, msg: &str) {
        match self {
            Cipher::Affine(ref mut affine) => affine.solve(language, msg),
            Cipher::Caesar(ref mut caesar) => caesar.solve(language, msg),
            _ => {}
        }
    }
    pub fn set_key(&mut self, key: &[String]) -> Result<()> {
        Ok(())
    }
}
