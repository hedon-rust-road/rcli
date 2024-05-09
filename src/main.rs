use clap::Parser;
use rcli::{cli, CmdExector};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let opts = cli::Opts::parse();
    opts.cmd.execute().await?;
    Ok(())
}
