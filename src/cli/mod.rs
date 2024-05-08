pub mod base64;
pub mod csv;
pub mod gen_pass;
pub mod http;
pub mod text;
pub mod time;

use std::path::{Path, PathBuf};

use self::http::HttpSubCommand;
use self::time::TimeOpts;
use clap::{Parser, Subcommand};

pub use self::base64::Base64SubCommand;
pub use self::csv::{CsvOpts, OutputFormat};
pub use self::gen_pass::GenPassOpts;
pub use self::text::{TextSignFormat, TextSignOpts, TextSubCommand, TextVerifyOpts};

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
    #[command(subcommand, about = "Base64 encode & decode")]
    Base64(Base64SubCommand),
    #[command(about = "Time utils")]
    Time(TimeOpts),
    #[command(subcommand, about = "Text sign & verify")]
    Text(TextSubCommand),
    #[command(subcommand, about = "Http")]
    Http(HttpSubCommand),
}

fn verify_file(filename: &str) -> Result<String, &'static str> {
    if filename == "-" || std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File dose not exist")
    }
}

fn verify_dir(path: &str) -> Result<PathBuf, &'static str> {
    let p = Path::new(path);
    if p.exists() && p.is_dir() {
        Ok(path.into())
    } else {
        Err("Path does not exist or is not a directory")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_file() {
        assert_eq!(verify_file("-"), Ok("-".into()));
        assert_eq!(verify_file("*"), Err("File dose not exist"));
        assert_eq!(verify_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(verify_file("not-exists"), Err("File dose not exist"))
    }
}
