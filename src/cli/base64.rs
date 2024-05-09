use clap::{Parser, ValueEnum};

use crate::{
    process::{process_decode, process_encode},
    CmdExector,
};

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

impl CmdExector for Base64SubCommand {
    async fn execute(self) -> anyhow::Result<()> {
        match self {
            Base64SubCommand::Decode(opts) => opts.execute().await,
            Base64SubCommand::Encode(opts) => opts.execute().await,
        }
    }
}

impl CmdExector for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encoded = process_decode(&self.input, self.format, self.no_padding)?;
        println!("\n{}", String::from_utf8(encoded)?);
        Ok(())
    }
}

impl CmdExector for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decoded = process_encode(&self.input, self.format, self.no_padding)?;
        println!("\n{}", decoded);
        Ok(())
    }
}
