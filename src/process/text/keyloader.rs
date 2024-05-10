use std::path::Path;

pub trait KeyLoader {
    /// Load key from path
    fn load(path: impl AsRef<Path>) -> anyhow::Result<Self>
    where
        Self: Sized;
}
