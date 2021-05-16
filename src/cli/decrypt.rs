use std::path::PathBuf;
use structopt::StructOpt;

use super::cipher::CipherOpt;
use super::RunSubmodule;
use crate::{cli::cipher::Cipher, error::Result, lang::Language};

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "snake")]
pub struct Decrypt {
    /// Which cipher to use
    #[structopt(short = "c", long, possible_values = &CipherOpt::variants(), case_insensitive = true)]
    cipher: CipherOpt,

    /// The arguments to parse a key from
    #[structopt(short = "k", long)]
    key: Option<Vec<String>>,

    /// Use a random key
    #[structopt(short = "r", long)]
    rand: bool,

    /// Use the default (identity) key
    #[structopt(short = "d", long)]
    default: bool,

    /// View the argument format for the key(s)
    #[structopt(long)]
    key_help: bool,

    /// The ciphertext
    #[structopt(short = "t", long)]
    text: String,

    /// Language file path
    #[structopt(short = "l", long, parse(from_os_str))]
    lang_file: PathBuf,
}

impl RunSubmodule for Decrypt {
    fn run(&self) -> Result<()> {
        let mut language = Language::from_pathbuf(&PathBuf::from(&self.lang_file))?;

        let mut cipher = Cipher::new(&self.cipher, &mut language);

        if self.key_help {
            println!("{}", cipher.key_help());
        } else {
            if self.rand {
                cipher.randomize(&mut language);
            } else if self.default {
                cipher.reset(&mut language);
            } else if let Some(key) = &self.key {
                cipher.set_key(&mut language, key)?;
            }

            println!("{}", cipher.to_string(&mut language));

            let ciphertext = cipher.decrypt(&mut language, &self.text);

            println!("{}", ciphertext);
        }

        Ok(())
    }
}
