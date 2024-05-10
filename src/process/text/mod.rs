pub mod blake3;
pub mod chacha20_poly1305;
pub mod decrypt;
pub mod ed25519;
pub mod encrypt;
pub mod keygenerator;
pub mod keyloader;

use anyhow::{anyhow, Ok};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use colored::*;

use crate::{cli::TextSignFormat, utils::get_reader};

use self::{
    blake3::Blake3,
    chacha20_poly1305::{
        Chacha20Poly1305Decryptor, Chacha20Poly1305Encryptor, NONCE_LENGTH, RAND_FLAG,
    },
    decrypt::Decrypt,
    ed25519::{Ed25519Signer, Ed25519Verifier},
    encrypt::Encrypt,
    keygenerator::KeyGenerator,
    keyloader::KeyLoader,
};

use super::{sign::TextSign, verify::TextVerify};

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

pub fn process_encrypt_text(key: &[u8], mut text: &[u8]) -> anyhow::Result<String> {
    let en = Chacha20Poly1305Encryptor::try_new(key)?;
    let mut res = en.encrypt(&mut text)?;
    if key == RAND_FLAG {
        let mut tmp = en.get_key();
        tmp.extend(res);
        res = tmp;
    }
    Ok(URL_SAFE_NO_PAD.encode(res))
}

pub fn process_decrypt_text(key: &[u8], text: &[u8]) -> anyhow::Result<String> {
    let binary = URL_SAFE_NO_PAD.decode(text)?;
    let de;
    let res;
    if key == RAND_FLAG {
        if binary.len() <= KEY_LENGTH + NONCE_LENGTH {
            return Err(anyhow!("[invalid text] not contains key or nonce"));
        }
        de = Chacha20Poly1305Decryptor::try_new(&binary[..KEY_LENGTH])?;
        let mut tmp = &binary[KEY_LENGTH..];
        res = de.decrypt(&mut tmp)?;
    } else {
        de = Chacha20Poly1305Decryptor::try_new(key)?;
        res = de.decrypt(&mut binary.as_slice())?;
    }

    Ok(String::from_utf8(res)?)
}

#[cfg(test)]
mod tests {
    use crate::process::{
        sign::TextSign,
        text::{chacha20_poly1305::RAND_FLAG, process_encrypt_text},
        verify::TextVerify,
    };

    use super::{process_decrypt_text, Blake3, Ed25519Signer, Ed25519Verifier};

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
        let key = b"noncenoncenoncenoncenoncenonce11";
        let res = process_encrypt_text(key, b"1")?;
        println!("{}", res);
        let res = process_decrypt_text(key, res.as_bytes())?;
        println!("{}", res);
        Ok(())
    }

    #[test]
    fn test_encrypt_decrypt_without_key() -> anyhow::Result<()> {
        let key = RAND_FLAG;
        let res = process_encrypt_text(key, b"1")?;
        println!("{}", res);
        let res = process_decrypt_text(key, res.as_bytes())?;
        println!("{}", res);
        Ok(())
    }
}
