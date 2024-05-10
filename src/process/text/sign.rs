use std::io::Read;

pub trait TextSign {
    /// Sign the data from the reader and return the signature.
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}
