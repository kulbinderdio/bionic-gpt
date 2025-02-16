// i18n module for handling translations
let currentLocale = 'en-US';

export function setLocale(locale: string) {
  currentLocale = locale;
}

export function t(key: string): string {
  // In a real implementation, this would look up translations from .ftl files
  // For testing purposes, we'll hardcode a few translations
  const translations: Record<string, Record<string, string>> = {
    'en-US': {
      'model-loading': 'Loading model...',
      'welcome-header': 'Welcome to Bionic GPT'
    },
    'fr-FR': {
      'welcome-header': 'Bienvenue sur Bionic GPT'
      // Deliberately omit other translations to test fallback
    }
  };

  // Try to get translation in current locale
  const localeTranslations = translations[currentLocale];
  if (localeTranslations && localeTranslations[key]) {
    return localeTranslations[key];
  }

  // Fallback to en-US
  return translations['en-US'][key] || key;
}