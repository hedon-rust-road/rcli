pub mod csv;
pub mod gen_pass;

use clap::{Parser, Subcommand};

pub use self::csv::CsvOpts;
pub use self::gen_pass::GenPassOpts;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

/// rcli csv -i input.csv -o output.json --header -d ','
#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "genpass", about = "Generate a password")]
    GenPass(GenPassOpts),
}
