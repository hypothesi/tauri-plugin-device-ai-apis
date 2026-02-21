/**
 * Capability detection API for the device AI plugin.
 * @module capabilities
 */

import { invoke } from "@tauri-apps/api/core";
import type { Capabilities, FeatureStatus } from "./types";
import {
  isTauri,
  hasWebSpeechRecognition,
  hasWebSpeechSynthesis,
  hasBarcodeDetection,
} from "./platform";

/**
 * Get the AI capabilities available on the current platform.
 *
 * When running in Tauri, returns native capabilities from the backend.
 * When running in a browser, returns Web API capabilities.
 *
 * @returns The capabilities object describing available features.
 *
 * @example
 * ```typescript
 * import { getCapabilities } from '@hypothesi/tauri-plugin-device-ai-apis';
 *
 * const caps = await getCapabilities();
 * if (caps.speechRecognition.available) {
 *   console.log('Speech recognition is available!');
 * }
 * ```
 */
export async function getCapabilities(): Promise<Capabilities> {
  if (isTauri()) {
    return invoke<Capabilities>("plugin:device-ai-apis|get_capabilities");
  }
  // Return web capabilities when not in Tauri
  return getWebCapabilities();
}

/**
 * Get capabilities available via Web APIs.
 */
function getWebCapabilities(): Capabilities {
  const notAvailable: FeatureStatus = {
    available: false,
    onDevice: false,
    requiresPermission: false,
  };

  return {
    speechRecognition: hasWebSpeechRecognition()
      ? {
          available: true,
          onDevice: false, // Web Speech API typically uses cloud
          requiresPermission: true,
        }
      : notAvailable,
    speechSynthesis: hasWebSpeechSynthesis()
      ? {
          available: true,
          onDevice: true,
          requiresPermission: false,
        }
      : notAvailable,
    textRecognition: notAvailable, // No standard web API for OCR
    barcodeDetection: hasBarcodeDetection()
      ? {
          available: true,
          onDevice: true,
          requiresPermission: false,
        }
      : notAvailable,
    faceDetection: notAvailable, // Shape Detection API face detection has very limited support
    imageClassification: notAvailable,
    languageIdentification: notAvailable,
    translation: notAvailable,
    languageModel: notAvailable, // No browser standard for on-device LLMs
  };
}

/**
 * Check if a specific feature is available.
 *
 * @param feature The feature to check.
 * @returns Whether the feature is available.
 */
export async function isFeatureAvailable(feature: keyof Capabilities): Promise<boolean> {
  const caps = await getCapabilities();
  return caps[feature]?.available ?? false;
}
