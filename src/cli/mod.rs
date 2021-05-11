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

pub fn run() -> Result<(), &'static str> {
    let options = ClassicCrypto::from_args();
    println!("{:#?}", options);
    Ok(())
}
