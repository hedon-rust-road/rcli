use std::io::Read;

pub trait Decrypt {
    /// Decrypt the base64 data from the reader and return the origin text.
    fn descypt(
        &self,
        key: &[u8; 32],
        nonce: &[u8; 12],
        reader: &mut dyn Read,
    ) -> anyhow::Result<Vec<u8>>;
}
