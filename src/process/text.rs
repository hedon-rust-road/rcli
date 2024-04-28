use std::{fs, io::Read, path::Path};

use anyhow::{anyhow, Ok};
use colored::*;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;

use crate::{cli::TextSignFormat, process::process_genpass, utils::get_reader};

const KEY_LENGTH: usize = 32;
const SIG_LENGTH: usize = 64;

pub trait TextSign {
    /// Sign the data from the reader and return the signature.
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}

pub trait TextVerify {
    /// Verify the data from the reader with the signature.
    fn verify(&self, reader: impl Read, sig: &[u8]) -> anyhow::Result<bool>;
}

pub trait KeyLoader {
    /// Load key from path
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized;
}

pub trait KeyGenerator {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>>;
}

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
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

impl Blake3 {
    pub fn new(key: [u8; KEY_LENGTH]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        if key.len() < KEY_LENGTH {
            return Err(anyhow!(
                "[invalid key] the length of `key` cannot be less than 32"
            ));
        }
        let key = &key[..KEY_LENGTH];
        let key = key.try_into().unwrap();
        let blake3 = Blake3 { key };
        Ok(blake3)
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

pub fn process_text_sign(
    input: &str,
    key: &str,
    format: TextSignFormat,
) -> anyhow::Result<Vec<u8>> {
    let mut reader = get_reader(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => Blake3::load(key)?.sign(&mut reader)?,
        TextSignFormat::Ed25519 => Ed25519Signer::load(key)?.sign(&mut reader)?,
    };
    Ok(signed)
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    format: TextSignFormat,
    sig: &[u8],
) -> anyhow::Result<()> {
    let mut reader = get_reader(input)?;
    let res = match format {
        TextSignFormat::Blake3 => Blake3::load(key)?.verify(&mut reader, sig)?,
        TextSignFormat::Ed25519 => Ed25519Verifier::load(key)?.verify(&mut reader, sig)?,
    };

    if res {
        println!("\n{}", "√ Signature verified".green())
    } else {
        println!("\n{}", "x Signature not verified".red())
    }

    Ok(())
}

pub fn process_text_gen_key(format: TextSignFormat) -> anyhow::Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
    }
}

#[cfg(test)]
mod tests {
    use crate::process::text::TextVerify;

    use super::{Blake3, Ed25519Signer, Ed25519Verifier, TextSign};

    #[test]
    fn test_blake3_sign_verify() -> anyhow::Result<()> {
        let blake3 = Blake3::try_new(b"@upMihMX^VfVVzYBs$BgQT*MqUMLj9up")?;

        let data = b"hello world";
        let signed_res = blake3.sign(&mut &data[..])?;
        assert!(blake3.verify(&mut &data[..], &signed_res)?);
        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> anyhow::Result<()> {
        let data = b"hello world";
        let sk = include_bytes!("../../fixtures/ed25519.sk");
        let pk = include_bytes!("../../fixtures/ed25519.pk");

        let signer = Ed25519Signer::try_new(sk)?;
        let verifier = Ed25519Verifier::try_new(pk)?;
        let signed_res = signer.sign(&mut &data[..])?;
        assert!(verifier.verify(&mut &data[..], &signed_res)?);
        Ok(())
    }
}
