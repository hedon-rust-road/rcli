use std::io::Read;

use anyhow::Ok;
use base64::engine::GeneralPurpose;
use base64::prelude::*;
use base64::Engine;

use crate::cli::base64::Base64Format;
use crate::utils;

pub fn process_encode(
    input: &str,
    format: Base64Format,
    no_padding: bool,
) -> anyhow::Result<String> {
    let buf = get_input(input)?;
    let engine = get_decode_engine(format, no_padding);
    let res = engine.encode(buf);
    Ok(res)
}

pub fn process_decode(
    input: &str,
    format: Base64Format,
    no_padding: bool,
) -> anyhow::Result<Vec<u8>> {
    let buf = get_input(input)?;
    let engine = get_decode_engine(format, no_padding);
    let res = engine.decode(buf)?;
    Ok(res)
}

fn get_input(input: &str) -> anyhow::Result<Vec<u8>> {
    let mut reader = utils::get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}

fn get_decode_engine(format: Base64Format, no_padding: bool) -> GeneralPurpose {
    match format {
        Base64Format::Standard => match no_padding {
            true => BASE64_STANDARD_NO_PAD,
            false => BASE64_STANDARD,
        },
        Base64Format::Urlsafe => match no_padding {
            true => BASE64_URL_SAFE_NO_PAD,
            false => BASE64_URL_SAFE,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format, false).is_ok());
    }

    #[test]
    fn test_process_decode() {
        let input = "fixtures/b64.txt";
        let format = Base64Format::Urlsafe;
        process_decode(input, format, true).unwrap();
    }
}
