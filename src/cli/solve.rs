use std::path::PathBuf;
use structopt::StructOpt;

use super::cipher::Cipher;

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "snake")]
pub struct Solve {
    /// Which cipher to use
    #[structopt(short = "c", long, possible_values = &Cipher::variants(), case_insensitive = true)]
    cipher: Cipher,

    /// The ciphertext (or use stdin)
    #[structopt(short = "t", long)]
    text: Option<String>,

    /// Display each solution as it is found
    #[structopt(long)]
    log: bool,

    /// Pretty print the key
    #[structopt(long)]
    pretty_key: bool,
    
    /// Do not print the key at all, only the plaintext
    #[structopt(long)]
    plain_only: bool,

    /// Language file path
    #[structopt(short = "l", long, parse(from_os_str))]
    lang_file: PathBuf,
}
