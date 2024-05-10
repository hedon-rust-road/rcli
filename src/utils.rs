use std::{fs::File, io::Read};

use colored::Colorize;

pub fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = match input {
        "-" => Box::new(std::io::stdin()),
        filename => Box::new(File::open(filename)?),
    };
    Ok(reader)
}

pub fn print_verify_result(res: bool) {
    if res {
        println!("\n{}", "√ Signature verified".green())
    } else {
        println!("\n{}", "x Signature not verified".red())
    }
}
