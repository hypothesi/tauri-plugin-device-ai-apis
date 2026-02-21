/**
 * Speech recognition and synthesis API.
 * @module speech
 */

import { invoke } from "@tauri-apps/api/core";
import type { RecognitionOptions, RecognitionResult, SynthesisOptions, Voice } from "./types";
import { isTauri, hasWebSpeechRecognition, hasWebSpeechSynthesis } from "./platform";
import {
  webRecognize,
  webStartRecognition,
  webStopRecognition,
  webSynthesize,
  webGetVoices,
} from "./web-speech";

/**
 * Perform one-shot speech recognition.
 *
 * Records audio from the microphone and returns the recognized text
 * when the user stops speaking.
 *
 * Uses native APIs when running in Tauri, falls back to Web Speech API
 * when running in a browser.
 *
 * @param options Recognition options.
 * @returns The recognition result.
 *
 * @example
 * ```typescript
 * import { speech } from '@hypothesi/tauri-plugin-device-ai-apis';
 *
 * const result = await speech.recognize({ language: 'en-US' });
 * console.log('You said:', result.text);
 * ```
 */
export async function recognize(options?: RecognitionOptions): Promise<RecognitionResult> {
  if (isTauri()) {
    return invoke<RecognitionResult>("plugin:device-ai-apis|speech_recognize", {
      options: options ?? {},
    });
  }
  if (hasWebSpeechRecognition()) {
    return webRecognize(options);
  }
  throw new Error("Speech recognition not available on this platform");
}

/**
 * Start streaming speech recognition.
 *
 * Returns a session ID that can be used to stop the session.
 * Uses native APIs when running in Tauri, falls back to Web Speech API
 * when running in a browser.
 *
 * @param options Recognition options.
 * @returns The session ID.
 */
export async function startRecognition(options?: RecognitionOptions): Promise<string> {
  if (isTauri()) {
    return invoke<string>("plugin:device-ai-apis|speech_recognize_start", {
      options: options ?? {},
    });
  }
  if (hasWebSpeechRecognition()) {
    return webStartRecognition(options);
  }
  throw new Error("Speech recognition not available on this platform");
}

/**
 * Stop a streaming speech recognition session.
 *
 * @param sessionId The session ID returned by startRecognition.
 * @returns The final recognition result.
 */
export async function stopRecognition(sessionId: string): Promise<RecognitionResult> {
  if (isTauri()) {
    return invoke<RecognitionResult>("plugin:device-ai-apis|speech_recognize_stop", {
      sessionId,
    });
  }
  // Web sessions start with 'web-speech-' prefix
  if (sessionId.startsWith("web-speech-")) {
    return webStopRecognition(sessionId);
  }
  throw new Error("Speech recognition not available on this platform");
}

/**
 * Synthesize and speak the given text.
 *
 * This is a blocking operation that completes when speech finishes.
 * Uses native APIs when running in Tauri, falls back to Web Speech API
 * when running in a browser.
 *
 * @param text The text to speak.
 * @param options Synthesis options.
 *
 * @example
 * ```typescript
 * import { speech } from '@hypothesi/tauri-plugin-device-ai-apis';
 *
 * await speech.synthesize('Hello, world!', { rate: 1.0 });
 * ```
 */
export async function synthesize(text: string, options?: SynthesisOptions): Promise<void> {
  if (isTauri()) {
    return invoke<void>("plugin:device-ai-apis|speech_synthesize", {
      text,
      options: options ?? {},
    });
  }
  if (hasWebSpeechSynthesis()) {
    return webSynthesize(text, options);
  }
  throw new Error("Speech synthesis not available on this platform");
}

/**
 * Get the list of available voices for speech synthesis.
 *
 * Uses native APIs when running in Tauri, falls back to Web Speech API
 * when running in a browser.
 *
 * @returns The list of available voices.
 *
 * @example
 * ```typescript
 * import { speech } from '@hypothesi/tauri-plugin-device-ai-apis';
 *
 * const voices = await speech.getVoices();
 * const englishVoices = voices.filter(v => v.language.startsWith('en'));
 * ```
 */
export async function getVoices(): Promise<Voice[]> {
  if (isTauri()) {
    return invoke<Voice[]>("plugin:device-ai-apis|speech_get_voices");
  }
  if (hasWebSpeechSynthesis()) {
    return webGetVoices();
  }
  throw new Error("Speech synthesis not available on this platform");
}
