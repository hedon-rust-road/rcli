use std::io::Read;

pub trait Encrypt {
    /// Encrypt the text from the reader, then base64 the encrypted text and return it.
    fn encrypt(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}
