use owo_colors::{AnsiColors, OwoColorize};
use std::path::PathBuf;
use structopt::StructOpt;

use super::RunSubmodule;
use crate::{error::Result, lang::Language};

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "snake")]
pub struct Analyse {
    /// The ciphertext
    #[structopt(short = "t", long)]
    text: String,

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

const PIOC_MIN: usize = 3;
const PIOC_MAX: usize = 100;

impl RunSubmodule for Analyse {
    fn run(&self) -> Result<()> {
        let language = Language::from_pathbuf(&PathBuf::from(&self.lang_file))?;
        let data = language.string_to_vec(&self.text);

        let all = !(self.chi
            || self.ioc
            || self.pioc
            || self.graph
            || self.likely_cipher
            || self.factor_length
            || self.guess_key);

        if self.ioc || all {
            println!(
                "Index of coincedence: {}",
                language.index_of_coincedence(&data),
            );
        }
        if self.chi || all {
            println!("Chi squared: {}", language.chi_squared(&data));
        }
        if self.likely_cipher || all {
            unimplemented!();
        }
        if self.factor_length || all {
            unimplemented!();
        }
        if self.guess_key || all {
            unimplemented!();
        }
        if self.graph || all {
            unimplemented!();
        }
        if self.pioc || all {
            println!("+---------------+--------------------------+");
            println!("| Period Length | Avg Index of Coincedence |");
            println!("+---------------+--------------------------+");

            for period in PIOC_MIN..=PIOC_MAX {
                let pioc = language.periodic_ioc(&data, period);

                let color = if (pioc - language.expected_ioc()).abs() < 0.01 {
                    AnsiColors::Blue
                } else {
                    AnsiColors::White
                };

                println!(
                    "| {:<13} | {:<24} |",
                    period.to_string().color(color),
                    pioc.to_string().color(color),
                );
            }

            println!("+---------------+--------------------------+");
        }

        Ok(())
    }
}
