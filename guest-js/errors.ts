/**
 * Error types and helpers for the device AI plugin.
 * @module errors
 */

/**
 * Programmatic error codes returned by the device AI plugin.
 */
export type DeviceAiErrorCode =
  | "FEATURE_NOT_AVAILABLE"
  | "PERMISSION_REQUIRED"
  | "PERMISSION_DENIED"
  | "SPEECH_RECOGNITION_FAILED"
  | "SPEECH_SYNTHESIS_FAILED"
  | "LANGUAGE_NOT_SUPPORTED"
  | "NO_SPEECH_DETECTED"
  | "INVALID_SESSION_ID"
  | "IMAGE_PROCESSING_FAILED"
  | "INVALID_IMAGE_FORMAT"
  | "INVALID_IMAGE_DATA"
  | "INVALID_ARGUMENTS"
  | "TEXT_PROCESSING_FAILED"
  | "TRANSLATION_FAILED"
  | "MODEL_NOT_INSTALLED"
  | "PLATFORM_ERROR"
  | "IO_ERROR"
  | "PLUGIN_INVOKE_ERROR"
  | "LLM_NOT_AVAILABLE"
  | "LLM_GENERATION_FAILED"
  | "LLM_SESSION_NOT_FOUND"
  | "LLM_CONTENT_FILTERED"
  | "LLM_CONTEXT_EXCEEDED"
  | "UNKNOWN";

/**
 * Structured programmatic metadata attached to a DeviceAiError.
 */
export interface DeviceAiErrorDetails {
  /** The feature name that is unavailable or caused an error. */
  feature?: string;
  /** The permission name that was required or denied (e.g. 'microphone', 'speechRecognition'). */
  permission?: string;
  /** The language code that is not supported. */
  language?: string;
  /** The source language code (e.g. for translation). */
  sourceLanguage?: string;
  /** The target language code (e.g. for translation). */
  targetLanguage?: string;
  /** The model type that is missing (e.g. 'translation', 'llm'). */
  modelType?: string;
  /** The session identifier that was invalid or not found. */
  sessionId?: string;
  /** The technical reason for failure or unavailability. */
  reason?: string;
  /** The expected format or parameter. */
  expected?: string;
  /** The actual format or parameter received. */
  actual?: string;
  /** Additional platform-specific error properties. */
  [key: string]: unknown;
}

/**
 * Structured error returned by the device AI plugin.
 */
export interface DeviceAiError {
  /** Error code for programmatic handling. */
  code: DeviceAiErrorCode;
  /** Technical error message. */
  message: string;
  /** Programmatic metadata for caller handling and recovery. */
  details?: DeviceAiErrorDetails;
}

/**
 * Normalize an error into a structured DeviceAiError.
 *
 * Handles native error objects, plain Error instances, and JSON-serialized
 * error strings returned by mobile plugin invokes.
 */
export function normalizeDeviceAiError(error: unknown): DeviceAiError {
  if (typeof error === "string") {
    try {
      const parsed = JSON.parse(error);
      if (typeof parsed === "object" && parsed !== null && "code" in parsed) {
        return parsed as DeviceAiError;
      }
    } catch {
      // Non-JSON string error
    }
    return {
      code: "UNKNOWN",
      message: error,
    };
  }

  if (isDeviceAiError(error)) {
    return error;
  }

  if (error instanceof Error) {
    return {
      code: "UNKNOWN",
      message: error.message,
    };
  }

  return {
    code: "UNKNOWN",
    message: String(error),
  };
}

/**
 * Check if an error represents a DeviceAiError.
 */
export function isDeviceAiError(error: unknown): error is DeviceAiError {
  if (typeof error === "string") {
    try {
      const parsed = JSON.parse(error);
      return (
        typeof parsed === "object" &&
        parsed !== null &&
        "code" in parsed &&
        "message" in parsed &&
        typeof (parsed as DeviceAiError).code === "string" &&
        typeof (parsed as DeviceAiError).message === "string"
      );
    } catch {
      return false;
    }
  }

  return (
    typeof error === "object" &&
    error !== null &&
    "code" in error &&
    "message" in error &&
    typeof (error as DeviceAiError).code === "string" &&
    typeof (error as DeviceAiError).message === "string"
  );
}

/**
 * Check if a feature is unavailable based on the error.
 */
export function isFeatureNotAvailable(error: unknown): boolean {
  return normalizeDeviceAiError(error).code === "FEATURE_NOT_AVAILABLE";
}

/**
 * Check if permission is required or denied based on the error.
 */
export function isPermissionError(error: unknown): boolean {
  const code = normalizeDeviceAiError(error).code;
  return code === "PERMISSION_REQUIRED" || code === "PERMISSION_DENIED";
}

/**
 * Check if an on-device model or language pack is not installed.
 */
export function isModelNotInstalled(error: unknown): boolean {
  return normalizeDeviceAiError(error).code === "MODEL_NOT_INSTALLED";
}

/**
 * Check if a language or language pair is not supported.
 */
export function isLanguageNotSupported(error: unknown): boolean {
  return normalizeDeviceAiError(error).code === "LANGUAGE_NOT_SUPPORTED";
}

/**
 * Check if no speech was detected during recognition.
 */
export function isNoSpeechDetected(error: unknown): boolean {
  return normalizeDeviceAiError(error).code === "NO_SPEECH_DETECTED";
}

/**
 * Check if the language model context window was exceeded based on the error.
 */
export function isLlmContextExceeded(error: unknown): boolean {
  return normalizeDeviceAiError(error).code === "LLM_CONTEXT_EXCEEDED";
}
