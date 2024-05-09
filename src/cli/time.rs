use clap::Parser;

use crate::{process::process_unix_to_string, CmdExector};

#[derive(Debug, Parser)]
pub struct TimeOpts {
    /// Specify the unix to format, if 0, means now.
    #[arg(default_value_t = 0)]
    pub unix: u64,
}

impl CmdExector for TimeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        process_unix_to_string(self.unix as i64)
    }
}
