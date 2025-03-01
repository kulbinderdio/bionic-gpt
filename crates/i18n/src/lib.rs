use i18n_embed::{
    fluent::{FluentLanguageLoader},
    LanguageLoader,
};
use rust_embed::RustEmbed;
use once_cell::sync::OnceCell;
use unic_langid::LanguageIdentifier;
use std::str::FromStr;
use std::collections::HashSet;

// Define the embedded assets
#[derive(RustEmbed)]
#[folder = "/workspace/i18n"]
pub struct Localizations;

// Create a singleton loader
static LANGUAGE_LOADER: OnceCell<FluentLanguageLoader> = OnceCell::new();

pub fn language_loader() -> &'static FluentLanguageLoader {
    LANGUAGE_LOADER.get_or_init(|| {
        let fallback = LanguageIdentifier::from_str("en").expect("Invalid language identifier");
        let loader = FluentLanguageLoader::new("web_ui", fallback);
        loader
            .load_fallback_language(&Localizations)
            .expect("Failed to load fallback language");
        loader
    })
}

// Basic translation function
pub fn get_message(message_id: &str) -> String {
    // Try to get the message, or return the message ID if not found
    let result = language_loader().get(message_id);
    if result.starts_with("No localization for id:") {
        message_id.to_string()
    } else {
        result
    }
}

// Translation function with arguments
pub fn get_message_with_args(message_id: &str, name: &str, count: &str) -> String {
    // First, get the message template
    let template = get_message(message_id);

    // If the template is the same as the message ID, it means the message was not found
    if template == message_id {
        return message_id.to_string();
    }

    // Replace {$name} with the provided name
    let with_name = template.replace("{$name}", name);

    // Replace {$count} with the provided count
    let with_count = with_name.replace("{$count}", count);

    with_count
}

// Translation macro
#[macro_export]
macro_rules! t {
    ($message_id:expr) => {
        $crate::get_message($message_id)
    };
    ($message_id:expr, name = $name:expr, count = $count:expr) => {
        $crate::get_message_with_args($message_id, &$name.to_string(), &$count.to_string())
    };
}

// Function to set language from Accept-Language header
pub fn set_language_from_accept_language(accept_language: &str) -> Result<(), Box<dyn std::error::Error>> {
    let loader = language_loader();

    let requested_languages = accept_language
        .split(',')
        .map(|lang| {
            let parts: Vec<&str> = lang.split(';').collect();
            parts[0].trim()
        })
        .filter_map(|lang_str| {
            LanguageIdentifier::from_str(lang_str).ok()
        })
        .collect::<Vec<_>>();

    if !requested_languages.is_empty() {
        i18n_embed::select(loader, &Localizations, &requested_languages)?;
    }

    Ok(())
}

// Function to get available languages
pub fn available_languages() -> HashSet<LanguageIdentifier> {
    let mut languages = HashSet::new();
    languages.insert(LanguageIdentifier::from_str("en").unwrap());
    languages.insert(LanguageIdentifier::from_str("es").unwrap());
    languages.insert(LanguageIdentifier::from_str("fr").unwrap());
    languages
}

#[cfg(test)]
mod tests {
    use super::{language_loader, get_message, get_message_with_args, set_language_from_accept_language, available_languages};
    use unic_langid::LanguageIdentifier;
    use std::str::FromStr;
    use i18n_embed::select;
    use std::sync::Mutex;
    use once_cell::sync::Lazy;

    // Use a mutex to ensure tests don't run in parallel
    static TEST_MUTEX: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

    #[test]
    fn test_language_loader_initialization() {
        // Just check that the loader can be initialized
        let _loader = language_loader();
    }

    #[test]
    fn test_default_language() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

        // Reset to English
        let english = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(language_loader(), &super::Localizations, &[english.clone()]);

        // Default should be English
        assert_eq!(get_message("test-message"), "This is a test message");
    }

    #[test]
    fn test_missing_message_returns_message_id() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

        // Reset to English
        let english = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(language_loader(), &super::Localizations, &[english.clone()]);

        assert_eq!(get_message("non-existent-message"), "non-existent-message");
    }

    #[test]
    fn test_available_languages() {
        let languages = available_languages();
        assert!(languages.contains(&LanguageIdentifier::from_str("en").unwrap()));
        assert!(languages.contains(&LanguageIdentifier::from_str("es").unwrap()));
        assert!(languages.contains(&LanguageIdentifier::from_str("fr").unwrap()));
    }

    #[test]
    fn test_set_language_from_accept_language_simple() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

        // Reset to English
        let english = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(language_loader(), &super::Localizations, &[english.clone()]);

        // Test with a simple Accept-Language header
        let _ = set_language_from_accept_language("es");
        assert_eq!(get_message("test-message"), "Este es un mensaje de prueba");

        // Reset to English for other tests
        let _ = select(language_loader(), &super::Localizations, &[english]);
    }

    #[test]
    fn test_set_language_from_accept_language_complex() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

        // Reset to English
        let english = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(language_loader(), &super::Localizations, &[english.clone()]);

        // Test with a complex Accept-Language header
        let _ = set_language_from_accept_language("fr-FR,fr;q=0.9,en;q=0.8");
        assert_eq!(get_message("test-message"), "Ceci est un message de test");

        // Reset to English for other tests
        let _ = select(language_loader(), &super::Localizations, &[english]);
    }

    #[test]
    fn test_set_language_from_accept_language_fallback() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

        // Reset to English
        let english = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(language_loader(), &super::Localizations, &[english.clone()]);

        // Test with a language that doesn't exist
        let _ = set_language_from_accept_language("de,ja;q=0.9,zh;q=0.8");
        // Should fall back to English
        assert_eq!(get_message("test-message"), "This is a test message");
    }

    #[test]
    fn test_set_language_from_accept_language_invalid() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

        // Reset to English
        let english = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(language_loader(), &super::Localizations, &[english.clone()]);

        // Test with an invalid Accept-Language header
        let _ = set_language_from_accept_language("invalid-language-tag");
        // Should remain English
        assert_eq!(get_message("test-message"), "This is a test message");
    }

    #[test]
    fn test_get_message_with_args() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

        // Reset to English
        let english = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(language_loader(), &super::Localizations, &[english.clone()]);

        // First, make sure the message exists
        assert_ne!(get_message("test-with-args"), "test-with-args");

        // Test with arguments
        let result = get_message_with_args("test-with-args", "John", "5");
        assert!(result.contains("John"));
        assert!(result.contains("5"));
        assert!(result.contains("notifications"));

        // Test with Spanish
        let _ = set_language_from_accept_language("es");
        let result = get_message_with_args("test-with-args", "John", "5");
        assert!(result.contains("John"));
        assert!(result.contains("5"));
        assert!(result.contains("notificaciones"));

        // Test with French
        let _ = set_language_from_accept_language("fr");
        let result = get_message_with_args("test-with-args", "John", "5");
        assert!(result.contains("John"));
        assert!(result.contains("5"));
        assert!(result.contains("notifications"));

        // Reset to English for other tests
        let _ = select(language_loader(), &super::Localizations, &[english]);
    }

    #[test]
    fn test_t_macro() {
        let _guard = TEST_MUTEX.lock().unwrap_or_else(|e| e.into_inner());

        // Reset to English
        let english = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(language_loader(), &super::Localizations, &[english.clone()]);

        // Test simple macro
        assert_eq!(crate::t!("test-message"), "This is a test message");

        // Test macro with arguments
        let result = crate::t!("test-with-args", name = "Jane", count = 10);
        assert!(result.contains("Jane"));
        assert!(result.contains("10"));
        assert!(result.contains("notifications"));

        // Test with Spanish
        let _ = set_language_from_accept_language("es");
        let result = crate::t!("test-with-args", name = "Jane", count = 10);
        assert!(result.contains("Jane"));
        assert!(result.contains("10"));
        assert!(result.contains("notificaciones"));

        // Reset to English for other tests
        let _ = select(language_loader(), &super::Localizations, &[english]);
    }
}
