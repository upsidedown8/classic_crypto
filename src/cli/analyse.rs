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

const DEFAULT_COLOR: AnsiColors = AnsiColors::White;
const EMPHASIS_COLOR: AnsiColors = AnsiColors::BrightMagenta;

impl RunSubmodule for Analyse {
    fn run(&self) -> Result<()> {
        let language = Language::from_pathbuf(&PathBuf::from(&self.lang_file))?;
        let data = language.string_to_vec(&self.text);

        if data.len() < MIN_LEN {
            return Err(Error::InsufficientInputLen {
                expected: MIN_LEN,
                actual: data.len(),
            });
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
            println!("{}", "Text length".to_string().bold());

            unimplemented!();
        }
        if self.guess_key || all {
            unimplemented!();
        }
        if self.graph || all {
            println!("{}", "Frequency Graph".to_string().bold());

            // convert text to uppercase
            let uppercase = self
                .text
                .chars()
                .map(|ch| {
                    if language.is_lower(&ch) {
                        language.to_upper(&ch)
                    } else {
                        ch
                    }
                })
                .collect::<String>();

            // find the set of unique chars
            let mut unique_chars = Vec::<char>::new();
            for ch in uppercase.chars() {
                if !unique_chars.contains(&ch) {
                    unique_chars.push(ch)
                }
            }
            unique_chars.sort_unstable();

            // count each char
            let mut alphabetic_count = 0;
            let mut counts = vec![0; unique_chars.len()];
            for ch in uppercase.chars() {
                if let Some(idx) = unique_chars.iter().position(|x| *x == ch) {
                    counts[idx] += 1;
                }
                if language.is_letter(&ch) {
                    alphabetic_count += 1;
                }
            }

            // associate pairs of data & sort by frequency
            let mut graph_data = (0..unique_chars.len())
                .filter(|&idx| counts[idx] > 0)
                .map(|idx| (unique_chars[idx], counts[idx]))
                .collect::<Vec<_>>();
            graph_data.sort_by(|a, b| b.1.cmp(&a.1));

            // display data
            let max_count = match graph_data.iter().max_by(|a, b| b.cmp(a)) {
                Some(item) => item.1,
                _ => self.text.len(),
            };
            graph_data.iter().for_each(|item| {
                let color = {
                    let mut res = DEFAULT_COLOR;
                    if language.is_letter(&item.0) {
                        let alpha_proportion = item.1 as f64 / (alphabetic_count as f64);
                        let expected_proportion =
                            language.unigram_probabilities[language.get_cp(&item.0) as usize];

                        let diff = (alpha_proportion - expected_proportion).abs();

                        // require a 2% margin
                        if diff < 0.02 {
                            res = EMPHASIS_COLOR;
                        }
                    }
                    res
                };

                let percent = 100.0 * (item.1 as f64) / (self.text.len() as f64);
                let width =
                    ((item.1 as f64) / (max_count as f64) * (TERM_WIDTH as f64 - 6.0)) as usize;
                let string = format!(
                    "{:>5.02}% | {:<5} | {} | [{}=>{}]",
                    percent,
                    item.1,
                    item.0,
                    "=".repeat(width),
                    " ".repeat(TERM_WIDTH - 6 - width),
                );

                println!("{}", string.color(color));
            });

            println!();
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
                    EMPHASIS_COLOR
                } else {
                    DEFAULT_COLOR
                };

                writeln!(
                    handle,
                    "{}",
                    format!(
                        "{:<6.04} | {:<3} | [{}=>{}]",
                        pioc,
                        period,
                        "=".repeat(width),
                        " ".repeat(TERM_WIDTH - width),
                    )
                    .color(color),
                )
                .map_err(map_err)?;
            }

            writeln!(handle).map_err(map_err)?;

            handle.flush().map_err(map_err)?;
        }

        Ok(())
    }
}
