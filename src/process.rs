use std::fs;

use serde_json::Value;

pub fn process_csv(input: &str, output: &str) -> anyhow::Result<()> {
    let mut reader = csv::Reader::from_path(input)?;
    let mut res = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for record in reader.records() {
        let record = record?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        res.push(json_value);
    }
    let res = serde_json::to_string_pretty(&res)?;
    fs::write(output, res)?;
    Ok(())
}
