# Iteration 2: Language Detection and Middleware

## Objective
Implement language detection from the Accept-Language HTTP header and create middleware for the web server to automatically set the appropriate language for each request.

## Technical Implementation Plan

### 1. Enhance the i18n Module with Language Detection

Update the `crates/web-server/src/i18n.rs` file to add language detection functionality:

```rust
// Add these imports if not already present
use std::collections::HashSet;

// Add this function to set language from Accept-Language header
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

// Add a function to get available languages
pub fn available_languages() -> HashSet<LanguageIdentifier> {
    let loader = language_loader();
    loader.locales()
}
```

### 2. Create Middleware for Web Server

Create a new file `crates/web-server/src/middleware/i18n.rs` with the following implementation:

```rust
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::LocalBoxFuture;
use std::future::{ready, Ready};

use crate::i18n;

pub struct I18nMiddleware;

impl<S, B> Transform<S, ServiceRequest> for I18nMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = I18nMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(I18nMiddlewareService { service }))
    }
}

pub struct I18nMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for I18nMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract Accept-Language header
        if let Some(accept_language) = req.headers().get("Accept-Language") {
            if let Ok(accept_language) = accept_language.to_str() {
                // Ignore errors from language detection - we'll fall back to default
                let _ = i18n::set_language_from_accept_language(accept_language);
            }
        }

        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
```

### 3. Create Middleware Module

Create a new directory and file structure if it doesn't exist:

```
crates/web-server/src/middleware/
└── mod.rs
```

Update `crates/web-server/src/middleware/mod.rs` to include the i18n middleware:

```rust
pub mod i18n;
```

### 4. Register the Middleware

Update `crates/web-server/src/main.rs` to register the i18n middleware:

```rust
// Add these imports
use crate::middleware::i18n::I18nMiddleware;

// In the HttpServer::new closure, add the middleware to the App
// Find the App::new() call and add .wrap(I18nMiddleware) after it
// For example:
let app = App::new()
    .wrap(I18nMiddleware)
    // ... other middleware and services
```

### 5. Update the Main Module to Include Middleware

Update `crates/web-server/src/main.rs` to include the middleware module:

```rust
// Add this line to the existing imports
mod middleware;
```

## Unit Tests

### 1. Update the i18n Test File

Update `crates/web-server/src/i18n_test.rs` to add tests for language detection:

```rust
#[cfg(test)]
mod tests {
    use super::i18n::{language_loader, get_message, set_language_from_accept_language, available_languages};
    use unic_langid::LanguageIdentifier;
    use std::str::FromStr;
    use i18n_embed::select;

    // ... existing tests ...

    #[test]
    fn test_available_languages() {
        let languages = available_languages();
        assert!(languages.contains(&LanguageIdentifier::from_str("en").unwrap()));
        assert!(languages.contains(&LanguageIdentifier::from_str("es").unwrap()));
        assert!(languages.contains(&LanguageIdentifier::from_str("fr").unwrap()));
    }

    #[test]
    fn test_set_language_from_accept_language_simple() {
        // Reset to English
        let english = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(language_loader(), &super::i18n::Localizations, &[english]);

        // Test with a simple Accept-Language header
        let _ = set_language_from_accept_language("es");
        assert_eq!(get_message("test-message"), "Este es un mensaje de prueba");
    }

    #[test]
    fn test_set_language_from_accept_language_complex() {
        // Reset to English
        let english = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(language_loader(), &super::i18n::Localizations, &[english]);

        // Test with a complex Accept-Language header
        let _ = set_language_from_accept_language("fr-FR,fr;q=0.9,en;q=0.8");
        assert_eq!(get_message("test-message"), "Ceci est un message de test");
    }

    #[test]
    fn test_set_language_from_accept_language_fallback() {
        // Reset to English
        let english = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(language_loader(), &super::i18n::Localizations, &[english]);

        // Test with a language that doesn't exist
        let _ = set_language_from_accept_language("de,ja;q=0.9,zh;q=0.8");
        // Should fall back to English
        assert_eq!(get_message("test-message"), "This is a test message");
    }

    #[test]
    fn test_set_language_from_accept_language_invalid() {
        // Reset to English
        let english = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(language_loader(), &super::i18n::Localizations, &[english]);

        // Test with an invalid Accept-Language header
        let _ = set_language_from_accept_language("invalid-language-tag");
        // Should remain English
        assert_eq!(get_message("test-message"), "This is a test message");
    }
}
```

### 2. Create Middleware Test File

Create a new test file `crates/web-server/src/middleware/i18n_test.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::i18n::I18nMiddleware;
    use actix_web::{test, web, App, HttpResponse};
    use actix_web::http::header;
    use crate::i18n::get_message;
    use unic_langid::LanguageIdentifier;
    use std::str::FromStr;
    use i18n_embed::select;

    async fn test_handler() -> HttpResponse {
        HttpResponse::Ok().body(get_message("test-message"))
    }

    #[actix_rt::test]
    async fn test_middleware_sets_language() {
        // Reset to English
        let english = LanguageIdentifier::from_str("en").unwrap();
        let _ = select(crate::i18n::language_loader(), &crate::i18n::Localizations, &[english]);

        // Create test app with middleware
        let app = test::init_service(
            App::new()
                .wrap(I18nMiddleware)
                .route("/test", web::get().to(test_handler))
        ).await;

        // Test with Spanish Accept-Language header
        let req = test::TestRequest::get()
            .uri("/test")
            .header(header::ACCEPT_LANGUAGE, "es")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        assert_eq!(body, "Este es un mensaje de prueba");

        // Test with French Accept-Language header
        let req = test::TestRequest::get()
            .uri("/test")
            .header(header::ACCEPT_LANGUAGE, "fr-FR,fr;q=0.9")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        assert_eq!(body, "Ceci est un message de test");

        // Test with no Accept-Language header (should default to English)
        let req = test::TestRequest::get()
            .uri("/test")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        assert_eq!(body, "This is a test message");
    }
}
```

## Acceptance Criteria

1. The i18n module is enhanced with language detection functionality
2. The middleware for the web server is created
3. The middleware is registered in the main application
4. All unit tests pass, verifying:
   - Available languages are correctly detected
   - Language is correctly set from simple Accept-Language headers
   - Language is correctly set from complex Accept-Language headers
   - Fallback to default language works when requested language is not available
   - Invalid language tags are handled gracefully
   - The middleware correctly sets the language based on the Accept-Language header

## Code Coverage Requirements

The unit tests should cover:
- Language detection from Accept-Language headers
- Handling of complex Accept-Language headers
- Fallback behavior for unavailable languages
- Middleware integration with the web server

This should achieve at least 90% code coverage for the language detection and middleware functionality.
