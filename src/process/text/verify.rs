use std::io::Read;

pub trait TextVerify {
    /// Verify the data from the reader with the signature.
    fn verify(&self, reader: impl Read, sig: &[u8]) -> anyhow::Result<bool>;
}
