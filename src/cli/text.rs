use std::{fs, path::PathBuf};

use anyhow::Ok;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::{Parser, ValueEnum};
use enum_dispatch::enum_dispatch;

use crate::{
    cli::{verify_dir, verify_file},
    process::{
        get_input, process_text_gen_key, process_text_sign, process_text_verify,
        text::{process_decrypt_text, process_encrypt_text},
    },
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
    GenKey(TextGenKeyOpts),
    #[command(about = "Encrypt text with chacha20poly1305 and output in base64")]
    Encrypt(TextEncryptOpts),
    #[command(about = "Decrypt the base64 text with chacha20poly1305")]
    Decrypt(TextDecryptOpts),
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
pub struct TextGenKeyOpts {
    #[arg(long, value_enum, default_value_t = TextSignFormat::Blake3)]
    pub format: TextSignFormat,
    #[arg(short, long, value_parser = verify_dir)]
    pub output: PathBuf,
}

#[derive(Debug, Parser)]
pub struct TextEncryptOpts {
    /// The key for chacha30poly1305, if not specifies,
    /// it would be randomly generated than set at the head of the response.
    #[arg(long, default_value = "-")]
    pub key: String,
    /// The text to encrypt,
    /// support both stdin[-] and file.
    #[arg(value_parser = verify_file, default_value = "-")]
    pub text: String,
}

#[derive(Debug, Parser)]
pub struct TextDecryptOpts {
    /// The key for chacha30poly1305,
    /// if not specifies means it's in the head of the text.
    #[arg(long, default_value = "-")]
    pub key: String,
    /// The base64 text to decrypt,
    /// support both stdin[-] and file.
    #[arg(value_parser = verify_file, default_value = "-")]
    pub text: String,
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

impl CmdExector for TextGenKeyOpts {
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

impl CmdExector for TextEncryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let text = get_input(&self.text)?;
        println!(
            "\n{}",
            process_encrypt_text(self.key.as_bytes(), text.as_slice())?
        );
        Ok(())
    }
}

impl CmdExector for TextDecryptOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let text = get_input(&self.text)?;
        println!(
            "\n{}",
            process_decrypt_text(self.key.as_bytes(), text.as_slice())?
        );
        Ok(())
    }
}
