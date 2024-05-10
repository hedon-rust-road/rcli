use std::io::Read;

pub trait Decrypt {
    /// Decrypt the base64 data from the reader and return the origin text.
    fn decrypt(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}
