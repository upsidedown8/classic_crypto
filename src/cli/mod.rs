use crate::error::Result;

use self::analyse::Analyse;
use self::auto_solve::AutoSolve;
use self::decrypt::Decrypt;
use self::encrypt::Encrypt;
use self::lang_gen::LangGen;
use self::solve::Solve;

mod analyse;
mod auto_solve;
mod decrypt;
mod encrypt;
mod lang_gen;
mod solve;

mod cipher;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "classic_crypto")]
#[structopt(rename_all = "snake")]
/// Encrypt, decrypt, analyse and solve classical ciphers
pub enum ClassicCrypto {
    /// Encrypt a message using a cipher
    Encrypt(Encrypt),

    /// Decrypt a message using a cipher
    Decrypt(Decrypt),

    /// Solve a message encrypted with a known cipher
    Solve(Solve),

    /// Solve a message encrypted with an unknown cipher
    AutoSolve(AutoSolve),

    /// Analyse a ciphertext
    Analyse(Analyse),

    /// Generate a Langauge file from a configuration file and text corpus
    LangGen(LangGen),
}

trait RunSubmodule {
    fn run(&self) -> Result<()>;
}

pub fn run() -> Result<()> {
    let options = ClassicCrypto::from_args();

    match options {
        ClassicCrypto::Encrypt(en) => en.run(),
        ClassicCrypto::Decrypt(de) => de.run(),
        ClassicCrypto::Solve(so) => so.run(),
        ClassicCrypto::AutoSolve(au) => au.run(),
        ClassicCrypto::Analyse(an) => an.run(),
        ClassicCrypto::LangGen(la) => la.run(),
    }
}
