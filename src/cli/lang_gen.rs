use super::{CliResult, RunSubmodule};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "snake")]
pub struct LangGen {
    /// Path to a TOML file containing language details, excluding
    /// scoring stats
    #[structopt(short = "f", long, parse(from_os_str))]
    config: PathBuf,

    /// Path to a .txt file containg the text corpus
    #[structopt(short = "c", long, parse(from_os_str))]
    corpus: PathBuf,

    /// Output file path
    #[structopt(short = "o", long, parse(from_os_str))]
    output: PathBuf,
}

impl RunSubmodule for LangGen {
    fn run(&self) -> CliResult {
        Ok(())
    }
}
