fn main() {}
#[cfg(test)]
mod tests {
    use anyhow::Ok;
    use base64::{prelude::*, Engine};

    #[test]
    fn test_standard_encode() {
        assert_eq!(
            BASE64_STANDARD.encode(b"Hedon Wang"),
            "SGVkb24gV2FuZw==".to_string()
        );
    }

    #[test]
    fn test_standard_decode() -> anyhow::Result<()> {
        assert_eq!(BASE64_STANDARD.decode(b"SGVkb24gV2FuZw==")?, b"Hedon Wang");
        Ok(())
    }

    #[test]
    fn test_standard_no_pad_encode() {
        assert_eq!(
            BASE64_STANDARD_NO_PAD.encode(b"Hedon Wang"),
            "SGVkb24gV2FuZw".to_string()
        );
    }

    #[test]
    fn test_standard_no_pad_decode() -> anyhow::Result<()> {
        assert_eq!(
            BASE64_STANDARD_NO_PAD.decode(b"SGVkb24gV2FuZw")?,
            b"Hedon Wang"
        );
        Ok(())
    }

    #[test]
    fn test_url_safe_encode() {
        assert_eq!(
            BASE64_URL_SAFE.encode(b"Hedon/Wang"),
            "SGVkb24vV2FuZw==".to_string()
        );
    }

    #[test]
    fn test_url_safe_decode() -> anyhow::Result<()> {
        assert_eq!(BASE64_URL_SAFE.decode(b"SGVkb24vV2FuZw==")?, b"Hedon/Wang");
        Ok(())
    }

    #[test]
    fn test_url_safe_no_pad_encode() {
        assert_eq!(
            BASE64_URL_SAFE_NO_PAD.encode(b"Hedon/Wang"),
            "SGVkb24vV2FuZw".to_string()
        );
    }

    #[test]
    fn test_url_safe_no_pad_decode() -> anyhow::Result<()> {
        assert_eq!(
            BASE64_URL_SAFE_NO_PAD.decode(b"SGVkb24vV2FuZw")?,
            b"Hedon/Wang"
        );
        Ok(())
    }
}
