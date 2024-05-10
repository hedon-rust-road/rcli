pub mod b64;
pub mod csv_convert;
pub mod gen_pass;
pub mod http_serve;
pub mod jwt;
pub mod sign;
pub mod text;
pub mod time;
pub mod verify;

use std::io::Read;

pub use b64::{process_decode, process_encode};
pub use csv_convert::process_csv;
pub use gen_pass::process_genpass;
pub use http_serve::process_http_serve;
pub use text::{process_text_gen_key, process_text_sign, process_text_verify};
pub use time::process_unix_to_string;

use crate::utils;

pub fn get_input(input: &str) -> anyhow::Result<Vec<u8>> {
    let reader = utils::get_reader(input)?;
    get_content(reader)
}

pub fn get_content(mut reader: impl Read) -> anyhow::Result<Vec<u8>> {
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}
