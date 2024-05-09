use std::{fs, path::PathBuf};

use anyhow::Ok;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::{Parser, ValueEnum};
use enum_dispatch::enum_dispatch;

use crate::{
    cli::{verify_dir, verify_file},
    process::{process_text_gen_key, process_text_sign, process_text_verify},
    CmdExector,
};

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
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

impl CmdExector for TextSignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let sig = process_text_sign(&self.input, &self.key, self.format)?;
        println!("\n{}", URL_SAFE_NO_PAD.encode(sig));
        Ok(())
    }
}

impl CmdExector for TextVerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let sig = URL_SAFE_NO_PAD.decode(&self.sig)?;
        process_text_verify(&self.input, &self.key, self.format, &sig)?;
        Ok(())
    }
}

impl CmdExector for GenKeyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let key = process_text_gen_key(self.format)?;
        match self.format {
            TextSignFormat::Blake3 => {
                let filename = &self.output.join("blake3.txt");
                fs::write(filename, &key[0])?;
            }
            TextSignFormat::Ed25519 => {
                let dir = &self.output;
                fs::write(dir.join("ed25519.sk"), &key[0])?;
                fs::write(dir.join("ed25519.pk"), &key[1])?;
            }
        }
        Ok(())
    }
}
