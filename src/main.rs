use clap::Parser;
use rcli::{cli, CmdExector};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = cli::Opts::parse();
    match opts.cmd {
        cli::SubCommand::Csv(opts) => opts.execute().await?,
        cli::SubCommand::GenPass(opts) => opts.execute().await?,
        cli::SubCommand::Base64(cmd) => cmd.execute().await?,
        cli::SubCommand::Time(opts) => opts.execute().await?,
        cli::SubCommand::Text(cmd) => cmd.execute().await?,
        cli::SubCommand::Http(cmd) => cmd.execute().await?,
    }
    Ok(())
}
