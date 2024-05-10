use std::{fs, io::Read, path::Path};

use super::{
    keygenerator::KeyGenerator, keyloader::KeyLoader, sign::TextSign, verify::TextVerify,
    KEY_LENGTH, SIG_LENGTH,
};
use anyhow::anyhow;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        let res = self.key.sign(&data);
        Ok(res.to_bytes().to_vec())
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>> {
        let mut res = vec![];

        let mut cspring = OsRng;
        let private_key: SigningKey = SigningKey::generate(&mut cspring);
        let public_key: VerifyingKey = private_key.verifying_key();

        res.push(private_key.as_bytes().to_vec());
        res.push(public_key.as_bytes().to_vec());
        Ok(res)
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, mut reader: impl Read, sig: &[u8]) -> anyhow::Result<bool> {
        if sig.len() < SIG_LENGTH {
            return Err(anyhow!(format!(
                "[invalid signature] the length of `sig` cannot be less than 64"
            )));
        }
        let sig = &sig[..SIG_LENGTH];
        let signature = Signature::from_bytes(sig.try_into()?);

        let mut data = Vec::new();
        reader.read_to_end(&mut data)?;
        let res = self.key.verify(&data, &signature).is_ok();
        Ok(res)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        if key.len() < KEY_LENGTH {
            return Err(anyhow!(format!(
                "[invalid key] the length of `key` cannot be less than {KEY_LENGTH}"
            )));
        }
        let key = &key[..KEY_LENGTH];
        let key = key.try_into()?;
        let key = SigningKey::from_bytes(key);
        Ok(Self { key })
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        if key.len() < KEY_LENGTH {
            return Err(anyhow!(format!(
                "[invalid key] the length of `key` cannot be less than {KEY_LENGTH}"
            )));
        }
        let key = &key[..KEY_LENGTH];
        let key = key.try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        Ok(Self { key })
    }
}
