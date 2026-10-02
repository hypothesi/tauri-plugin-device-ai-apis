/**
 * Text processing API for language identification and translation.
 * @module text
 */

import { invoke } from "@tauri-apps/api/core";
import type { LanguageIdentification, Translation, TranslationAvailability } from "./types";

/**
 * Identify the language of the given text.
 *
 * @param text The text to analyze.
 * @returns The language identification result.
 *
 * @example
 * ```typescript
 * import { text } from '@hypothesi/tauri-plugin-device-ai-apis';
 *
 * const result = await text.identifyLanguage('Bonjour, comment allez-vous?');
 * console.log(`Language: ${result.language} (${result.confidence})`);
 * ```
 */
export async function identifyLanguage(text: string): Promise<LanguageIdentification> {
  return invoke<LanguageIdentification>("plugin:device-ai-apis|text_identify_language", { text });
}

/**
 * Check on-device translation availability between two languages.
 *
 * Allows developers to proactively check if an offline language model
 * is already installed, supported but needs download, or unsupported before calling translate().
 *
 * @param from Source language code.
 * @param to Target language code.
 * @returns The translation availability status ('installed' | 'supported' | 'unsupported').
 *
 * @example
 * ```typescript
 * import { text } from '@hypothesi/tauri-plugin-device-ai-apis';
 *
 * const avail = await text.checkTranslationAvailability('en', 'es');
 * if (avail.status === 'installed') {
 *   const result = await text.translate('Hello', 'en', 'es');
 * }
 * ```
 */
export async function checkTranslationAvailability(
  from: string,
  to: string,
): Promise<TranslationAvailability> {
  return invoke<TranslationAvailability>("plugin:device-ai-apis|text_check_translation_availability", {
    from,
    to,
  });
}

/**
 * Translate text from one language to another.
 *
 * @param text The text to translate.
 * @param from Source language code.
 * @param to Target language code.
 * @returns The translation result.
 *
 * @example
 * ```typescript
 * import { text } from '@hypothesi/tauri-plugin-device-ai-apis';
 *
 * const result = await text.translate('Hello, world!', 'en', 'fr');
 * console.log(`Translation: ${result.translatedText}`);
 * ```
 */
export async function translate(text: string, from: string, to: string): Promise<Translation> {
  return invoke<Translation>("plugin:device-ai-apis|text_translate", {
    text,
    from,
    to,
  });
}
