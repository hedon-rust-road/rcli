use std::{
    collections::BTreeMap,
    time::{SystemTime, UNIX_EPOCH},
};

use colored::Colorize;
use hmac::{Hmac, Mac};
use jwt::{Header, SignWithKey, Token, VerifyWithKey};
use sha2::Sha256;

pub struct JwtSigner {}
pub struct JwtVerifier {}

pub fn process_jwt_sign(key: &[u8], sub: String, aud: String, exp: u64) -> anyhow::Result<String> {
    let key = get_key(key)?;
    let header = Header {
        algorithm: jwt::AlgorithmType::Hs256,
        ..Default::default()
    };

    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

    let mut claims = BTreeMap::new();
    claims.insert("sub", sub);
    claims.insert("aud", aud);
    claims.insert("exp", (now + exp).to_string());

    let token = Token::new(header, claims).sign_with_key(&key)?;
    Ok(token.into())
}

pub fn process_jwt_verify(key: &[u8], jwt: String) -> anyhow::Result<bool> {
    let key = get_key(key)?;
    let token: Result<Token<Header, BTreeMap<String, String>, _>, jwt::Error> =
        jwt.verify_with_key(&key);
    let res = match token {
        Ok(token) => {
            println!("{}", format!("\n\n{:?}", token.header()).purple());
            println!("{}", format!("Claims {:?}", token.claims()).purple());
            true
        }
        Err(_) => false,
    };
    Ok(res)
}

fn get_key(key: &[u8]) -> anyhow::Result<Hmac<Sha256>> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(key)?;
    Ok(key)
}
