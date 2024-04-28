use clap::Parser;
use colored::*;
use rcli::{
    cli::{self, Base64SubCommand},
    process::{
        self,
        b64::{process_decode, process_encode},
        time::process_unix_to_string,
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
            eprintln!(
                "Successfully generate password: {}, strength score is: {}",
                password.bright_cyan(),
                score.to_string().red(),
            )
        }
        cli::SubCommand::Base64(opts) => match opts {
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format, opts.no_padding)?
            }
            Base64SubCommand::Encode(opts) => {
                process_encode(&opts.input, opts.format, opts.no_padding)?
            }
        },
        cli::SubCommand::Time(opts) => process_unix_to_string(opts.unix as i64)?,
    }
    Ok(())
}
