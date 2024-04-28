use anyhow::{anyhow, Ok};
use chrono::{prelude::Local, TimeZone};
use colored::*;

pub fn process_unix_to_string(unix: i64) -> anyhow::Result<()> {
    if unix == 0 {
        let now = Local::now();
        println!(
            "{}: {}",
            now.timestamp().to_string().yellow(),
            now.format("%Y-%m-%d %H:%M:%S %Z").to_string().purple()
        );
        return Ok(());
    }

    let t = Local.timestamp_opt(unix, 0).single();
    if t.is_none() {
        return Err(anyhow!(format!("invalid unix: {}", unix)));
    }
    println!(
        "{}: {}",
        unix.to_string().yellow(),
        t.unwrap().to_string().purple()
    );
    Ok(())
}
