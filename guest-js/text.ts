/**
 * Text processing API for language identification and translation.
 * @module text
 */

import { invoke } from "@tauri-apps/api/core";
import type { LanguageIdentification, Translation } from "./types";

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
