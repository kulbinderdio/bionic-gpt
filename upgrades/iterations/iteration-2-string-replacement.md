# Iteration 2 - String Replacement

## Objectives
- Replace all web UI hardcoded strings with i18n lookups
- Maintain browser language detection only (no UI switcher)

## Implementation Steps

1. **Frontend File Targets** (crates/web-assets/typescript/ and crates/web-pages/):
```rust
// Before
html!("div", ["Welcome to Bionic GPT"])

// After
html!("div", [t!("welcome-header")])
```

2. **Add Fluent entries** (locales/en-US/app.ftl):
```ftl
chat-history-empty = No chat history available
model-loading = Loading model...
```

3. **Browser locale detection** (add to main.rs):
```rust
let browser_lang = web_sys::window()
    .and_then(|w| w.navigator().language())
    .unwrap_or("en-US".into());
rust_i18n::set_locale(&browser_lang);
```

## Validation Tests
1. [ ] Zero hardcoded strings in web-pages/ modules
2. [ ] All t! macros resolve to locale entries
3. [ ] Browser language detection works (test en vs fr browsers)
4. [ ] Fallback to en-US when translation missing