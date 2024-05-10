use std::io::Read;

pub trait Encrypt {
    /// Encrypt the text from the reader, then base64 the encrypted text and return it.
    fn enscypt(
        &self,
        key: &[u8; 32],
        nonce: &[u8; 12],
        reader: &mut dyn Read,
    ) -> anyhow::Result<Vec<u8>>;
}
