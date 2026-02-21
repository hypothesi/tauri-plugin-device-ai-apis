/**
 * tauri-plugin-device-ai-apis
 *
 * A Tauri plugin that provides cross-platform access to device-native AI capabilities
 * including speech recognition, text-to-speech, vision (OCR, barcode, face detection),
 * and natural language processing.
 *
 * @packageDocumentation
 */

// Capability detection
export { getCapabilities, isFeatureAvailable } from "./capabilities";

// Speech APIs
export * as speech from "./speech";

// Vision APIs
export * as vision from "./vision";

// Text processing APIs
export * as text from "./text";

// LLM APIs
export * as llm from "./llm";

// Platform detection utilities
export {
  isTauri,
  isWeb,
  hasWebSpeechRecognition,
  hasWebSpeechSynthesis,
  hasBarcodeDetection,
} from "./platform";

// Types
export type {
  // Capabilities
  Capabilities,
  FeatureStatus,
  // Speech
  RecognitionOptions,
  RecognitionResult,
  RecognitionAlternative,
  SynthesisOptions,
  Voice,
  VoiceQuality,
  VoiceGender,
  // Vision
  ImageSource,
  BoundingBox,
  OcrOptions,
  RecognitionLevel,
  TextRecognitionResult,
  TextBlock,
  TextLine,
  BarcodeFormat,
  BarcodeOptions,
  Barcode,
  FaceOptions,
  Face,
  FaceLandmarks,
  FaceAttributes,
  Point,
  ClassificationOptions,
  Classification,
  // Text
  LanguageIdentification,
  LanguageAlternative,
  Translation,
  // LLM
  LlmAvailability,
  LlmModelCapabilities,
  LlmModelInfo,
  LlmGenerateOptions,
  LlmFinishReason,
  LlmUsage,
  LlmGenerateResult,
  LlmStreamEvent,
  LlmSessionOptions,
  LlmSummarizeOptions,
  LlmSummarizeResult,
  LlmRewriteTone,
  LlmRewriteOptions,
  LlmRewriteResult,
} from "./types";

// Errors
export { isDeviceAiError, isFeatureNotAvailable, isPermissionError } from "./errors";
export type { DeviceAiError, DeviceAiErrorCode } from "./errors";
