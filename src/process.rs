use std::fs;

use serde_json::Value;

use crate::opts::OutputFormat;

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> anyhow::Result<()> {
    let mut reader = csv::Reader::from_path(input)?;
    let mut res = Vec::with_capacity(128);
    let headers = reader.headers()?.clone();
    for record in reader.records() {
        let record = record?;
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        res.push(json_value);
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&res)?,
        OutputFormat::Yaml => serde_yaml::to_string(&res)?,
    };

    fs::write(output, content)?;
    Ok(())
}
