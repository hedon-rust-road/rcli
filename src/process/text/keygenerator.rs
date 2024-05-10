pub trait KeyGenerator {
    fn generate() -> anyhow::Result<Vec<Vec<u8>>>;
}
