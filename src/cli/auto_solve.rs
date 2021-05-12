use std::path::PathBuf;
use structopt::StructOpt;

use super::RunSubmodule;
use crate::error::Result;

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "snake")]
pub struct AutoSolve {
    /// The ciphertext to solve
    #[structopt(short = "t", long)]
    text: String,

    /// Display each solution as it is found
    #[structopt(long)]
    log: bool,

    /// Pretty print the key
    #[structopt(long)]
    pretty_key: bool,

    /// Do not print the key at all, only the plaintext
    #[structopt(long)]
    plain_only: bool,

    /// Test monoalphabetic ciphers
    #[structopt(long)]
    monoalph: bool,

    ///Test polyalphabetic ciphers
    #[structopt(long)]
    polyalph: bool,

    /// Test polygraphic ciphers
    #[structopt(long)]
    polygraph: bool,

    /// Test transposition ciphers
    #[structopt(long)]
    transpos: bool,

    /// Language file path
    #[structopt(short = "l", long, parse(from_os_str))]
    lang_file: PathBuf,
}

impl RunSubmodule for AutoSolve {
    fn run(&self) -> Result<()> {
        Ok(())
    }
}
