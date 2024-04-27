use std::fmt;

use crate::cli::verify_file;
use clap::{Parser, ValueEnum};

#[derive(Debug, Parser)]
pub struct CsvOpts {
    /// Input file path
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,

    /// Output file path
    #[arg(short, long)]
    pub output: Option<String>,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Json)]
    pub format: OutputFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum OutputFormat {
    Json,
    Yaml,
}

impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}
