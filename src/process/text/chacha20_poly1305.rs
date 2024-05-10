use std::io::Read;

use crate::process::get_content;
use anyhow::anyhow;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};

use super::decrypt::Decrypt;
use super::encrypt::Encrypt;

pub const KEY_LENGTH: usize = 32;
pub const NONCE_LENGTH: usize = 12;
pub const RAND_FLAG: &[u8] = b"-";

pub struct Chacha20Poly1305Encryptor {
    key: Key,
    cipher: ChaCha20Poly1305,
}

pub struct Chacha20Poly1305Decryptor {
    cipher: ChaCha20Poly1305,
}

impl Encrypt for Chacha20Poly1305Encryptor {
    fn encrypt(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let buf = get_content(reader)?;
        let res = self
            .cipher
            .encrypt(&nonce, buf.as_slice())
            .map_err(|e| anyhow!("encrypt failed, {}", e))?;

        // nonce + encrypted_text
        let mut nonce = nonce.as_slice().to_vec();
        assert_eq!(nonce.len(), NONCE_LENGTH);
        nonce.extend(res);
        Ok(nonce)
    }
}

impl Chacha20Poly1305Decryptor {
    pub fn new(key: Key) -> Self {
        Self {
            cipher: ChaCha20Poly1305::new(&key),
        }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        Ok(Self::new(get_key(key)?))
    }
}

impl Chacha20Poly1305Encryptor {
    pub fn new(key: Key) -> Self {
        Self {
            key,
            cipher: ChaCha20Poly1305::new(&key),
        }
    }

    pub fn try_new(key: &[u8]) -> anyhow::Result<Self> {
        Ok(Self::new(get_key(key)?))
    }

    pub fn get_key(&self) -> Vec<u8> {
        self.key.to_vec()
    }
}

impl Decrypt for Chacha20Poly1305Decryptor {
    fn decrypt(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let content = get_content(reader)?;
        if content.len() <= NONCE_LENGTH {
            return Err(anyhow!("invalid input"));
        }

        // nonce + encrypted_text
        let nonce = &content[..NONCE_LENGTH];
        let content = &content[NONCE_LENGTH..];

        let nonce = Nonce::from_slice(nonce);
        let res = self
            .cipher
            .decrypt(nonce, content)
            .map_err(|e| anyhow!("decrypt failed, {}", e))?;
        Ok(res)
    }
}

fn get_key(key: &[u8]) -> anyhow::Result<Key> {
    if key == RAND_FLAG {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        return Ok(key);
    }
    if key.len() < KEY_LENGTH {
        return Err(anyhow!(
            "[invalid key] the length of `key` cannot be less than {KEY_LENGTH}"
        ));
    }
    let key = &key[..KEY_LENGTH];
    Ok(*Key::from_slice(key))
}
