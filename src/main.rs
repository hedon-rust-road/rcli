use clap::Parser;
use colored::*;
use rcli::{
    cli::{self},
    process,
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
    }
    Ok(())
}
