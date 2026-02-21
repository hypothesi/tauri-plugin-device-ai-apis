/**
 * Platform detection utilities.
 * @module platform
 */

/**
 * Check if we're running inside a Tauri application.
 */
export function isTauri(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}

/**
 * Check if we're running in a web browser (not Tauri).
 */
export function isWeb(): boolean {
  return typeof window !== "undefined" && !isTauri();
}

/**
 * Check if the Web Speech API is available.
 */
export function hasWebSpeechRecognition(): boolean {
  if (typeof window === "undefined") return false;
  return "SpeechRecognition" in window || "webkitSpeechRecognition" in window;
}

/**
 * Check if the Web Speech Synthesis API is available.
 */
export function hasWebSpeechSynthesis(): boolean {
  if (typeof window === "undefined") return false;
  return "speechSynthesis" in window;
}

/**
 * Check if the Barcode Detection API is available.
 */
export function hasBarcodeDetection(): boolean {
  if (typeof window === "undefined") return false;
  return "BarcodeDetector" in window;
}
