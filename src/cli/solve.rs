use std::path::PathBuf;
use structopt::StructOpt;

use super::cipher::CipherOpt;
use super::RunSubmodule;
use crate::{cli::cipher::Cipher, error::Result, lang::Language};

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "snake")]
pub struct Solve {
    /// Which cipher to use
    #[structopt(short = "c", long, possible_values = &CipherOpt::variants(), case_insensitive = true)]
    cipher: CipherOpt,

    /// The ciphertext
    #[structopt(short = "t", long)]
    text: String,

    /// Only display the key
    #[structopt(long)]
    key_only: bool,

    /// Only display the plaintext
    #[structopt(long)]
    plain_only: bool,

    /// Language file path
    #[structopt(short = "l", long, parse(from_os_str))]
    lang_file: PathBuf,
}

impl RunSubmodule for Solve {
    fn run(&self) -> Result<()> {
        let mut language = Language::from_pathbuf(&PathBuf::from(&self.lang_file))?;

        let mut cipher = Cipher::new(&self.cipher, &mut language);

        cipher.solve(&mut language, &self.text);

        let all = !(self.key_only || self.plain_only);

        if self.key_only || all {
            println!("{}", cipher.to_string(&mut language));
        }
        if self.plain_only || all {
            println!("plaintext: {}", cipher.decrypt(&mut language, &self.text));
        }

        Ok(())
    }
}
