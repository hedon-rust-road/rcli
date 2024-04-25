use std::fs;

use clap::{Parser, Subcommand};
use rcli::player::Player;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

/// rcli csv -i input.csv -o output.json --header -d ','
#[derive(Debug, Subcommand)]
enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    /// Input file path
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,

    /// Output file path
    #[arg(short, long, default_value = "output.json")]
    output: String,

    /// CSV has header or not
    #[arg(long, default_value_t = false)]
    header: bool,

    /// Delimiter
    #[arg(short, long, default_value_t = ',')]
    delimiter: char,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File dose not exists")
    }
}

fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = csv::Reader::from_path(opts.input)?;

            let players: Vec<Player> = reader.deserialize::<Player>().map(|r| r.unwrap()).collect();
            let res = serde_json::to_string_pretty(&players)?;
            fs::write(opts.output, res)?;
        }
    }
    Ok(())
}
