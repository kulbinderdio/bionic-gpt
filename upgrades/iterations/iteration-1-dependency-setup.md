# Iteration 1 - Dependency Setup and Infrastructure

## Objectives
- Add rust-i18n dependency
- Create locale directory structure
- Establish basic i18n initialization

## Implementation Steps

1. Add to Cargo.toml:
```toml
[dependencies]
rust-i18n = "3"
```

2. Create directory structure:
```bash
mkdir -p locales/en-US
```

3. Create base locale file (locales/en-US/app.ftl):
```ftl
welcome-header = Welcome to Bionic GPT
new-chat-button = New Chat
```

4. Initialize in main.rs:
```rust
rust_i18n::i18n!("locales");
rust_i18n::set_locale("en-US");
```

## Validation Tests
1. [ ] `cargo check` succeeds with new dependency
2. [ ] Locales directory exists with en-US/app.ftl
3. [ ] Basic i18n macro compiles in test module