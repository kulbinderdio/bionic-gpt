# Iteration 3: Translation Integration

## Objective
Implement a translation macro and update templates to use translations. This iteration focuses on making the internationalization system usable throughout the application.

## Technical Implementation Plan

### 1. Create Translation Macro

Update the `crates/web-server/src/i18n.rs` file to add a translation macro:

```rust
// Add this import
use std::collections::HashMap;

// Add this macro definition
#[macro_export]
macro_rules! t {
    ($message_id:expr) => {{
        $crate::i18n::get_message($message_id)
    }};
    ($message_id:expr, $($args:ident),*) => {{
        let mut args = std::collections::HashMap::new();
        $(
            args.insert(stringify!($args).to_string(), $args.to_string());
        )*
        $crate::i18n::get_message_with_args($message_id, &args)
    }};
}

// Add this function to support the macro with arguments
pub fn get_message_with_args(message_id: &str, args: &HashMap<String, String>) -> String {
    let loader = language_loader();

    // Convert HashMap to fluent args
    let fluent_args = args.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect::<HashMap<_, _>>();

    loader
        .get_args(message_id, &fluent_args)
        .unwrap_or_else(|_| message_id.to_string())
}
```

### 2. Update Localization Files to Include Arguments

Update the English localization file at `i18n/en/web_ui.ftl`:

```ftl
# Basic test messages
test-message = This is a test message
test-with-args = Hello, {$name}! You have {$count} notifications.

# Common UI elements
common-welcome = Welcome to BionicGPT
common-loading = Loading...
common-error = An error occurred
common-success = Success!
common-cancel = Cancel
common-save = Save
common-delete = Delete
common-edit = Edit
common-search = Search
```

Update the Spanish localization file at `i18n/es/web_ui.ftl`:

```ftl
# Basic test messages
test-message = Este es un mensaje de prueba
test-with-args = ¡Hola, {$name}! Tienes {$count} notificaciones.

# Common UI elements
common-welcome = Bienvenido a BionicGPT
common-loading = Cargando...
common-error = Se ha producido un error
common-success = ¡Éxito!
common-cancel = Cancelar
common-save = Guardar
common-delete = Eliminar
common-edit = Editar
common-search = Buscar
```

Update the French localization file at `i18n/fr/web_ui.ftl`:

```ftl
# Basic test messages
test-message = Ceci est un message de test
test-with-args = Bonjour, {$name}! Vous avez {$count} notifications.

# Common UI elements
common-welcome = Bienvenue à BionicGPT
common-loading = Chargement...
common-error = Une erreur s'est produite
common-success = Succès!
common-cancel = Annuler
common-save = Enregistrer
common-delete = Supprimer
common-edit = Modifier
common-search = Rechercher
```

### 3. Create a Helper Module for Template Integration

Create a new file `crates/web-server/src/i18n_helpers.rs` with the following implementation:

```rust
use crate::i18n;
use std::collections::HashMap;

// Helper function for templates that don't support macros directly
pub fn translate(message_id: &str) -> String {
    i18n::get_message(message_id)
}

// Helper function for templates with arguments
pub fn translate_with_args(message_id: &str, args: HashMap<String, String>) -> String {
    i18n::get_message_with_args(message_id, &args)
}
```

### 4. Update Templates to Use Translations

For this step, we need to identify the templating system used in BionicGPT. Based on the project structure, it appears to be using a custom templating system or possibly a Rust HTML generation library.

For demonstration purposes, we'll show how to update templates in different scenarios:

#### For Rust HTML Generation (e.g., using `maud` or similar):

```rust
// Before
html! {
    h1 { "Welcome to BionicGPT" }
    p { "Loading..." }
    button { "Save" }
}

// After
html! {
    h1 { (t!("common-welcome")) }
    p { (t!("common-loading")) }
    button { (t!("common-save")) }
}
```

#### For Template-Based Rendering (e.g., Tera, Askama):

```html
<!-- Before -->
<h1>Welcome to BionicGPT</h1>
<p>Loading...</p>
<button>Save</button>

<!-- After -->
<h1>{{ translate("common-welcome") }}</h1>
<p>{{ translate("common-loading") }}</p>
<button>{{ translate("common-save") }}</button>
```

#### For Templates with Arguments:

```html
<!-- Before -->
<p>Hello, {{ user.name }}! You have {{ notification_count }} notifications.</p>

<!-- After -->
<p>{{ translate_with_args("test-with-args", {"name": user.name, "count": notification_count}) }}</p>
```

### 5. Update the Main Module to Include i18n Helpers

Update `crates/web-server/src/main.rs` to include the i18n helpers module:

```rust
// Add this line to the existing imports
mod i18n_helpers;
```

### 6. Register Template Helpers

If the application uses a template engine that requires registering helpers, update the appropriate setup code. For example, with Tera:

```rust
// Find where the template engine is initialized
let mut tera = Tera::new("templates/**/*")?;

// Add the translation helpers
tera.register_function("translate", crate::i18n_helpers::translate);
tera.register_function("translate_with_args", crate::i18n_helpers::translate_with_args);
```

## Unit Tests

### 1. Update the i18n Test File

Update `crates/web-server/src/i18n_test.rs` to add tests for the translation macro and arguments:

```rust
#[cfg(test)]
mod tests {
    use super::i18n::{language_loader, get_message, get_message_with_args, set_language_from_accept_language};
    use unic_langid::LanguageIdentifier;
    use std::str::FromStr;
    use std::collections::HashMap;
    use i18n_embed::select;

    // ... existing tests ...

    #[test]
    fn test_get_message_with_args() {
        // Reset to English
        let english = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(language_loader(), &super::i18n::Localizations, &[english]);

        let mut args = HashMap::new();
        args.insert("name".to_string(), "John".to_string());
        args.insert("count".to_string(), "5".to_string());

        assert_eq!(
            get_message_with_args("test-with-args", &args),
            "Hello, John! You have 5 notifications."
        );
    }

    #[test]
    fn test_t_macro_simple() {
        // Reset to English
        let english = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(language_loader(), &super::i18n::Localizations, &[english]);

        assert_eq!(t!("common-welcome"), "Welcome to BionicGPT");
    }

    #[test]
    fn test_t_macro_with_args() {
        // Reset to English
        let english = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(language_loader(), &super::i18n::Localizations, &[english]);

        let name = "Alice".to_string();
        let count = "3".to_string();

        assert_eq!(
            t!("test-with-args", name, count),
            "Hello, Alice! You have 3 notifications."
        );
    }

    #[test]
    fn test_t_macro_with_different_languages() {
        // Test with Spanish
        let spanish = LanguageIdentifier::from_str("es").unwrap();
        let _ = select(language_loader(), &super::i18n::Localizations, &[spanish]);

        let name = "Carlos".to_string();
        let count = "7".to_string();

        assert_eq!(
            t!("test-with-args", name, count),
            "¡Hola, Carlos! Tienes 7 notificaciones."
        );

        // Test with French
        let french = LanguageIdentifier::from_str("fr").unwrap();
        let _ = select(language_loader(), &super::i18n::Localizations, &[french]);

        assert_eq!(
            t!("test-with-args", name, count),
            "Bonjour, Carlos! Vous avez 7 notifications."
        );
    }
}
```

### 2. Create i18n Helpers Test File

Create a new test file `crates/web-server/src/i18n_helpers_test.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::i18n_helpers::{translate, translate_with_args};
    use unic_langid::LanguageIdentifier;
    use std::str::FromStr;
    use std::collections::HashMap;
    use i18n_embed::select;

    #[test]
    fn test_translate() {
        // Reset to English
        let english = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(crate::i18n::language_loader(), &crate::i18n::Localizations, &[english]);

        assert_eq!(translate("common-welcome"), "Welcome to BionicGPT");
    }

    #[test]
    fn test_translate_with_args() {
        // Reset to English
        let english = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(crate::i18n::language_loader(), &crate::i18n::Localizations, &[english]);

        let mut args = HashMap::new();
        args.insert("name".to_string(), "John".to_string());
        args.insert("count".to_string(), "5".to_string());

        assert_eq!(
            translate_with_args("test-with-args", args),
            "Hello, John! You have 5 notifications."
        );
    }

    #[test]
    fn test_translate_with_different_languages() {
        // Test with Spanish
        let spanish = LanguageIdentifier::from_str("es").unwrap();
        let _ = select(crate::i18n::language_loader(), &crate::i18n::Localizations, &[spanish]);

        assert_eq!(translate("common-welcome"), "Bienvenido a BionicGPT");

        // Test with French
        let french = LanguageIdentifier::from_str("fr").unwrap();
        let _ = select(crate::i18n::language_loader(), &crate::i18n::Localizations, &[french]);

        assert_eq!(translate("common-welcome"), "Bienvenue à BionicGPT");
    }
}
```

## Acceptance Criteria

1. The translation macro is implemented and works correctly
2. The localization files are updated to include arguments
3. The helper module for template integration is created
4. Templates are updated to use translations
5. All unit tests pass, verifying:
   - The translation macro works for simple messages
   - The translation macro works with arguments
   - The translation macro works with different languages
   - The helper functions work correctly
   - The helper functions work with different languages

## Code Coverage Requirements

The unit tests should cover:
- Translation macro functionality
- Translation with arguments
- Helper functions for templates
- Integration with different languages

This should achieve at least 90% code coverage for the translation integration functionality.
