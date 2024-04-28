fn main() {
    let hash1 = blake3::hash(b"foobarbaz");

    let mut hasher = blake3::Hasher::new();
    hasher.update(b"foo");
    hasher.update(b"bar");
    hasher.update(b"baz");
    let hash2 = hasher.finalize();
    assert_eq!(hash1, hash2);

    let mut output = [0; 1000];
    let mut output_reader = hasher.finalize_xof();
    output_reader.fill(&mut output);
    assert_eq!(hash1, output[..32]);

    println!("{}", hash1);

    let example_key = [42u8; 32];
    let mac1 = blake3::keyed_hash(&example_key, b"foobarbaz");

    let mut hasher = blake3::Hasher::new_keyed(&example_key);
    hasher.update(b"foobarbaz");
    let mac2 = hasher.finalize();
    assert_eq!(mac1, mac2);
    println!("{}", mac1);

    const EMAIL_CONTEXT: &str = "BLAKE3 example 2020-01-07 17:10:44 email key";
    const API_CONTEXT: &str = "BLAKE3 example 2020-01-07 17:11:21 API key";
    let email_key = blake3::derive_key(EMAIL_CONTEXT, &example_key);
    let api_key = blake3::derive_key(API_CONTEXT, &example_key);
    assert_ne!(email_key, api_key);
}
