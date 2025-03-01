#[cfg(test)]
mod tests {
    use i18n::{get_message, set_language_from_accept_language, language_loader, Localizations};
    use unic_langid::LanguageIdentifier;
    use std::str::FromStr;
    use i18n_embed::select;
    use std::sync::Mutex;
    use once_cell::sync::Lazy;

    // Use a mutex to ensure tests don't run in parallel
    static TEST_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

    #[test]
    fn test_middleware_sets_language() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

        // This is a simplified test that directly tests the language detection functionality
        // without using the middleware, since the middleware just calls set_language_from_accept_language

        // Reset to English
        let english = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(language_loader(), &Localizations, &[english.clone()]);
        assert_eq!(get_message("test-message"), "This is a test message");

        // Test with Spanish Accept-Language header
        let _ = set_language_from_accept_language("es");
        assert_eq!(get_message("test-message"), "Este es un mensaje de prueba");

        // Test with French Accept-Language header
        let _ = set_language_from_accept_language("fr-FR,fr;q=0.9,en;q=0.8");
        assert_eq!(get_message("test-message"), "Ceci est un message de test");

        // Reset to English before testing empty header
        let english_before_empty = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(language_loader(), &Localizations, &[english_before_empty]);

        // Test with no Accept-Language header (should default to English)
        let _ = set_language_from_accept_language("");
        assert_eq!(get_message("test-message"), "This is a test message");

        // Reset to English for other tests
        let english_reset = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(language_loader(), &Localizations, &[english_reset]);
    }
}
