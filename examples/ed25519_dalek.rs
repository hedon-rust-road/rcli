use ed25519_dalek::ed25519::signature::SignerMut;
use ed25519_dalek::{Signature, SigningKey};
use ed25519_dalek::{Verifier, VerifyingKey};
use rand::rngs::OsRng;

fn main() {
    let mut cspring = OsRng;

    // private key
    let mut signing_key: SigningKey = SigningKey::generate(&mut cspring);

    // sign
    let message: &[u8] = b"This is a test of the tsunami alert system.";
    let signature: Signature = signing_key.sign(message);
    println!("{}", signature);

    // use private key to verify
    assert!(signing_key.verify(message, &signature).is_ok());

    // generate public key and use public key to verify
    let verifying_key: VerifyingKey = signing_key.verifying_key();
    assert!(verifying_key.verify(message, &signature).is_ok());
}
