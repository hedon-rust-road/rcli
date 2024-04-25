use clap::Parser;
use rcli::{opts, process::process_csv};

fn main() -> anyhow::Result<()> {
    let opts = opts::Opts::parse();
    match opts.cmd {
        opts::SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output
            } else {
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, &output, opts.format)?
        }
    }
    Ok(())
}
