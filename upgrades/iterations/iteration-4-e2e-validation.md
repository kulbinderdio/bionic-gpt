# Iteration 4 - End-to-End Validation

## Objectives
- Verify complete i18n implementation
- Ensure zero regression in core functionality
- Validate browser language integration

## Implementation Steps

1. **Browser Testing Matrix**:
```bash
# Test different browser locales
BROWSER_LANG=es-ES cargo test --test e2e
BROWSER_LANG=fr-FR cargo test --test e2e
BROWSER_LANG=ja-JP cargo test --test e2e
```

2. **Add visual regression tests** (crates/web-assets/__tests__/screenshots.test.ts):
```typescript
test('renders correctly in French', async () => {
  setLocale('fr-FR');
  await expect(page).toMatchScreenSnapshot();
});
```

3. **Final audit checklist**:
```text
- [ ] All UI text comes from locale files
- [ ] No console warnings about missing translations
- [ ] Browser language detection works across Chrome/Firefox/Safari
```

## Validation Tests
1. [ ] 100% test coverage on i18n-related code
2. [ ] All E2E tests pass with mixed locale scenarios
3. [ ] Performance budget maintained (check bundle size)
4. [ ] Security review passes (no eval/unsafe i18n patterns)