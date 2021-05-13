use owo_colors::{AnsiColors, OwoColorize};
use std::path::PathBuf;
use structopt::StructOpt;

use super::RunSubmodule;
use crate::{
    error::{Error, Result},
    lang::Language,
};

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

const PIOC_MIN: usize = 1;
const PIOC_MAX: usize = 100;
const TERM_WIDTH: usize = 50;
const MIN_LEN: usize = 4;

impl RunSubmodule for Analyse {
    fn run(&self) -> Result<()> {
        let language = Language::from_pathbuf(&PathBuf::from(&self.lang_file))?;
        let data = language.string_to_vec(&self.text);

        if data.len() < 8 {
            return Err(Error::InsufficientInputLen {
                expected: MIN_LEN,
                actual: data.len(),
            })
        }

        let all = !(self.chi
            || self.ioc
            || self.pioc
            || self.graph
            || self.likely_cipher
            || self.factor_length
            || self.guess_key);

        if self.ioc || all {
            println!(
                "{}: {:.04}\n",
                "Index of coincedence".to_string().bold(),
                language.index_of_coincedence(&data),
            );
        }
        if self.chi || all {
            println!(
                "{}: {:.04}\n",
                "Chi squared".to_string().bold(),
                language.chi_squared(&data)
            );
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
            use std::io::Write;

            let mut handle = std::io::BufWriter::new(std::io::stdout());

            let map_err = |_| Error::CouldntWriteToStdout;

            writeln!(
                handle,
                "{}",
                "Periodic Index of Coincedence".to_string().bold()
            )
            .map_err(map_err)?;

            for period in PIOC_MIN..=PIOC_MAX.min(data.len()) {
                let pioc = language.periodic_ioc(&data, period);

                let abs_diff = (pioc - language.expected_ioc()).abs();
                let multiplier = 1.0 - abs_diff / language.expected_ioc();
                let width = if pioc.is_nan() {
                    0
                } else {
                    (TERM_WIDTH as f64 * multiplier) as usize
                };

                let color = if abs_diff < 0.008 {
                    AnsiColors::BrightMagenta
                } else {
                    AnsiColors::White
                };

                writeln!(
                    handle,
                    "{}",
                    format!(
                        "{:<6.04} => {:<3} [ {}=>{} ]",
                        pioc,
                        period,
                        "=".repeat(width),
                        " ".repeat(TERM_WIDTH - width),
                    ).color(color),
                ).map_err(map_err)?;
            }

            writeln!(handle).map_err(map_err)?;

            handle.flush().map_err(map_err)?;
        }

        Ok(())
    }
}
