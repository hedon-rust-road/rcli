pub mod blake3;
pub mod decrypt;
pub mod ed25519;
pub mod encrypt;
pub mod keygenerator;
pub mod keyloader;
pub mod sign;
pub mod verify;

use anyhow::{anyhow, Ok};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use colored::*;

use crate::{cli::TextSignFormat, utils::get_reader};

use self::{
    blake3::Blake3,
    ed25519::{Ed25519Signer, Ed25519Verifier},
    keygenerator::KeyGenerator,
    keyloader::KeyLoader,
    sign::TextSign,
    verify::TextVerify,
};

const KEY_LENGTH: usize = 32;
const SIG_LENGTH: usize = 64;

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

pub fn process_encrypt_text(key: String, nonce: String, text: &[u8]) -> anyhow::Result<String> {
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key.as_bytes()));
    let nonce = Nonce::from_slice(nonce.as_bytes()); // 96-bits; unique per message
    let ciphertext = cipher
        .encrypt(nonce, text)
        .map_err(|e| anyhow!("encrypt error: {}", e))?;
    Ok(URL_SAFE_NO_PAD.encode(ciphertext))
}

pub fn process_decrypt_text(key: String, nonce: String, text: &[u8]) -> anyhow::Result<String> {
    let binary = URL_SAFE_NO_PAD.decode(text)?;
    let cipher = ChaCha20Poly1305::new(Key::from_slice(key.as_bytes()));
    let nonce = Nonce::from_slice(nonce.as_bytes()); // 96-bits; unique per message
    let res = cipher
        .decrypt(nonce, binary.as_ref())
        .map_err(|e| anyhow!("decrypt error: {}", e))?;
    Ok(String::from_utf8(res)?)
}

#[cfg(test)]
mod tests {
    use crate::process::text::{process_encrypt_text, TextVerify};

    use super::{process_decrypt_text, Blake3, Ed25519Signer, Ed25519Verifier, TextSign};

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
        let sk = include_bytes!("../../../fixtures/ed25519.sk");
        let pk = include_bytes!("../../../fixtures/ed25519.pk");

        let signer = Ed25519Signer::try_new(sk)?;
        let verifier = Ed25519Verifier::try_new(pk)?;
        let signed_res = signer.sign(&mut &data[..])?;
        assert!(verifier.verify(&mut &data[..], &signed_res)?);
        Ok(())
    }

    #[test]
    fn test_encrypt_decrypt() -> anyhow::Result<()> {
        let key = "noncenoncenoncenoncenoncenonce11".to_string();
        let nonce = "noncenonce11".to_string();
        let res = process_encrypt_text(key.clone(), nonce.clone(), b"1")?;
        println!("{}", res);
        let res = process_decrypt_text(key.clone(), nonce.clone(), res.as_bytes())?;
        println!("{}", res);
        Ok(())
    }
}
