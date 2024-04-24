// rcli csv -i input.csv -o output.json --header -d ','

use std::fs::{self, read};

use anyhow::Ok;
use clap::Parser;

use crate::player::Player;

mod player;

#[derive(Debug, Parser)]
struct Opts {
    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    output: String,

    #[arg(long, default_value_t = false)]
    header: bool,

    #[arg(short, long, default_value_t = ',')]
    deperator: char,
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();

    let f = fs::File::open(opts.input)?;
    let mut reader = csv::Reader::from_reader(f);

    for record in reader.deserialize() {
        let player: Player = record?;
        println!("{:?}", player);
    }

    Ok(())
}
