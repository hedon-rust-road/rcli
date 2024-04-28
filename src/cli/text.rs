use std::path::PathBuf;

use clap::{Parser, ValueEnum};

use crate::cli::{verify_dir, verify_file};

#[derive(Debug, Parser)]
pub enum TextSubCommand {
    #[command(about = "Sign a message with a private key")]
    Sign(TextSignOpts),
    #[command(about = "Veriy a signed message with a private/public key")]
    Verify(TextVerifyOpts),
    #[command(name = "genkey", about = "Generate a pair of key")]
    GenKey(GenKeyOpts),
}

#[derive(Debug, Parser)]
pub struct TextSignOpts {
    /// Message to be signed
    #[arg(short, long, value_parser = verify_file,  default_value = "-")]
    pub input: String,
    /// Private key
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    /// Sign format
    #[arg(long, value_enum, default_value_t = TextSignFormat::Blake3)]
    pub format: TextSignFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    /// Provide message to be verified
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    /// Provide private or public key to verify
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    /// Sign format
    #[arg(long, value_enum, default_value_t = TextSignFormat::Blake3)]
    pub format: TextSignFormat,
    /// Signature
    #[arg(long)]
    pub sig: String,
}

#[derive(Debug, Parser)]
pub struct GenKeyOpts {
    #[arg(long, value_enum, default_value_t = TextSignFormat::Blake3)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_dir)]
    pub output: PathBuf,
}
