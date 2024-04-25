use clap::Parser;
use rcli::{opts, process};

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
        opts::SubCommand::GenPass(opts) => process::gen_pass::process_genpass(&opts)?,
    }
    Ok(())
}
