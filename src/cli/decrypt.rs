use std::path::PathBuf;
use structopt::StructOpt;

use super::cipher::Cipher;
use super::RunSubmodule;
use crate::error::Result;

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "snake")]
pub struct Decrypt {
    /// Which cipher to use
    #[structopt(short = "c", long, possible_values = &Cipher::variants(), case_insensitive = true)]
    cipher: Cipher,

    /// The arguments to parse a key from
    #[structopt(short = "k", long)]
    key: Option<Vec<String>>,

    /// Use a random key
    #[structopt(short = "r", long)]
    rand: bool,

    /// Use the default (identity) key
    #[structopt(short = "d", long)]
    default: bool,

    /// The ciphertext
    #[structopt(short = "t", long)]
    text: String,

    /// Language file path
    #[structopt(short = "l", long, parse(from_os_str))]
    lang_file: PathBuf,
}

impl RunSubmodule for Decrypt {
    fn run(&self) -> Result<()> {
        Ok(())
    }
}
