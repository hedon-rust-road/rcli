use clap::Parser;

#[derive(Debug, Parser)]
pub struct TimeOpts {
    /// Specify the unix to format, if 0, means now.
    #[arg(default_value_t = 0)]
    pub unix: u64,
}
