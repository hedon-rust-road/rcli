use std::{fs::File, io::Read};

pub fn get_reader(input: &str) -> anyhow::Result<Box<dyn Read>> {
    let reader: Box<dyn Read> = match input {
        "-" => Box::new(std::io::stdin()),
        filename => Box::new(File::open(filename)?),
    };
    Ok(reader)
}
