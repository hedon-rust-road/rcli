use clap::Parser;
use colored::*;
use zxcvbn::zxcvbn;

use crate::{process, CmdExector};

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = false)]
    pub no_uppercase: bool,

    #[arg(long, default_value_t = false)]
    pub no_lowercase: bool,

    #[arg(long, default_value_t = false)]
    pub no_number: bool,

    #[arg(long, default_value_t = false)]
    pub no_symbol: bool,
}

impl CmdExector for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let password = process::gen_pass::process_genpass(
            self.length,
            !self.no_uppercase,
            !self.no_lowercase,
            !self.no_number,
            !self.no_symbol,
        )?;
        let score = zxcvbn(&password, &[]).unwrap().score();
        println!(
            "Successfully generate password: {}, strength score is: {}",
            password.bright_cyan(),
            score.to_string().red(),
        );
        Ok(())
    }
}
