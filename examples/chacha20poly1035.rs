use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
fn main() {
    let key = Key::from_slice(b"an example very very secret key."); // 32 bytes
    let cipher = ChaCha20Poly1305::new(key);

    let nonce = Nonce::from_slice(b"unique nonce"); // 12-byte nonce

    let ciphertext = cipher
        .encrypt(nonce, b"plaintext message".as_ref())
        .expect("encryption failure!");

    let plaintext = cipher.decrypt(nonce, ciphertext.as_ref());

    match plaintext {
        Ok(pt) => println!("Decrypted text: {:?}", std::str::from_utf8(&pt)),
        Err(e) => println!("Error: decrypt error: {:?}", e),
    }
}
