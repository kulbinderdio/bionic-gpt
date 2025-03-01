# Iteration 1: Setup and Basic Infrastructure

## Objective
Set up the foundational infrastructure for internationalization (i18n) in the BionicGPT web interface. This iteration focuses on adding the required dependencies, creating the directory structure for localization files, and implementing the basic i18n module with a language loader.

## Technical Implementation Plan

### 1. Add Required Dependencies

Add the following dependencies to `Cargo.toml`:

```toml
[dependencies]
i18n-embed = "0.14.1"
i18n-embed-fl = "0.7.0"
rust-embed = "8.0.0"
fluent = "0.16.0"
unic-langid = "0.9.1"
once_cell = "1.18.0"  # For static initialization
```

### 2. Create Directory Structure for Localization Files

Create the following directory structure:

```
bionic-gpt/
└── i18n/
    ├── en/
    │   └── web_ui.ftl
    ├── es/
    │   └── web_ui.ftl
    └── fr/
        └── web_ui.ftl
```

### 3. Create Basic Localization Files

Create a basic English localization file at `i18n/en/web_ui.ftl`:

```ftl
# Basic test messages
test-message = This is a test message
```

Create similar files for Spanish (`i18n/es/web_ui.ftl`):

```ftl
# Basic test messages
test-message = Este es un mensaje de prueba
```

And French (`i18n/fr/web_ui.ftl`):

```ftl
# Basic test messages
test-message = Ceci est un message de test
```

### 4. Create i18n Module

Create a new file `crates/web-server/src/i18n.rs` with the following implementation:

```rust
use fluent::FluentResource;
use i18n_embed::{
    fluent::{fluent_language_loader, FluentLanguageLoader},
    LanguageLoader,
};
use rust_embed::RustEmbed;
use once_cell::sync::OnceCell;
use unic_langid::LanguageIdentifier;
use std::str::FromStr;

// Define the embedded assets
#[derive(RustEmbed)]
#[folder = "i18n"]
struct Localizations;

// Create a singleton loader
static LANGUAGE_LOADER: OnceCell<FluentLanguageLoader> = OnceCell::new();

pub fn language_loader() -> &'static FluentLanguageLoader {
    LANGUAGE_LOADER.get_or_init(|| {
        let loader = fluent_language_loader!();
        // Load English as fallback
        let fallback = LanguageIdentifier::from_str("en").expect("Invalid language identifier");
        loader
            .load_fallback_language(&Localizations)
            .expect("Failed to load fallback language");
        loader
    })
}

// Basic translation function for testing
pub fn get_message(message_id: &str) -> String {
    language_loader()
        .get(message_id)
        .unwrap_or_else(|_| message_id.to_string())
}
```

### 5. Update the Main Module to Include i18n

Update `crates/web-server/src/main.rs` to include the i18n module:

```rust
// Add this line to the existing imports
mod i18n;
```

## Unit Tests

Create a test file `crates/web-server/src/i18n_test.rs` with the following tests:

```rust
#[cfg(test)]
mod tests {
    use super::i18n::{language_loader, get_message};
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
```

## Acceptance Criteria

1. All dependencies are correctly added to `Cargo.toml`
2. The directory structure for localization files is created
3. Basic localization files are created for English, Spanish, and French
4. The i18n module is created with a language loader function
5. The main module is updated to include the i18n module
6. All unit tests pass, verifying:
   - The language loader initializes correctly
   - The default language (English) is loaded
   - Missing messages return the message ID as a fallback

## Code Coverage Requirements

The unit tests should cover:
- Language loader initialization
- Default language loading
- Fallback behavior for missing messages

This should achieve at least 90% code coverage for the i18n module.
