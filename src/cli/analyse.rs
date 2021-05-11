use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "snake")]
pub struct Analyse {
    /// The ciphertext (or use stdin)
    #[structopt(short = "t", long)]
    text: Option<String>,
    
    /// Calculate index of coincedence
    #[structopt(short = "i", long)]
    ioc: bool,
    
    /// Calculate periodic index of coincedence
    #[structopt(short = "p", long)]
    pioc: bool,

    /// Calculate chi-squared score
    #[structopt(short = "c", long)]
    chi: bool,
    
    /// Display a letter frequency graph
    #[structopt(short = "g", long)]
    graph: bool,

    /// Guess the cipher type
    #[structopt(long)]
    likely_cipher: bool,
    
    /// Find prime factorisations for various lengths
    #[structopt(short = "f", long)]
    factor_length: bool,
    
    /// Guess a monoalphabetic decryption key based on frequencies
    #[structopt(long)]
    guess_key: bool,

    /// Language file path
    #[structopt(short = "l", long, parse(from_os_str))]
    lang_file: PathBuf,
}
