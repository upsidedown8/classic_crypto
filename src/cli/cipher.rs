use crate::{
    error::{Error, Result},
    lang::Language,
    Affine, Asymmetric, Atbash, Autokey, Beaufort, Bellaso, BlockTransposition, Caesar,
    ClassicVigenere, ColumnTransposition, Enigma, Keyed, KeyedVigenere, Morse, Porta, Railfence,
    Rot13, Scytale, SimpleSubstitution, Solve, Symmetric,
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
        Autokey,
        // Baconian,
        Beaufort,
        Bellaso,
        // Bifid,
        BlockTranspos,
        Caesar,
        // Chaocipher,
        ClassicVig,
        // Clock,
        // Chase,
        ColumnTranspos,
        Enigma,
        // Fialka,
        // FourSquare,
        // FractionatedMorse,
        // Hill,
        // HomophonicSubstitution,
        KeyedVig,
        // Lorenz,
        Morse,
        // MyszkowskiTransposition,
        // Playfair,
        // PolybiusSquare,
        Porta,
        // Purple,
        Railfence,
        Rot13,
        Scytale,
        SimpleSub,
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
    Autokey(Autokey),
    // Baconian(Baconian),
    Beaufort(Beaufort),
    Bellaso(Bellaso),
    // Bifid(Bifid),
    BlockTransposition(BlockTransposition),
    Caesar(Caesar),
    // Chaocipher(Chaocipher),
    ClassicVigenere(ClassicVigenere),
    // Clock(Clock),
    // Chase(Chase),
    ColumnTransposition(ColumnTransposition),
    Enigma(Enigma),
    // Fialka(Fialka),
    // FourSquare(FourSquare),
    // FractionatedMorse(FractionatedMorse),
    // Hill(Hill),
    // HomophonicSubstitution(HomophonicSubstitution),
    KeyedVigenere(KeyedVigenere),
    // Lorenz(Lorenz),
    Morse(Morse),
    // MyszkowskiTransposition(MyszkowskiTransposition),
    // Playfair(Playfair),
    // PolybiusSquare(PolybiusSquare),
    Porta(Porta),
    // Purple(Purple),
    Railfence(Railfence),
    Rot13(Rot13),
    Scytale(Scytale),
    SimpleSubstitution(SimpleSubstitution),
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
            CipherOpt::Autokey => Cipher::Autokey(Autokey::new(language)),
            CipherOpt::Beaufort => Cipher::Beaufort(Beaufort::new(language)),
            CipherOpt::Bellaso => Cipher::Bellaso(Bellaso::new(language)),
            CipherOpt::BlockTranspos => {
                Cipher::BlockTransposition(BlockTransposition::new(language))
            }
            CipherOpt::Caesar => Cipher::Caesar(Caesar::new(language)),
            CipherOpt::ClassicVig => Cipher::ClassicVigenere(ClassicVigenere::new(language)),
            CipherOpt::ColumnTranspos => {
                Cipher::ColumnTransposition(ColumnTransposition::new(language))
            }
            CipherOpt::Enigma => Cipher::Enigma(Enigma::new(language)),
            CipherOpt::KeyedVig => Cipher::KeyedVigenere(KeyedVigenere::new(language)),
            CipherOpt::Morse => Cipher::Morse(Morse {}),
            CipherOpt::Porta => Cipher::Porta(Porta::new(language)),
            CipherOpt::Railfence => Cipher::Railfence(Railfence::new(language)),
            CipherOpt::Rot13 => Cipher::Rot13(Rot13 {}),
            CipherOpt::Scytale => Cipher::Scytale(Scytale::new(language)),
            CipherOpt::SimpleSub => Cipher::SimpleSubstitution(SimpleSubstitution::new(language)),
        }
    }
    pub fn randomize(&mut self, language: &mut Language) {
        match self {
            Cipher::Affine(ref mut affine) => affine.randomize(language),
            Cipher::Autokey(ref mut autokey) => autokey.randomize(language),
            Cipher::Beaufort(ref mut beaufort) => beaufort.randomize(language),
            Cipher::Bellaso(ref mut bellaso) => bellaso.randomize(language),
            Cipher::BlockTransposition(ref mut blocktranspos) => blocktranspos.randomize(language),
            Cipher::Caesar(ref mut caesar) => caesar.randomize(language),
            Cipher::ClassicVigenere(ref mut classicvig) => classicvig.randomize(language),
            Cipher::ColumnTransposition(ref mut columntranspos) => {
                columntranspos.randomize(language)
            }
            Cipher::Enigma(ref mut enigma) => enigma.randomize(language),
            Cipher::KeyedVigenere(ref mut keyedvig) => keyedvig.randomize(language),
            Cipher::Porta(ref mut porta) => porta.randomize(language),
            Cipher::Railfence(ref mut railfence) => railfence.randomize(language),
            Cipher::Scytale(ref mut scytale) => scytale.randomize(language),
            Cipher::SimpleSubstitution(ref mut simplesub) => simplesub.randomize(language),
            _ => {}
        };
    }
    pub fn reset(&mut self, language: &mut Language) {
        match self {
            Cipher::Affine(ref mut affine) => affine.reset(language),
            Cipher::Autokey(ref mut autokey) => autokey.reset(language),
            Cipher::Beaufort(ref mut beaufort) => beaufort.reset(language),
            Cipher::Bellaso(ref mut bellaso) => bellaso.reset(language),
            Cipher::BlockTransposition(ref mut blocktranspos) => blocktranspos.reset(language),
            Cipher::Caesar(ref mut caesar) => caesar.reset(language),
            Cipher::ClassicVigenere(ref mut classicvig) => classicvig.reset(language),
            Cipher::ColumnTransposition(ref mut columntranspos) => columntranspos.reset(language),
            Cipher::Enigma(ref mut enigma) => enigma.reset(language),
            Cipher::KeyedVigenere(ref mut keyedvig) => keyedvig.reset(language),
            Cipher::Porta(ref mut porta) => porta.reset(language),
            Cipher::Railfence(ref mut railfence) => railfence.reset(language),
            Cipher::Scytale(ref mut scytale) => scytale.reset(language),
            Cipher::SimpleSubstitution(ref mut simplesub) => simplesub.reset(language),
            _ => {}
        };
    }
    pub fn to_string(&self, language: &mut Language) -> String {
        match self {
            Cipher::Affine(ref affine) => affine.to_string(language),
            Cipher::Autokey(ref autokey) => autokey.to_string(language),
            Cipher::Beaufort(ref beaufort) => beaufort.to_string(language),
            Cipher::Bellaso(ref bellaso) => bellaso.to_string(language),
            Cipher::BlockTransposition(ref blocktranspos) => blocktranspos.to_string(language),
            Cipher::Caesar(ref caesar) => caesar.to_string(language),
            Cipher::ClassicVigenere(ref classicvig) => classicvig.to_string(language),
            Cipher::ColumnTransposition(ref columntranspos) => columntranspos.to_string(language),
            Cipher::Enigma(ref enigma) => enigma.to_string(language),
            Cipher::KeyedVigenere(ref keyedvig) => keyedvig.to_string(language),
            Cipher::Porta(ref porta) => porta.to_string(language),
            Cipher::Railfence(ref railfence) => railfence.to_string(language),
            Cipher::Scytale(ref scytale) => scytale.to_string(language),
            Cipher::SimpleSubstitution(ref simplesub) => simplesub.to_string(language),
            _ => String::new(),
        }
    }
    pub fn encrypt(&self, language: &mut Language, msg: &str) -> String {
        match self {
            Cipher::Affine(ref affine) => affine.encrypt(language, msg),
            Cipher::Atbash(ref atbash) => atbash.run(language, msg),
            Cipher::Autokey(ref autokey) => autokey.encrypt(language, msg),
            Cipher::Beaufort(ref beaufort) => beaufort.run(language, msg),
            Cipher::Bellaso(ref bellaso) => bellaso.run(language, msg),
            Cipher::BlockTransposition(ref blocktranspos) => blocktranspos.encrypt(language, msg),
            Cipher::Caesar(ref caesar) => caesar.encrypt(language, msg),
            Cipher::ClassicVigenere(ref classicvig) => classicvig.encrypt(language, msg),
            Cipher::ColumnTransposition(ref columntranspos) => {
                columntranspos.encrypt(language, msg)
            }
            Cipher::Enigma(ref enigma) => enigma.run(language, msg),
            Cipher::KeyedVigenere(ref keyedvig) => keyedvig.encrypt(language, msg),
            Cipher::Morse(ref morse) => morse.encrypt(language, msg),
            Cipher::Porta(ref porta) => porta.run(language, msg),
            Cipher::Railfence(ref railfence) => railfence.encrypt(language, msg),
            Cipher::Rot13(ref rot13) => rot13.run(language, msg),
            Cipher::Scytale(ref scytale) => scytale.encrypt(language, msg),
            Cipher::SimpleSubstitution(ref simplesub) => simplesub.encrypt(language, msg),
        }
    }
    pub fn decrypt(&self, language: &mut Language, msg: &str) -> String {
        match self {
            Cipher::Affine(ref affine) => affine.decrypt(language, msg),
            Cipher::Atbash(ref atbash) => atbash.run(language, msg),
            Cipher::Autokey(ref autokey) => autokey.decrypt(language, msg),
            Cipher::Beaufort(ref beaufort) => beaufort.run(language, msg),
            Cipher::Bellaso(ref bellaso) => bellaso.run(language, msg),
            Cipher::BlockTransposition(ref blocktranspos) => blocktranspos.decrypt(language, msg),
            Cipher::Caesar(ref caesar) => caesar.decrypt(language, msg),
            Cipher::ClassicVigenere(ref classicvig) => classicvig.decrypt(language, msg),
            Cipher::ColumnTransposition(ref columntranspos) => {
                columntranspos.decrypt(language, msg)
            }
            Cipher::Enigma(ref enigma) => enigma.run(language, msg),
            Cipher::KeyedVigenere(ref keyedvig) => keyedvig.decrypt(language, msg),
            Cipher::Morse(ref morse) => morse.decrypt(language, msg),
            Cipher::Porta(ref porta) => porta.run(language, msg),
            Cipher::Railfence(ref railfence) => railfence.decrypt(language, msg),
            Cipher::Rot13(ref rot13) => rot13.run(language, msg),
            Cipher::Scytale(ref scytale) => scytale.decrypt(language, msg),
            Cipher::SimpleSubstitution(ref simplesub) => simplesub.decrypt(language, msg),
        }
    }
    pub fn solve(&mut self, language: &mut Language, msg: &str) {
        match self {
            Cipher::Affine(ref mut affine) => affine.solve(language, msg),
            Cipher::Autokey(ref mut autokey) => autokey.solve(language, msg),
            Cipher::Beaufort(ref mut beaufort) => beaufort.solve(language, msg),
            Cipher::Bellaso(ref mut bellaso) => bellaso.solve(language, msg),
            Cipher::BlockTransposition(ref mut blocktranspos) => blocktranspos.solve(language, msg),
            Cipher::Caesar(ref mut caesar) => caesar.solve(language, msg),
            Cipher::ClassicVigenere(ref mut classicvig) => classicvig.solve(language, msg),
            Cipher::ColumnTransposition(ref mut columntranspos) => {
                columntranspos.solve(language, msg)
            }
            Cipher::Porta(ref mut porta) => porta.solve(language, msg),
            Cipher::Railfence(ref mut railfence) => railfence.solve(language, msg),
            Cipher::Scytale(ref mut scytale) => scytale.solve(language, msg),
            Cipher::SimpleSubstitution(ref mut simplesub) => simplesub.solve(language, msg),
            _ => {}
        }
    }
    pub fn set_key(&mut self, language: &mut Language, args: &[String]) -> Result<()> {
        let keys = match self {
            Cipher::Affine(ref mut affine) => affine.keys_mut(),
            Cipher::Autokey(ref mut autokey) => autokey.keys_mut(),
            Cipher::Beaufort(ref mut beaufort) => beaufort.keys_mut(),
            Cipher::Bellaso(ref mut bellaso) => bellaso.keys_mut(),
            Cipher::BlockTransposition(ref mut blocktranspos) => blocktranspos.keys_mut(),
            Cipher::Caesar(ref mut caesar) => caesar.keys_mut(),
            Cipher::ClassicVigenere(ref mut classicvig) => classicvig.keys_mut(),
            Cipher::ColumnTransposition(ref mut columntranspos) => columntranspos.keys_mut(),
            Cipher::Enigma(ref mut enigma) => enigma.keys_mut(),
            Cipher::KeyedVigenere(ref mut keyedvig) => keyedvig.keys_mut(),
            Cipher::Porta(ref mut porta) => porta.keys_mut(),
            Cipher::Railfence(ref mut railfence) => railfence.keys_mut(),
            Cipher::Scytale(ref mut scytale) => scytale.keys_mut(),
            Cipher::SimpleSubstitution(ref mut simplesub) => simplesub.keys_mut(),
            _ => vec![],
        };

        let args = args
            .iter()
            .map(|arg| arg.split(':').collect::<Vec<_>>())
            .filter(|arg| arg.len() >= 2)
            .collect::<Vec<_>>();

        for key in keys {
            for arg in args.iter() {
                let short_name = key.key_info().short_name.clone();
                if arg[0] == short_name {
                    let remaining_arg = arg[1..]
                        .iter()
                        .copied()
                        .fold(String::new(), |acc, x| acc + x);
                    key.set_key_str(language, &remaining_arg)
                        .map_err(|err| match err {
                            Error::InvalidKeyFmt { expected, actual } => Error::InvalidKeyFmt {
                                expected: format!("[{}]: {}", short_name, expected),
                                actual,
                            },
                            _ => err,
                        })?;
                }
            }
        }

        Ok(())
    }
    pub fn key_help(&self) -> String {
        let keys = match self {
            Cipher::Affine(ref affine) => affine.keys(),
            Cipher::Autokey(ref autokey) => autokey.keys(),
            Cipher::Beaufort(ref beaufort) => beaufort.keys(),
            Cipher::Bellaso(ref bellaso) => bellaso.keys(),
            Cipher::BlockTransposition(ref blocktranspos) => blocktranspos.keys(),
            Cipher::Caesar(ref caesar) => caesar.keys(),
            Cipher::ClassicVigenere(ref classicvig) => classicvig.keys(),
            Cipher::ColumnTransposition(ref columntranspos) => columntranspos.keys(),
            Cipher::Enigma(ref enigma) => enigma.keys(),
            Cipher::KeyedVigenere(ref keyedvig) => keyedvig.keys(),
            Cipher::Porta(ref porta) => porta.keys(),
            Cipher::Railfence(ref railfence) => railfence.keys(),
            Cipher::Scytale(ref scytale) => scytale.keys(),
            Cipher::SimpleSubstitution(ref simplesub) => simplesub.keys(),
            _ => vec![],
        };

        let mut result = String::new();

        for key in keys {
            result.push_str(&format!(
                "{} ({})\n",
                key.key_info().short_name,
                key.key_info().desc,
            ));
        }

        result
    }
}
