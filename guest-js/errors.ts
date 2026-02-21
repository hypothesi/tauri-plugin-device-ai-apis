/**
 * Error types for the device AI plugin.
 * @module errors
 */

/**
 * Error codes returned by the device AI plugin.
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
  | "TEXT_PROCESSING_FAILED"
  | "TRANSLATION_FAILED"
  | "PLATFORM_ERROR"
  | "IO_ERROR"
  | "PLUGIN_INVOKE_ERROR"
  | "LLM_NOT_AVAILABLE"
  | "LLM_GENERATION_FAILED"
  | "LLM_SESSION_NOT_FOUND"
  | "LLM_CONTENT_FILTERED"
  | "UNKNOWN";

/**
 * Error response from the device AI plugin.
 */
export interface DeviceAiError {
  /** Error code for programmatic handling. */
  code: DeviceAiErrorCode;
  /** Human-readable error message. */
  message: string;
}

/**
 * Check if an error is a DeviceAiError.
 */
export function isDeviceAiError(error: unknown): error is DeviceAiError {
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
 * Check if a feature is not available based on the error.
 */
export function isFeatureNotAvailable(error: unknown): boolean {
  return isDeviceAiError(error) && error.code === "FEATURE_NOT_AVAILABLE";
}

/**
 * Check if permission is required or denied based on the error.
 */
export function isPermissionError(error: unknown): boolean {
  return (
    isDeviceAiError(error) &&
    (error.code === "PERMISSION_REQUIRED" || error.code === "PERMISSION_DENIED")
  );
}
