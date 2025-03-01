#[cfg(test)]
mod tests {
    use crate::i18n::{language_loader, get_message};
    use unic_langid::LanguageIdentifier;
    use std::str::FromStr;
    use i18n_embed::select;

    #[test]
    fn test_language_loader_initialization() {
        let loader = language_loader();
        assert!(loader.has_language(&LanguageIdentifier::from_str("en").unwrap()));
    }

    #[test]
    fn test_default_language() {
        // Default should be English
        assert_eq!(get_message("test-message"), "This is a test message");
    }

    #[test]
    fn test_missing_message_returns_message_id() {
        assert_eq!(get_message("non-existent-message"), "non-existent-message");
    }
}
