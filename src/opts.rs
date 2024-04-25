use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

/// rcli csv -i input.csv -o output.json --header -d ','
#[derive(Debug, Subcommand)]
pub enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
pub struct CsvOpts {
    /// Input file path
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    /// Output file path
    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    /// CSV has header or not
    #[arg(long, default_value_t = false)]
    pub header: bool,

    /// Delimiter
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("File dose not exists")
    }
}
