use std::fs::{self};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use clap::Parser;
use colored::*;
use rcli::{
    cli::{self, Base64SubCommand, TextSubCommand},
    process::{
        self, process_decode, process_encode, process_text_gen_key, process_text_sign,
        process_text_verify, process_unix_to_string,
    },
};
use zxcvbn::zxcvbn;

fn main() -> anyhow::Result<()> {
    let opts = cli::Opts::parse();
    match opts.cmd {
        cli::SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output
            } else {
                format!("output.{}", opts.format)
            };
            process::csv_convert::process_csv(&opts.input, &output, opts.format)?
        }
        cli::SubCommand::GenPass(opts) => {
            let password = process::gen_pass::process_genpass(
                opts.length,
                !opts.no_uppercase,
                !opts.no_lowercase,
                !opts.no_number,
                !opts.no_symbol,
            )?;
            let score = zxcvbn(&password, &[]).unwrap().score();
            println!(
                "Successfully generate password: {}, strength score is: {}",
                password.bright_cyan(),
                score.to_string().red(),
            )
        }
        cli::SubCommand::Base64(cmd) => match cmd {
            Base64SubCommand::Decode(opts) => {
                let encoded = process_decode(&opts.input, opts.format, opts.no_padding)?;
                println!("\n{}", String::from_utf8(encoded)?);
            }
            Base64SubCommand::Encode(opts) => {
                let decoded = process_encode(&opts.input, opts.format, opts.no_padding)?;
                println!("\n{}", decoded);
            }
        },
        cli::SubCommand::Time(opts) => process_unix_to_string(opts.unix as i64)?,
        cli::SubCommand::Text(cmd) => match cmd {
            TextSubCommand::Sign(opts) => {
                let sig = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("\n{}", URL_SAFE_NO_PAD.encode(sig));
            }
            TextSubCommand::Verify(opts) => {
                let sig = URL_SAFE_NO_PAD.decode(&opts.sig)?;
                process_text_verify(&opts.input, &opts.key, opts.format, &sig)?
            }
            TextSubCommand::GenKey(opts) => {
                let key = process_text_gen_key(opts.format)?;
                match opts.format {
                    rcli::TextSignFormat::Blake3 => {
                        let filename = &opts.output.join("blake3.txt");
                        fs::write(filename, &key[0])?;
                    }
                    rcli::TextSignFormat::Ed25519 => {
                        let dir = &opts.output;
                        fs::write(dir.join("ed25519.sk"), &key[0])?;
                        fs::write(dir.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
        },
    }
    Ok(())
}
