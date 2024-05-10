use std::{fs, io::Read, path::Path};

use anyhow::anyhow;

use crate::process::process_genpass;

use super::{
    keygenerator::KeyGenerator, keyloader::KeyLoader, sign::TextSign, verify::TextVerify,
    KEY_LENGTH,
};

pub struct Blake3 {
    key: [u8; 32],
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let expected = blake3::keyed_hash(&self.key, &buf);
        let expected = expected.as_bytes();
        Ok(expected == sig)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let mut res = vec![];
        let pass = process_genpass(32, true, true, true, true)?;
        res.push(pass.as_bytes().to_vec());
        Ok(res)
    }
}

impl Blake3 {
    pub fn new(key: [u8; KEY_LENGTH]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        if key.len() < KEY_LENGTH {
            return Err(anyhow!(
                "[invalid key] the length of `key` cannot be less than {KEY_LENGTH}"
            ));
        }
        let key = &key[..KEY_LENGTH];
        let key = key.try_into().unwrap();
        let blake3 = Blake3 { key };
        Ok(blake3)
    }
}
