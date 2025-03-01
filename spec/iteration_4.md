# Iteration 4: Localization Files and Testing

## Objective
Extract hardcoded strings from the BionicGPT web interface into Fluent localization files, create comprehensive translation files for multiple languages, and implement integration tests to ensure the internationalization system works correctly in real-world scenarios.

## Technical Implementation Plan

### 1. Identify and Extract Hardcoded Strings

Perform a comprehensive audit of the BionicGPT web interface to identify all hardcoded strings that need to be internationalized. This includes:

- Navigation elements
- Button labels
- Form labels and placeholders
- Error messages
- Success messages
- Page titles and headings
- Tooltips and help text

For each identified string, create a unique message ID following a consistent naming convention:

- `section-element-description` format (e.g., `nav-home`, `form-username-label`, `error-login-failed`)

### 2. Create Comprehensive Localization Files

Based on the audit, create comprehensive localization files for English, Spanish, and French. Organize the files into logical sections for better maintainability.

Update the English localization file at `i18n/en/web_ui.ftl`:

```ftl
# Navigation
nav-home = Home
nav-settings = Settings
nav-profile = Profile
nav-logout = Log out
nav-dashboard = Dashboard
nav-documents = Documents
nav-datasets = Datasets
nav-models = Models
nav-prompts = Prompts
nav-pipelines = Pipelines
nav-teams = Teams
nav-audit-trail = Audit Trail
nav-api-keys = API Keys
nav-rate-limits = Rate Limits

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
common-filter = Filter
common-sort = Sort
common-refresh = Refresh
common-back = Back
common-next = Next
common-previous = Previous
common-submit = Submit
common-reset = Reset
common-close = Close
common-open = Open
common-view = View
common-download = Download
common-upload = Upload
common-yes = Yes
common-no = No
common-ok = OK
common-confirm = Confirm
common-actions = Actions

# Form labels
form-username = Username
form-password = Password
form-email = Email
form-name = Name
form-description = Description
form-created-at = Created at
form-updated-at = Updated at
form-status = Status
form-type = Type
form-size = Size
form-language = Language
form-tags = Tags
form-required = Required
form-optional = Optional

# Error messages
error-login-failed = Login failed. Please check your credentials.
error-not-found = The requested resource was not found.
error-unauthorized = You are not authorized to perform this action.
error-forbidden = Access to this resource is forbidden.
error-server = A server error occurred. Please try again later.
error-validation = Please fix the validation errors and try again.
error-file-too-large = The file is too large. Maximum size is {$size}.
error-unsupported-file-type = Unsupported file type. Allowed types: {$types}.
error-connection = Could not connect to the server. Please check your internet connection.
error-timeout = The request timed out. Please try again.

# Success messages
success-login = You have successfully logged in.
success-logout = You have successfully logged out.
success-create = {$item} has been created successfully.
success-update = {$item} has been updated successfully.
success-delete = {$item} has been deleted successfully.
success-upload = File has been uploaded successfully.
success-download = File has been downloaded successfully.

# Dashboard
dashboard-title = Dashboard
dashboard-welcome = Welcome to BionicGPT, {$username}!
dashboard-stats = You have {$count} active projects.
dashboard-recent-activity = Recent Activity
dashboard-no-activity = No recent activity.

# Documents
documents-title = Documents
documents-upload = Upload Document
documents-empty = No documents found.
documents-search = Search documents...
documents-filter = Filter documents
documents-sort = Sort documents

# Datasets
datasets-title = Datasets
datasets-create = Create Dataset
datasets-empty = No datasets found.
datasets-search = Search datasets...
datasets-filter = Filter datasets
datasets-sort = Sort datasets

# Models
models-title = Models
models-create = Create Model
models-empty = No models found.
models-search = Search models...
models-filter = Filter models
models-sort = Sort models

# Prompts
prompts-title = Prompts
prompts-create = Create Prompt
prompts-empty = No prompts found.
prompts-search = Search prompts...
prompts-filter = Filter prompts
prompts-sort = Sort prompts

# Pipelines
pipelines-title = Pipelines
pipelines-create = Create Pipeline
pipelines-empty = No pipelines found.
pipelines-search = Search pipelines...
pipelines-filter = Filter pipelines
pipelines-sort = Sort pipelines

# Teams
teams-title = Teams
teams-create = Create Team
teams-empty = No teams found.
teams-search = Search teams...
teams-filter = Filter teams
teams-sort = Sort teams
teams-members = Team Members
teams-add-member = Add Member
teams-remove-member = Remove Member

# Audit Trail
audit-trail-title = Audit Trail
audit-trail-empty = No audit records found.
audit-trail-search = Search audit trail...
audit-trail-filter = Filter audit trail
audit-trail-sort = Sort audit trail
audit-trail-user = User
audit-trail-action = Action
audit-trail-resource = Resource
audit-trail-timestamp = Timestamp

# API Keys
api-keys-title = API Keys
api-keys-create = Create API Key
api-keys-empty = No API keys found.
api-keys-search = Search API keys...
api-keys-filter = Filter API keys
api-keys-sort = Sort API keys
api-keys-name = Name
api-keys-key = Key
api-keys-created = Created
api-keys-expires = Expires
api-keys-status = Status
api-keys-copy = Copy
api-keys-copied = Copied!

# Rate Limits
rate-limits-title = Rate Limits
rate-limits-create = Create Rate Limit
rate-limits-empty = No rate limits found.
rate-limits-search = Search rate limits...
rate-limits-filter = Filter rate limits
rate-limits-sort = Sort rate limits
rate-limits-name = Name
rate-limits-limit = Limit
rate-limits-period = Period
rate-limits-resource = Resource
rate-limits-status = Status

# Profile
profile-title = Profile
profile-update = Update Profile
profile-change-password = Change Password
profile-current-password = Current Password
profile-new-password = New Password
profile-confirm-password = Confirm Password
profile-save = Save Changes
profile-cancel = Cancel

# Settings
settings-title = Settings
settings-general = General
settings-appearance = Appearance
settings-notifications = Notifications
settings-security = Security
settings-api = API
settings-advanced = Advanced
settings-save = Save Settings
settings-cancel = Cancel
settings-theme = Theme
settings-language = Language
settings-timezone = Timezone
settings-date-format = Date Format
settings-time-format = Time Format
```

Create similar files for Spanish (`i18n/es/web_ui.ftl`) and French (`i18n/fr/web_ui.ftl`) with appropriate translations.

### 3. Update Templates to Use Translations

Update all templates in the BionicGPT web interface to use the translation system. This involves:

1. Identifying all hardcoded strings in the templates
2. Replacing them with calls to the translation function or macro
3. Ensuring that any dynamic content is properly passed as arguments

For example, in the `crates/web-pages/app_layout.rs` file:

```rust
// Before
html! {
    div class="app-header" {
        h1 { "BionicGPT Dashboard" }
        nav {
            a href="/home" { "Home" }
            a href="/settings" { "Settings" }
            a href="/logout" { "Log out" }
        }
    }
}

// After
html! {
    div class="app-header" {
        h1 { (t!("dashboard-title")) }
        nav {
            a href="/home" { (t!("nav-home")) }
            a href="/settings" { (t!("nav-settings")) }
            a href="/logout" { (t!("nav-logout")) }
        }
    }
}
```

### 4. Create Integration Tests

Create integration tests to verify that the internationalization system works correctly in real-world scenarios. These tests should cover:

1. Language detection from browser settings
2. Correct rendering of translated content
3. Handling of dynamic content with arguments
4. Fallback behavior for missing translations

Create a new test file `crates/web-server/tests/i18n_integration_test.rs`:

```rust
use actix_web::{test, web, App, HttpResponse};
use actix_web::http::header;
use crate::middleware::i18n::I18nMiddleware;
use crate::i18n::{get_message, set_language_from_accept_language};
use unic_langid::LanguageIdentifier;
use std::str::FromStr;

// Test handler that returns translated content
async fn test_handler() -> HttpResponse {
    let welcome = get_message("common-welcome");
    let dashboard = get_message("dashboard-title");
    let content = format!("<html><body><h1>{}</h1><h2>{}</h2></body></html>", welcome, dashboard);
    HttpResponse::Ok().content_type("text/html").body(content)
}

#[actix_rt::test]
async fn test_language_detection_and_rendering() {
    // Create test app with middleware
    let app = test::init_service(
        App::new()
            .wrap(I18nMiddleware)
            .route("/test", web::get().to(test_handler))
    ).await;

    // Test with English
    let req = test::TestRequest::get()
        .uri("/test")
        .header(header::ACCEPT_LANGUAGE, "en")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Welcome to BionicGPT"));
    assert!(body_str.contains("Dashboard"));

    // Test with Spanish
    let req = test::TestRequest::get()
        .uri("/test")
        .header(header::ACCEPT_LANGUAGE, "es")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Bienvenido a BionicGPT"));
    assert!(body_str.contains("Panel de control"));

    // Test with French
    let req = test::TestRequest::get()
        .uri("/test")
        .header(header::ACCEPT_LANGUAGE, "fr")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Bienvenue à BionicGPT"));
    assert!(body_str.contains("Tableau de bord"));
}

// Test handler that returns translated content with arguments
async fn test_handler_with_args() -> HttpResponse {
    let username = "TestUser";
    let count = "5";

    let welcome = t!("dashboard-welcome", username);
    let stats = t!("dashboard-stats", count);

    let content = format!("<html><body><p>{}</p><p>{}</p></body></html>", welcome, stats);
    HttpResponse::Ok().content_type("text/html").body(content)
}

#[actix_rt::test]
async fn test_translation_with_arguments() {
    // Create test app with middleware
    let app = test::init_service(
        App::new()
            .wrap(I18nMiddleware)
            .route("/test-args", web::get().to(test_handler_with_args))
    ).await;

    // Test with English
    let req = test::TestRequest::get()
        .uri("/test-args")
        .header(header::ACCEPT_LANGUAGE, "en")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Welcome to BionicGPT, TestUser!"));
    assert!(body_str.contains("You have 5 active projects"));

    // Test with Spanish
    let req = test::TestRequest::get()
        .uri("/test-args")
        .header(header::ACCEPT_LANGUAGE, "es")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Bienvenido a BionicGPT, TestUser!"));
    assert!(body_str.contains("Tienes 5 proyectos activos"));
}
```

### 5. Test Edge Cases

Create tests for edge cases to ensure the internationalization system is robust:

```rust
#[actix_rt::test]
async fn test_edge_cases() {
    // Create test app with middleware
    let app = test::init_service(
        App::new()
            .wrap(I18nMiddleware)
            .route("/test", web::get().to(test_handler))
    ).await;

    // Test with unsupported language (should fall back to English)
    let req = test::TestRequest::get()
        .uri("/test")
        .header(header::ACCEPT_LANGUAGE, "ja")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Welcome to BionicGPT"));
    assert!(body_str.contains("Dashboard"));

    // Test with invalid language tag (should fall back to English)
    let req = test::TestRequest::get()
        .uri("/test")
        .header(header::ACCEPT_LANGUAGE, "invalid-language-tag")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Welcome to BionicGPT"));
    assert!(body_str.contains("Dashboard"));

    // Test with empty Accept-Language header (should use default language)
    let req = test::TestRequest::get()
        .uri("/test")
        .header(header::ACCEPT_LANGUAGE, "")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Welcome to BionicGPT"));
    assert!(body_str.contains("Dashboard"));

    // Test with no Accept-Language header (should use default language)
    let req = test::TestRequest::get()
        .uri("/test")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert!(body_str.contains("Welcome to BionicGPT"));
    assert!(body_str.contains("Dashboard"));

    // Test with missing translation key (should return the key itself)
    // Create a handler that uses a non-existent key
    let app = test::init_service(
        App::new()
            .wrap(I18nMiddleware)
            .route("/test-missing", web::get().to(|| async {
                let missing = get_message("non-existent-key");
                HttpResponse::Ok().body(missing)
            }))
    ).await;

    let req = test::TestRequest::get()
        .uri("/test-missing")
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body = test::read_body(resp).await;
    let body_str = std::str::from_utf8(&body).unwrap();
    assert_eq!(body_str, "non-existent-key");
}
```

### 6. Create Documentation

Create documentation for the internationalization system to help developers understand how to use it and how to add new translations.

Create a new file `docs/i18n.md`:

```markdown
# Internationalization (i18n) in BionicGPT

BionicGPT uses the `i18n-embed` crate with Fluent for internationalization. This document explains how to use and extend the i18n system.

## Using Translations

In Rust code, use the `t!` macro to translate strings:

```rust
// Simple translation
let save_label = t!("common-save");

// Translation with variable substitution
let welcome = t!("dashboard-welcome", username);
```

In templates that don't support macros directly, use the helper functions:

```html
<h1>{{ translate("common-welcome") }}</h1>
<p>{{ translate_with_args("dashboard-welcome", {"username": user.name}) }}</p>
```

## Adding New Translations

1. Add the new string to `i18n/en/web_ui.ftl` with a unique ID
2. Add translations for the same ID in other language files (`i18n/es/web_ui.ftl`, `i18n/fr/web_ui.ftl`, etc.)

## Adding New Languages

1. Create a new directory in `i18n/` named with the language code (e.g., `i18n/de/`)
2. Copy the FTL files from the `en` directory
3. Translate all messages in the copied files

## Translation Guidelines

1. Use a consistent naming convention for message IDs: `section-element-description`
2. Organize messages into logical sections with comments
3. Keep translations concise and clear
4. Use arguments for dynamic content: `welcome = Hello, {$name}!`
5. Test translations in context to ensure they fit the UI

## Fluent Syntax Reference

Fluent is a localization system designed for natural-sounding translations. Here's a quick reference:

- Simple message: `key = value`
- Message with arguments: `key = value with {$arg}`
- Multiline message:
  ```
  key =
      First line
      Second line
  ```
- Comments: `# This is a comment`
- Attributes:
  ```
  key = value
      .attribute = attribute value
  ```
- Selectors:
  ```
  key = {$count ->
      [one] You have one notification
     *[other] You have {$count} notifications
  }
  ```

For more details, see the [Fluent Syntax Guide](https://projectfluent.org/fluent/guide/).
```

## Acceptance Criteria

1. All hardcoded strings in the BionicGPT web interface are extracted into Fluent localization files
2. Comprehensive localization files are created for English, Spanish, and French
3. All templates are updated to use the translation system
4. Integration tests verify that the internationalization system works correctly in real-world scenarios
5. Edge cases are tested to ensure the system is robust
6. Documentation is created to help developers understand how to use and extend the i18n system

## Code Coverage Requirements

The unit and integration tests should cover:
- Language detection from browser settings
- Correct rendering of translated content
- Handling of dynamic content with arguments
- Fallback behavior for missing translations
- Edge cases such as unsupported languages, invalid language tags, and missing translation keys

This should achieve at least 90% code coverage for the internationalization system.
