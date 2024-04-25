use clap::Parser;
use rcli::{opts, process::process_csv};

fn main() -> anyhow::Result<()> {
    let opts = opts::Opts::parse();
    match opts.cmd {
        opts::SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output)?,
    }
    Ok(())
}
