use clap::Parser;
use colored::*;
use rcli::{opts, process};
use zxcvbn::zxcvbn;

fn main() -> anyhow::Result<()> {
    let opts = opts::Opts::parse();
    match opts.cmd {
        opts::SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output
            } else {
                format!("output.{}", opts.format)
            };
            process::csv_convert::process_csv(&opts.input, &output, opts.format)?
        }
        opts::SubCommand::GenPass(opts) => {
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
    }
    Ok(())
}
