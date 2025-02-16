import { setLocale, t } from '../i18n';

describe('i18n', () => {
  test('falls back to en-US for missing translations', () => {
    setLocale('fr-FR');
    expect(t('model-loading')).toBe('Loading model...');
  });

  test('uses available translations when present', () => {
    setLocale('fr-FR');
    expect(t('welcome-header')).toBe('Bienvenue sur Bionic GPT');
  });

  test('defaults to en-US when locale not supported', () => {
    setLocale('invalid-locale');
    expect(t('model-loading')).toBe('Loading model...');
  });
});