use clap::{Parser, ValueEnum};

use super::verify_file;

#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Decode a base64 to string")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    /// Specify the input file. If -, it means input from stdin
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    /// Encode format
    #[arg(long, value_enum, default_value_t = Base64Format::Standard)]
    pub format: Base64Format,
    /// Whether not use padding
    #[arg(long, default_value_t = false)]
    pub no_padding: bool,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    /// Specify the input file. If -, it means input from stdin
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    /// Decode format
    #[arg(long, value_enum, default_value_t = Base64Format::Standard)]
    pub format: Base64Format,
    /// Whether not use padding
    #[arg(long, default_value_t = false)]
    pub no_padding: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Base64Format {
    Standard,
    Urlsafe,
}
