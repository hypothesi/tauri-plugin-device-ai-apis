/**
 * TypeScript type definitions for the device AI plugin.
 * @module types
 */

// =============================================================================
// Capability Types
// =============================================================================

/**
 * Status of a specific AI feature on the current platform.
 */
export interface FeatureStatus {
  /** Whether the feature is available on this platform. */
  available: boolean;
  /** Whether processing happens on-device (vs cloud). */
  onDevice: boolean;
  /** Whether the feature requires explicit permission from the user. */
  requiresPermission: boolean;
  /** List of supported language codes, if applicable. */
  supportedLanguages?: string[];
}

/**
 * Represents the available AI capabilities on the current platform.
 */
export interface Capabilities {
  /** Speech-to-text recognition capability. */
  speechRecognition: FeatureStatus;
  /** Text-to-speech synthesis capability. */
  speechSynthesis: FeatureStatus;
  /** Text recognition (OCR) in images. */
  textRecognition: FeatureStatus;
  /** Barcode and QR code detection. */
  barcodeDetection: FeatureStatus;
  /** Face detection in images. */
  faceDetection: FeatureStatus;
  /** Image classification/labeling. */
  imageClassification: FeatureStatus;
  /** Language identification of text. */
  languageIdentification: FeatureStatus;
  /** Text translation between languages. */
  translation: FeatureStatus;
  /** On-device language model for text generation. */
  languageModel: FeatureStatus;
}

// =============================================================================
// Speech Types
// =============================================================================

/**
 * Options for speech recognition.
 */
export interface RecognitionOptions {
  /** BCP-47 language code (e.g., "en-US", "fr-FR"). */
  language?: string;
  /** Whether to continue recognizing after the first result. */
  continuous?: boolean;
  /** Whether to report interim (partial) results. */
  interimResults?: boolean;
}

/**
 * An alternative transcription result.
 */
export interface RecognitionAlternative {
  /** The alternative text. */
  text: string;
  /** Confidence score from 0.0 to 1.0. */
  confidence: number;
}

/**
 * Result of speech recognition.
 */
export interface RecognitionResult {
  /** The recognized text. */
  text: string;
  /** Confidence score from 0.0 to 1.0. */
  confidence: number;
  /** Whether this is a final result or interim. */
  isFinal: boolean;
  /** Alternative transcriptions with their confidence scores. */
  alternatives: RecognitionAlternative[];
}

/**
 * Quality level of a voice.
 */
export type VoiceQuality = "default" | "enhanced" | "premium";

/**
 * Gender of a voice.
 */
export type VoiceGender = "male" | "female" | "neutral";

/**
 * Information about an available voice.
 */
export interface Voice {
  /** Unique identifier for the voice. */
  id: string;
  /** Human-readable name of the voice. */
  name: string;
  /** BCP-47 language code. */
  language: string;
  /** Whether this is the default voice for its language. */
  isDefault: boolean;
  /** Voice quality level. */
  quality?: VoiceQuality;
  /** Voice gender, if known. */
  gender?: VoiceGender;
}

/**
 * Options for text-to-speech synthesis.
 */
export interface SynthesisOptions {
  /** Voice identifier to use. */
  voice?: string;
  /** Speech rate multiplier (1.0 is normal speed). */
  rate?: number;
  /** Voice pitch multiplier (1.0 is normal pitch). */
  pitch?: number;
  /** Volume from 0.0 to 1.0. */
  volume?: number;
}

// =============================================================================
// Vision Types
// =============================================================================

/**
 * Source of an image for processing.
 * Can be base64-encoded data, raw bytes, or a file path.
 */
export type ImageSource = { base64: string } | { bytes: number[] } | { filePath: string };

/**
 * Bounding box for detected elements.
 * All values are normalized to 0.0-1.0 range.
 */
export interface BoundingBox {
  /** X coordinate of the top-left corner (0.0 to 1.0). */
  x: number;
  /** Y coordinate of the top-left corner (0.0 to 1.0). */
  y: number;
  /** Width as a fraction of image width (0.0 to 1.0). */
  width: number;
  /** Height as a fraction of image height (0.0 to 1.0). */
  height: number;
}

/**
 * Level of recognition accuracy vs speed.
 */
export type RecognitionLevel = "fast" | "accurate";

/**
 * Options for OCR/text recognition.
 */
export interface OcrOptions {
  /** Languages to prioritize for recognition. */
  languages?: string[];
  /** Recognition level (fast vs accurate). */
  recognitionLevel?: RecognitionLevel;
}

/**
 * A line of text within a text block.
 */
export interface TextLine {
  /** The text content of this line. */
  text: string;
  /** Bounding box of the text line. */
  boundingBox: BoundingBox;
}

/**
 * A block of text recognized in an image.
 */
export interface TextBlock {
  /** The text content of this block. */
  text: string;
  /** Bounding box of the text block. */
  boundingBox: BoundingBox;
  /** Individual lines within this block. */
  lines: TextLine[];
  /** Confidence score from 0.0 to 1.0. */
  confidence?: number;
}

/**
 * Result of text recognition (OCR).
 */
export interface TextRecognitionResult {
  /** Complete text found in the image. */
  text: string;
  /** Text blocks found in the image. */
  blocks: TextBlock[];
}

/**
 * Supported barcode formats.
 */
export type BarcodeFormat =
  | "qr_code"
  | "ean_13"
  | "ean_8"
  | "code_128"
  | "code_39"
  | "code_93"
  | "codabar"
  | "itf"
  | "upc_a"
  | "upc_e"
  | "pdf417"
  | "aztec"
  | "data_matrix";

/**
 * Options for barcode detection.
 */
export interface BarcodeOptions {
  /** Barcode formats to detect. If empty, all formats are detected. */
  formats?: BarcodeFormat[];
}

/**
 * A detected barcode.
 */
export interface Barcode {
  /** The barcode format. */
  format: BarcodeFormat;
  /** The decoded value. */
  rawValue: string;
  /** Bounding box of the barcode. */
  boundingBox: BoundingBox;
  /** Parsed data, if applicable (e.g., URL, contact info). */
  parsedData?: unknown;
}

/**
 * Options for face detection.
 */
export interface FaceOptions {
  /** Whether to detect facial landmarks. */
  detectLandmarks?: boolean;
  /** Whether to classify attributes (e.g., smiling, eyes open). */
  classifyAttributes?: boolean;
}

/**
 * A 2D point with normalized coordinates.
 */
export interface Point {
  /** X coordinate (0.0 to 1.0). */
  x: number;
  /** Y coordinate (0.0 to 1.0). */
  y: number;
}

/**
 * Facial landmark positions.
 */
export interface FaceLandmarks {
  /** Left eye position. */
  leftEye?: Point;
  /** Right eye position. */
  rightEye?: Point;
  /** Nose position. */
  nose?: Point;
  /** Left mouth corner. */
  mouthLeft?: Point;
  /** Right mouth corner. */
  mouthRight?: Point;
}

/**
 * Face attributes/classifications.
 */
export interface FaceAttributes {
  /** Probability that the face is smiling (0.0 to 1.0). */
  smilingProbability?: number;
  /** Probability that the left eye is open (0.0 to 1.0). */
  leftEyeOpenProbability?: number;
  /** Probability that the right eye is open (0.0 to 1.0). */
  rightEyeOpenProbability?: number;
}

/**
 * A detected face.
 */
export interface Face {
  /** Bounding box of the face. */
  boundingBox: BoundingBox;
  /** Facial landmarks, if requested. */
  landmarks?: FaceLandmarks;
  /** Face attributes, if classification was requested. */
  attributes?: FaceAttributes;
  /** Roll angle (rotation around the axis pointing out of the image). */
  rollAngle?: number;
  /** Yaw angle (rotation around the vertical axis). */
  yawAngle?: number;
}

/**
 * Options for image classification.
 */
export interface ClassificationOptions {
  /** Maximum number of classifications to return. */
  maxResults?: number;
  /** Minimum confidence threshold (0.0 to 1.0). */
  minConfidence?: number;
}

/**
 * A classification/label for an image.
 */
export interface Classification {
  /** The label/category identifier. */
  identifier: string;
  /** Confidence score from 0.0 to 1.0. */
  confidence: number;
}

// =============================================================================
// Text Processing Types
// =============================================================================

/**
 * An alternative language identification result.
 */
export interface LanguageAlternative {
  /** Language code. */
  language: string;
  /** Confidence score from 0.0 to 1.0. */
  confidence: number;
}

/**
 * Result of language identification.
 */
export interface LanguageIdentification {
  /** The most likely language code. */
  language: string;
  /** Confidence score from 0.0 to 1.0. */
  confidence: number;
  /** Alternative language guesses. */
  alternatives: LanguageAlternative[];
}

/**
 * Result of text translation.
 */
export interface Translation {
  /** The translated text. */
  translatedText: string;
  /** Source language code. */
  sourceLanguage: string;
  /** Target language code. */
  targetLanguage: string;
}

// =============================================================================
// LLM Types
// =============================================================================

/**
 * Whether an on-device language model is available.
 */
export interface LlmAvailability {
  /** Whether a language model is available. */
  available: boolean;
  /** Reason the model is unavailable, if applicable. */
  reason?: string;
}

/**
 * Capabilities of the on-device language model.
 */
export interface LlmModelCapabilities {
  /** Whether streaming generation is supported. */
  streaming: boolean;
  /** Whether system prompts are supported. */
  systemPrompts: boolean;
  /** Whether temperature control is supported. */
  temperatureControl: boolean;
  /** Whether max tokens control is supported. */
  maxTokensControl: boolean;
  /** Whether seed-based reproducibility is supported. */
  seedSupport: boolean;
  /** Whether top-p sampling is supported. */
  topPSupport: boolean;
  /** Whether top-k sampling is supported. */
  topKSupport: boolean;
  /** Whether text summarization is supported. */
  summarize: boolean;
  /** Whether text rewriting is supported. */
  rewrite: boolean;
}

/**
 * Information about the on-device language model.
 */
export interface LlmModelInfo {
  /** Unique model identifier. */
  id: string;
  /** Human-readable model name. */
  name: string;
  /** Model provider identifier. */
  provider: string;
  /** Maximum context window in tokens. */
  contextWindow: number;
  /** Whether the model runs entirely on-device. */
  onDevice: boolean;
  /** Model capabilities. */
  capabilities: LlmModelCapabilities;
}

/**
 * Options for text generation.
 */
export interface LlmGenerateOptions {
  /** The prompt to generate from. */
  prompt: string;
  /** Optional system prompt to guide the model's behavior. */
  systemPrompt?: string;
  /** Sampling temperature (0.0 to 2.0). */
  temperature?: number;
  /** Maximum number of tokens to generate. */
  maxTokens?: number;
  /** Top-p (nucleus) sampling parameter. */
  topP?: number;
  /** Top-k sampling parameter. */
  topK?: number;
  /** Random seed for reproducible generation. */
  seed?: number;
}

/**
 * Reason a generation finished.
 */
export type LlmFinishReason = "stop" | "length" | "safety" | "error";

/**
 * Token usage statistics.
 */
export interface LlmUsage {
  /** Number of tokens in the prompt. */
  promptTokens?: number;
  /** Number of tokens generated. */
  completionTokens?: number;
  /** Total tokens used. */
  totalTokens?: number;
}

/**
 * Result of a text generation request.
 */
export interface LlmGenerateResult {
  /** The generated text content. */
  content: string;
  /** The model that generated the response. */
  model: string;
  /** Reason the generation finished. */
  finishReason: LlmFinishReason;
  /** Token usage statistics, if available. */
  usage?: LlmUsage;
}

/**
 * A streaming event from text generation.
 */
export type LlmStreamEvent =
  | { type: "delta"; content: string }
  | { type: "done"; content: string; finishReason: LlmFinishReason; usage?: LlmUsage }
  | { type: "error"; message: string };

/**
 * Options for creating a multi-turn session.
 */
export interface LlmSessionOptions {
  /** Optional system prompt for the session. */
  systemPrompt?: string;
  /** Default temperature for the session. */
  temperature?: number;
  /** Default max tokens for the session. */
  maxTokens?: number;
}

/**
 * Options for text summarization.
 */
export interface LlmSummarizeOptions {
  /** The text to summarize. */
  text: string;
}

/**
 * Result of text summarization.
 */
export interface LlmSummarizeResult {
  /** The summarized text. */
  summary: string;
  /** The model that generated the summary. */
  model: string;
}

/**
 * Tone for text rewriting.
 */
export type LlmRewriteTone = "casual" | "formal" | "professional";

/**
 * Options for text rewriting.
 */
export interface LlmRewriteOptions {
  /** The text to rewrite. */
  text: string;
  /** The desired tone for the rewritten text. */
  tone?: LlmRewriteTone;
}

/**
 * Result of text rewriting.
 */
export interface LlmRewriteResult {
  /** The rewritten text. */
  rewrittenText: string;
  /** The model that generated the rewrite. */
  model: string;
}
