# Iteration 3 - Fallback Configuration

## Objectives
- Implement automatic browser locale detection
- Configure en-US as default fallback
- Validate missing translation handling

## Implementation Steps

1. **Enhanced locale initialization** (main.rs):
```rust
rust_i18n::i18n!("locales", fallback = "en-US");

let lang = detect_browser_lang().unwrap_or("en-US".into());
rust_i18n::set_locale(&lang);
```

2. **Create fallback test locale** (locales/fr-FR/app.ftl):
```ftl
welcome-header = Bienvenue sur Bionic GPT
# Deliberately omit other translations
```

3. **Add fallback test case** (crates/web-assets/typescript/__tests__/i18n.test.ts):
```typescript
test('falls back to en-US for missing translations', () => {
  setLocale('fr-FR');
  expect(t('model-loading')).toBe('Loading model...');
});
```

## Validation Tests
1. [ ] Browser returns en-US when unsupported locale requested
2. [ ] Missing keys in active locale use fallback translations
3. [ ] Unit tests verify fallback behavior
4. [ ] CI pipeline passes with new test cases