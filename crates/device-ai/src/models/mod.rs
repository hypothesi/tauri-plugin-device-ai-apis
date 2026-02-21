//! Data models and types for the device AI plugin.
//!
//! This module contains all the shared data structures used across
//! the plugin's API surface.

use serde::{Deserialize, Serialize};

// =============================================================================
// Capability Types
// =============================================================================

/// Represents the available AI capabilities on the current platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    /// Speech-to-text recognition capability.
    pub speech_recognition: FeatureStatus,
    /// Text-to-speech synthesis capability.
    pub speech_synthesis: FeatureStatus,
    /// Text recognition (OCR) in images.
    pub text_recognition: FeatureStatus,
    /// Barcode and QR code detection.
    pub barcode_detection: FeatureStatus,
    /// Face detection in images.
    pub face_detection: FeatureStatus,
    /// Image classification/labeling.
    pub image_classification: FeatureStatus,
    /// Language identification of text.
    pub language_identification: FeatureStatus,
    /// Text translation between languages.
    pub translation: FeatureStatus,
    /// On-device language model (LLM) capability.
    pub language_model: FeatureStatus,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            speech_recognition: FeatureStatus::unavailable(),
            speech_synthesis: FeatureStatus::unavailable(),
            text_recognition: FeatureStatus::unavailable(),
            barcode_detection: FeatureStatus::unavailable(),
            face_detection: FeatureStatus::unavailable(),
            image_classification: FeatureStatus::unavailable(),
            language_identification: FeatureStatus::unavailable(),
            translation: FeatureStatus::unavailable(),
            language_model: FeatureStatus::unavailable(),
        }
    }
}

/// Status of a specific AI feature on the current platform.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeatureStatus {
    /// Whether the feature is available on this platform.
    pub available: bool,
    /// Whether processing happens on-device (vs cloud).
    pub on_device: bool,
    /// Whether the feature requires explicit permission from the user.
    pub requires_permission: bool,
    /// List of supported language codes, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_languages: Option<Vec<String>>,
}

impl FeatureStatus {
    /// Create a status indicating the feature is unavailable.
    pub fn unavailable() -> Self {
        Self {
            available: false,
            on_device: false,
            requires_permission: false,
            supported_languages: None,
        }
    }

    /// Create a status indicating the feature is available on-device.
    pub fn available_on_device(requires_permission: bool) -> Self {
        Self {
            available: true,
            on_device: true,
            requires_permission,
            supported_languages: None,
        }
    }

    /// Add supported languages to this feature status.
    pub fn with_languages(mut self, languages: Vec<String>) -> Self {
        self.supported_languages = Some(languages);
        self
    }
}

// =============================================================================
// Speech Types
// =============================================================================

/// Unique identifier for a speech recognition session.
pub type SpeechSessionId = String;

/// Options for speech recognition.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecognitionOptions {
    /// BCP-47 language code (e.g., "en-US", "fr-FR").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Whether to continue recognizing after the first result.
    #[serde(default)]
    pub continuous: bool,
    /// Whether to report interim (partial) results.
    #[serde(default)]
    pub interim_results: bool,
    /// Audio source for recognition (microphone or file).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_source: Option<AudioSource>,
}

impl RecognitionOptions {
    /// Create default recognition options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the recognition language.
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Control whether recognition continues after the first result.
    pub fn continuous(mut self, continuous: bool) -> Self {
        self.continuous = continuous;
        self
    }

    /// Control whether interim results are emitted.
    pub fn interim_results(mut self, interim_results: bool) -> Self {
        self.interim_results = interim_results;
        self
    }

    /// Set the audio source.
    pub fn with_audio_source(mut self, audio_source: AudioSource) -> Self {
        self.audio_source = Some(audio_source);
        self
    }
}

/// Source of audio for speech recognition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AudioSource {
    /// Use the device microphone (live input).
    Microphone,
    /// Base64-encoded audio file data.
    Base64(String),
    /// Raw audio bytes (for internal use).
    Bytes(Vec<u8>),
    /// File path to the audio file.
    FilePath(String),
}

impl AudioSource {
    /// Use the microphone as the audio source.
    pub fn microphone() -> Self {
        Self::Microphone
    }

    /// Create an audio source from base64 data.
    pub fn from_base64(data: impl Into<String>) -> Self {
        Self::Base64(data.into())
    }

    /// Create an audio source from raw bytes.
    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self::Bytes(bytes.into())
    }

    /// Create an audio source from a file path.
    pub fn from_path(path: impl Into<String>) -> Self {
        Self::FilePath(path.into())
    }
}

/// Result of speech recognition.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecognitionResult {
    /// The recognized text.
    pub text: String,
    /// Confidence score from 0.0 to 1.0.
    pub confidence: f32,
    /// Whether this is a final result or interim.
    pub is_final: bool,
    /// Alternative transcriptions with their confidence scores.
    #[serde(default)]
    pub alternatives: Vec<RecognitionAlternative>,
}

/// An alternative transcription result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecognitionAlternative {
    /// The alternative text.
    pub text: String,
    /// Confidence score from 0.0 to 1.0.
    pub confidence: f32,
}

/// Options for text-to-speech synthesis.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SynthesisOptions {
    /// Voice identifier to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<String>,
    /// Speech rate multiplier (1.0 is normal speed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<f32>,
    /// Voice pitch multiplier (1.0 is normal pitch).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<f32>,
    /// Volume from 0.0 to 1.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<f32>,
}

impl SynthesisOptions {
    /// Create default synthesis options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Select a voice by ID.
    pub fn with_voice(mut self, voice: impl Into<String>) -> Self {
        self.voice = Some(voice.into());
        self
    }

    /// Set the speaking rate multiplier.
    pub fn rate(mut self, rate: f32) -> Self {
        self.rate = Some(rate);
        self
    }

    /// Set the pitch multiplier.
    pub fn pitch(mut self, pitch: f32) -> Self {
        self.pitch = Some(pitch);
        self
    }

    /// Set the output volume.
    pub fn volume(mut self, volume: f32) -> Self {
        self.volume = Some(volume);
        self
    }
}

/// Information about an available voice.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Voice {
    /// Unique identifier for the voice.
    pub id: String,
    /// Human-readable name of the voice.
    pub name: String,
    /// BCP-47 language code.
    pub language: String,
    /// Whether this is the default voice for its language.
    #[serde(default)]
    pub is_default: bool,
    /// Voice quality level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<VoiceQuality>,
    /// Voice gender, if known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<VoiceGender>,
}

/// Quality level of a voice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VoiceQuality {
    /// Standard quality voice.
    Default,
    /// Enhanced quality voice.
    Enhanced,
    /// Premium quality voice.
    Premium,
}

/// Gender of a voice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VoiceGender {
    Male,
    Female,
    Neutral,
}

// =============================================================================
// Vision Types
// =============================================================================

/// Source of an image for processing.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ImageSource {
    /// Base64-encoded image data.
    Base64(String),
    /// Raw bytes (for internal use, serialized as array).
    Bytes(Vec<u8>),
    /// File path to the image.
    FilePath(String),
}

impl ImageSource {
    /// Create an image source from base64 data.
    pub fn from_base64(data: impl Into<String>) -> Self {
        Self::Base64(data.into())
    }

    /// Create an image source from raw bytes.
    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self::Bytes(bytes.into())
    }

    /// Create an image source from a file path.
    pub fn from_path(path: impl Into<String>) -> Self {
        Self::FilePath(path.into())
    }
}

/// Bounding box for detected elements.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BoundingBox {
    /// X coordinate of the top-left corner (0.0 to 1.0).
    pub x: f32,
    /// Y coordinate of the top-left corner (0.0 to 1.0).
    pub y: f32,
    /// Width as a fraction of image width (0.0 to 1.0).
    pub width: f32,
    /// Height as a fraction of image height (0.0 to 1.0).
    pub height: f32,
}

/// Options for OCR/text recognition.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcrOptions {
    /// Languages to prioritize for recognition.
    #[serde(default)]
    pub languages: Vec<String>,
    /// Recognition level (fast vs accurate).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recognition_level: Option<RecognitionLevel>,
}

impl OcrOptions {
    /// Create default OCR options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a preferred language.
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.languages.push(language.into());
        self
    }

    /// Replace the preferred language list.
    pub fn with_languages(
        mut self,
        languages: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.languages = languages.into_iter().map(Into::into).collect();
        self
    }

    /// Set the recognition level.
    pub fn recognition_level(mut self, recognition_level: RecognitionLevel) -> Self {
        self.recognition_level = Some(recognition_level);
        self
    }
}

/// Level of recognition accuracy vs speed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RecognitionLevel {
    /// Fast recognition, may be less accurate.
    Fast,
    /// Accurate recognition, may be slower.
    Accurate,
}

/// Result of text recognition (OCR).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextRecognitionResult {
    /// Complete text found in the image.
    pub text: String,
    /// Text blocks found in the image.
    #[serde(default)]
    pub blocks: Vec<TextBlock>,
}

/// A block of text recognized in an image.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextBlock {
    /// The text content of this block.
    pub text: String,
    /// Bounding box of the text block.
    pub bounding_box: BoundingBox,
    /// Individual lines within this block.
    #[serde(default)]
    pub lines: Vec<TextLine>,
    /// Confidence score from 0.0 to 1.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
}

/// A line of text within a text block.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextLine {
    /// The text content of this line.
    pub text: String,
    /// Bounding box of the text line.
    pub bounding_box: BoundingBox,
}

/// Options for barcode detection.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BarcodeOptions {
    /// Barcode formats to detect. If empty, all formats are detected.
    #[serde(default)]
    pub formats: Vec<BarcodeFormat>,
}

impl BarcodeOptions {
    /// Create default barcode-detection options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a supported barcode format.
    pub fn with_format(mut self, format: BarcodeFormat) -> Self {
        self.formats.push(format);
        self
    }

    /// Replace the format list.
    pub fn with_formats(mut self, formats: impl IntoIterator<Item = BarcodeFormat>) -> Self {
        self.formats = formats.into_iter().collect();
        self
    }
}

/// Supported barcode formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BarcodeFormat {
    QrCode,
    #[serde(rename = "ean_13")]
    Ean13,
    #[serde(rename = "ean_8")]
    Ean8,
    #[serde(rename = "code_128")]
    Code128,
    #[serde(rename = "code_39")]
    Code39,
    #[serde(rename = "code_93")]
    Code93,
    Codabar,
    Itf,
    #[serde(rename = "upc_a")]
    UpcA,
    #[serde(rename = "upc_e")]
    UpcE,
    #[serde(rename = "pdf417")]
    Pdf417,
    Aztec,
    DataMatrix,
}

/// A detected barcode.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Barcode {
    /// The barcode format.
    pub format: BarcodeFormat,
    /// The decoded value.
    pub raw_value: String,
    /// Bounding box of the barcode.
    pub bounding_box: BoundingBox,
    /// Parsed data, if applicable (e.g., URL, contact info).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsed_data: Option<serde_json::Value>,
}

/// Options for face detection.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaceOptions {
    /// Whether to detect facial landmarks.
    #[serde(default)]
    pub detect_landmarks: bool,
    /// Whether to classify attributes (e.g., smiling, eyes open).
    #[serde(default)]
    pub classify_attributes: bool,
}

impl FaceOptions {
    /// Create default face-detection options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Toggle landmark detection.
    pub fn detect_landmarks(mut self, detect_landmarks: bool) -> Self {
        self.detect_landmarks = detect_landmarks;
        self
    }

    /// Toggle attribute classification.
    pub fn classify_attributes(mut self, classify_attributes: bool) -> Self {
        self.classify_attributes = classify_attributes;
        self
    }
}

/// A detected face.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Face {
    /// Bounding box of the face.
    pub bounding_box: BoundingBox,
    /// Facial landmarks, if requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub landmarks: Option<FaceLandmarks>,
    /// Face attributes, if classification was requested.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<FaceAttributes>,
    /// Roll angle (rotation around the axis pointing out of the image).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roll_angle: Option<f32>,
    /// Yaw angle (rotation around the vertical axis).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yaw_angle: Option<f32>,
}

/// Facial landmark positions.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaceLandmarks {
    /// Left eye position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_eye: Option<Point>,
    /// Right eye position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_eye: Option<Point>,
    /// Nose position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nose: Option<Point>,
    /// Left mouth corner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouth_left: Option<Point>,
    /// Right mouth corner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mouth_right: Option<Point>,
}

/// A 2D point with normalized coordinates.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Point {
    /// X coordinate (0.0 to 1.0).
    pub x: f32,
    /// Y coordinate (0.0 to 1.0).
    pub y: f32,
}

/// Face attributes/classifications.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaceAttributes {
    /// Probability that the face is smiling (0.0 to 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smiling_probability: Option<f32>,
    /// Probability that the left eye is open (0.0 to 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_eye_open_probability: Option<f32>,
    /// Probability that the right eye is open (0.0 to 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right_eye_open_probability: Option<f32>,
}

/// Options for image classification.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClassificationOptions {
    /// Maximum number of classifications to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<u32>,
    /// Minimum confidence threshold (0.0 to 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_confidence: Option<f32>,
}

impl ClassificationOptions {
    /// Create default image-classification options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Limit the number of results returned.
    pub fn max_results(mut self, max_results: u32) -> Self {
        self.max_results = Some(max_results);
        self
    }

    /// Set the minimum confidence threshold.
    pub fn min_confidence(mut self, min_confidence: f32) -> Self {
        self.min_confidence = Some(min_confidence);
        self
    }
}

/// A classification/label for an image.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Classification {
    /// The label/category identifier.
    pub identifier: String,
    /// Confidence score from 0.0 to 1.0.
    pub confidence: f32,
}

// =============================================================================
// Text Processing Types
// =============================================================================

/// Result of language identification.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageIdentification {
    /// The most likely language code.
    pub language: String,
    /// Confidence score from 0.0 to 1.0.
    pub confidence: f32,
    /// Alternative language guesses.
    #[serde(default)]
    pub alternatives: Vec<LanguageAlternative>,
}

/// An alternative language identification result.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LanguageAlternative {
    /// Language code.
    pub language: String,
    /// Confidence score from 0.0 to 1.0.
    pub confidence: f32,
}

/// Result of text translation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Translation {
    /// The translated text.
    pub translated_text: String,
    /// Source language code.
    pub source_language: String,
    /// Target language code.
    pub target_language: String,
}

// =============================================================================
// Language Model (LLM) Types
// =============================================================================

/// Whether the on-device language model is available.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmAvailability {
    /// Whether the language model is available.
    pub available: bool,
    /// Reason the model is unavailable, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Capabilities of the on-device language model.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmModelCapabilities {
    /// Whether streaming generation is supported.
    pub streaming: bool,
    /// Whether system prompts are supported.
    pub system_prompts: bool,
    /// Whether temperature control is supported.
    pub temperature_control: bool,
    /// Whether max token limit is supported.
    pub max_tokens_control: bool,
    /// Whether seed-based reproducibility is supported.
    pub seed_support: bool,
    /// Whether top-p sampling is supported.
    pub top_p_support: bool,
    /// Whether top-k sampling is supported.
    pub top_k_support: bool,
    /// Whether text summarization is supported.
    pub summarize: bool,
    /// Whether text rewriting is supported.
    pub rewrite: bool,
}

/// Information about the on-device language model.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmModelInfo {
    /// Model identifier.
    pub id: String,
    /// Human-readable model name.
    pub name: String,
    /// Provider identifier (e.g., "apple-foundationmodels", "microsoft-phi-silica").
    pub provider: String,
    /// Maximum context window size in tokens.
    pub context_window: u32,
    /// Whether all processing is on-device.
    pub on_device: bool,
    /// Model capabilities.
    pub capabilities: LlmModelCapabilities,
}

/// Options for language model text generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmGenerateOptions {
    /// The prompt to generate text from.
    pub prompt: String,
    /// Optional system prompt to set model behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    /// Sampling temperature (higher = more random). Typical range: 0.0–2.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Maximum number of tokens to generate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// Top-p (nucleus) sampling parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    /// Top-k sampling parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<u32>,
    /// Random seed for reproducible output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<u64>,
}

impl LlmGenerateOptions {
    /// Create generation options with the required prompt.
    pub fn new(prompt: impl Into<String>) -> Self {
        Self {
            prompt: prompt.into(),
            system_prompt: None,
            temperature: None,
            max_tokens: None,
            top_p: None,
            top_k: None,
            seed: None,
        }
    }

    /// Set the optional system prompt.
    pub fn system_prompt(mut self, system_prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(system_prompt.into());
        self
    }

    /// Set the sampling temperature.
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set the maximum token count.
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    /// Set top-p sampling.
    pub fn top_p(mut self, top_p: f64) -> Self {
        self.top_p = Some(top_p);
        self
    }

    /// Set top-k sampling.
    pub fn top_k(mut self, top_k: u32) -> Self {
        self.top_k = Some(top_k);
        self
    }

    /// Set a deterministic seed.
    pub fn seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }
}

/// Why the model stopped generating.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LlmFinishReason {
    /// Normal completion.
    Stop,
    /// Hit the max token limit.
    Length,
    /// Content filter triggered.
    Safety,
    /// An error occurred during generation.
    Error,
}

/// Token usage statistics for a generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmUsage {
    /// Number of tokens in the prompt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens: Option<u32>,
    /// Number of tokens generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<u32>,
    /// Total tokens (prompt + completion).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<u32>,
}

/// Result of a non-streaming language model generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmGenerateResult {
    /// The generated text.
    pub content: String,
    /// Model identifier that produced this result.
    pub model: String,
    /// Why the model stopped generating.
    pub finish_reason: LlmFinishReason,
    /// Token usage statistics, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<LlmUsage>,
}

/// A streaming event from language model generation.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum LlmStreamEvent {
    /// An incremental token/chunk of generated text.
    Delta {
        /// The new text chunk.
        content: String,
    },
    /// Generation is complete.
    Done {
        /// The full generated text.
        content: String,
        /// Why the model stopped generating.
        #[serde(rename = "finishReason")]
        finish_reason: LlmFinishReason,
        /// Token usage statistics, if available.
        #[serde(skip_serializing_if = "Option::is_none")]
        usage: Option<LlmUsage>,
    },
    /// An error occurred during generation.
    Error {
        /// Error description.
        message: String,
    },
}

/// Options for creating a multi-turn language model session.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmSessionOptions {
    /// Optional system prompt to set model behavior for the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    /// Sampling temperature for the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// Maximum tokens per response in this session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

impl LlmSessionOptions {
    /// Create default session options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the optional system prompt.
    pub fn system_prompt(mut self, system_prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(system_prompt.into());
        self
    }

    /// Set the session temperature.
    pub fn temperature(mut self, temperature: f64) -> Self {
        self.temperature = Some(temperature);
        self
    }

    /// Set the per-response maximum token count.
    pub fn max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }
}

/// Unique identifier for a language model session.
pub type LlmSessionId = String;

/// Options for text summarization.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmSummarizeOptions {
    /// The text to summarize.
    pub text: String,
}

impl LlmSummarizeOptions {
    /// Create summarize options with the required text.
    pub fn new(text: impl Into<String>) -> Self {
        Self { text: text.into() }
    }
}

/// Result of text summarization.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmSummarizeResult {
    /// The summarized text.
    pub summary: String,
    /// Model identifier that produced this result.
    pub model: String,
}

/// Tone for text rewriting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LlmRewriteTone {
    /// Casual, informal tone.
    Casual,
    /// Formal tone.
    Formal,
    /// Professional business tone.
    Professional,
}

/// Options for text rewriting.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmRewriteOptions {
    /// The text to rewrite.
    pub text: String,
    /// Target tone for the rewritten text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tone: Option<LlmRewriteTone>,
}

impl LlmRewriteOptions {
    /// Create rewrite options with the required text.
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            tone: None,
        }
    }

    /// Set the target tone.
    pub fn tone(mut self, tone: LlmRewriteTone) -> Self {
        self.tone = Some(tone);
        self
    }
}

/// Result of text rewriting.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LlmRewriteResult {
    /// The rewritten text.
    pub rewritten_text: String,
    /// Model identifier that produced this result.
    pub model: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // FeatureStatus Tests
    // =========================================================================

    #[test]
    fn test_feature_status_unavailable() {
        let status = FeatureStatus::unavailable();
        assert!(!status.available);
        assert!(!status.on_device);
        assert!(!status.requires_permission);
        assert!(status.supported_languages.is_none());
    }

    #[test]
    fn test_feature_status_available_on_device() {
        let status = FeatureStatus::available_on_device(true);
        assert!(status.available);
        assert!(status.on_device);
        assert!(status.requires_permission);
    }

    #[test]
    fn test_feature_status_with_languages() {
        let status = FeatureStatus::available_on_device(false)
            .with_languages(vec!["en-US".to_string(), "fr-FR".to_string()]);
        assert!(status.available);
        assert_eq!(status.supported_languages.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_feature_status_serialization() {
        let status =
            FeatureStatus::available_on_device(true).with_languages(vec!["en-US".to_string()]);
        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("\"available\":true"));
        assert!(json.contains("\"onDevice\":true"));
        assert!(json.contains("\"requiresPermission\":true"));
        assert!(json.contains("\"supportedLanguages\""));
    }

    // =========================================================================
    // Capabilities Tests
    // =========================================================================

    #[test]
    fn test_capabilities_default_all_unavailable() {
        let caps = Capabilities::default();
        assert!(!caps.speech_recognition.available);
        assert!(!caps.speech_synthesis.available);
        assert!(!caps.text_recognition.available);
        assert!(!caps.barcode_detection.available);
        assert!(!caps.face_detection.available);
        assert!(!caps.image_classification.available);
        assert!(!caps.language_identification.available);
        assert!(!caps.translation.available);
        assert!(!caps.language_model.available);
    }

    #[test]
    fn test_capabilities_serialization() {
        let caps = Capabilities::default();
        let json = serde_json::to_string(&caps).unwrap();
        assert!(json.contains("speechRecognition"));
        assert!(json.contains("speechSynthesis"));
        assert!(json.contains("textRecognition"));
        assert!(json.contains("barcodeDetection"));
    }

    // =========================================================================
    // Speech Types Tests
    // =========================================================================

    #[test]
    fn test_recognition_options_default() {
        let opts = RecognitionOptions::default();
        assert!(opts.language.is_none());
        assert!(!opts.continuous);
        assert!(!opts.interim_results);
    }

    #[test]
    fn test_recognition_options_deserialization() {
        let json = r#"{"language":"en-US","continuous":true,"interimResults":true}"#;
        let opts: RecognitionOptions = serde_json::from_str(json).unwrap();
        assert_eq!(opts.language.as_deref(), Some("en-US"));
        assert!(opts.continuous);
        assert!(opts.interim_results);
    }

    #[test]
    fn test_recognition_result_serialization() {
        let result = RecognitionResult {
            text: "Hello world".to_string(),
            confidence: 0.95,
            is_final: true,
            alternatives: vec![RecognitionAlternative {
                text: "Hello world".to_string(),
                confidence: 0.95,
            }],
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"text\":\"Hello world\""));
        assert!(json.contains("\"confidence\":0.95"));
        assert!(json.contains("\"isFinal\":true"));
    }

    #[test]
    fn test_synthesis_options_default() {
        let opts = SynthesisOptions::default();
        assert!(opts.voice.is_none());
        assert!(opts.rate.is_none());
        assert!(opts.pitch.is_none());
        assert!(opts.volume.is_none());
    }

    #[test]
    fn test_voice_serialization() {
        let voice = Voice {
            id: "com.apple.voice.samantha".to_string(),
            name: "Samantha".to_string(),
            language: "en-US".to_string(),
            is_default: true,
            quality: Some(VoiceQuality::Enhanced),
            gender: Some(VoiceGender::Female),
        };
        let json = serde_json::to_string(&voice).unwrap();
        assert!(json.contains("\"id\":\"com.apple.voice.samantha\""));
        assert!(json.contains("\"isDefault\":true"));
        assert!(json.contains("\"quality\":\"enhanced\""));
        assert!(json.contains("\"gender\":\"female\""));
    }

    #[test]
    fn test_voice_quality_serialization() {
        assert_eq!(
            serde_json::to_string(&VoiceQuality::Default).unwrap(),
            "\"default\""
        );
        assert_eq!(
            serde_json::to_string(&VoiceQuality::Enhanced).unwrap(),
            "\"enhanced\""
        );
        assert_eq!(
            serde_json::to_string(&VoiceQuality::Premium).unwrap(),
            "\"premium\""
        );
    }

    #[test]
    fn test_voice_gender_serialization() {
        assert_eq!(
            serde_json::to_string(&VoiceGender::Male).unwrap(),
            "\"male\""
        );
        assert_eq!(
            serde_json::to_string(&VoiceGender::Female).unwrap(),
            "\"female\""
        );
        assert_eq!(
            serde_json::to_string(&VoiceGender::Neutral).unwrap(),
            "\"neutral\""
        );
    }

    // =========================================================================
    // Vision Types Tests
    // =========================================================================

    #[test]
    fn test_image_source_base64_serialization() {
        let source = ImageSource::Base64("SGVsbG8gd29ybGQ=".to_string());
        let json = serde_json::to_string(&source).unwrap();
        assert!(json.contains("\"base64\":\"SGVsbG8gd29ybGQ=\""));
    }

    #[test]
    fn test_image_source_file_path_serialization() {
        let source = ImageSource::FilePath("/path/to/image.png".to_string());
        let json = serde_json::to_string(&source).unwrap();
        assert!(json.contains("\"filePath\":\"/path/to/image.png\""));
    }

    #[test]
    fn test_bounding_box_normalized() {
        let bbox = BoundingBox {
            x: 0.1,
            y: 0.2,
            width: 0.5,
            height: 0.3,
        };
        assert!(bbox.x >= 0.0 && bbox.x <= 1.0);
        assert!(bbox.y >= 0.0 && bbox.y <= 1.0);
        assert!(bbox.width >= 0.0 && bbox.width <= 1.0);
        assert!(bbox.height >= 0.0 && bbox.height <= 1.0);
    }

    #[test]
    fn test_ocr_options_default() {
        let opts = OcrOptions::default();
        assert!(opts.languages.is_empty());
        assert!(opts.recognition_level.is_none());
    }

    #[test]
    fn test_recognition_level_serialization() {
        assert_eq!(
            serde_json::to_string(&RecognitionLevel::Fast).unwrap(),
            "\"fast\""
        );
        assert_eq!(
            serde_json::to_string(&RecognitionLevel::Accurate).unwrap(),
            "\"accurate\""
        );
    }

    #[test]
    fn test_text_recognition_result_serialization() {
        let result = TextRecognitionResult {
            text: "Hello World".to_string(),
            blocks: vec![TextBlock {
                text: "Hello".to_string(),
                bounding_box: BoundingBox {
                    x: 0.0,
                    y: 0.0,
                    width: 0.5,
                    height: 0.1,
                },
                lines: vec![],
                confidence: Some(0.99),
            }],
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"text\":\"Hello World\""));
        assert!(json.contains("\"blocks\""));
    }

    #[test]
    fn test_barcode_format_serialization() {
        assert_eq!(
            serde_json::to_string(&BarcodeFormat::QrCode).unwrap(),
            "\"qr_code\""
        );
        assert_eq!(
            serde_json::to_string(&BarcodeFormat::Ean13).unwrap(),
            "\"ean_13\""
        );
        assert_eq!(
            serde_json::to_string(&BarcodeFormat::Code128).unwrap(),
            "\"code_128\""
        );
    }

    #[test]
    fn test_barcode_serialization() {
        let barcode = Barcode {
            format: BarcodeFormat::QrCode,
            raw_value: "https://example.com".to_string(),
            bounding_box: BoundingBox {
                x: 0.1,
                y: 0.1,
                width: 0.3,
                height: 0.3,
            },
            parsed_data: None,
        };
        let json = serde_json::to_string(&barcode).unwrap();
        assert!(json.contains("\"format\":\"qr_code\""));
        assert!(json.contains("\"rawValue\":\"https://example.com\""));
    }

    #[test]
    fn test_face_options_default() {
        let opts = FaceOptions::default();
        assert!(!opts.detect_landmarks);
        assert!(!opts.classify_attributes);
    }

    #[test]
    fn test_face_serialization() {
        let face = Face {
            bounding_box: BoundingBox {
                x: 0.2,
                y: 0.1,
                width: 0.4,
                height: 0.5,
            },
            landmarks: Some(FaceLandmarks {
                left_eye: Some(Point { x: 0.3, y: 0.2 }),
                right_eye: Some(Point { x: 0.5, y: 0.2 }),
                nose: Some(Point { x: 0.4, y: 0.35 }),
                mouth_left: None,
                mouth_right: None,
            }),
            attributes: Some(FaceAttributes {
                smiling_probability: Some(0.8),
                left_eye_open_probability: Some(0.95),
                right_eye_open_probability: Some(0.92),
            }),
            roll_angle: Some(5.0),
            yaw_angle: Some(-3.0),
        };
        let json = serde_json::to_string(&face).unwrap();
        assert!(json.contains("\"boundingBox\""));
        assert!(json.contains("\"landmarks\""));
        assert!(json.contains("\"attributes\""));
        assert!(json.contains("\"smilingProbability\""));
    }

    #[test]
    fn test_classification_options_default() {
        let opts = ClassificationOptions::default();
        assert!(opts.max_results.is_none());
        assert!(opts.min_confidence.is_none());
    }

    #[test]
    fn test_classification_serialization() {
        let classification = Classification {
            identifier: "cat".to_string(),
            confidence: 0.89,
        };
        let json = serde_json::to_string(&classification).unwrap();
        assert!(json.contains("\"identifier\":\"cat\""));
        assert!(json.contains("\"confidence\":0.89"));
    }

    // =========================================================================
    // Text Processing Types Tests
    // =========================================================================

    #[test]
    fn test_language_identification_serialization() {
        let result = LanguageIdentification {
            language: "en".to_string(),
            confidence: 0.97,
            alternatives: vec![LanguageAlternative {
                language: "de".to_string(),
                confidence: 0.02,
            }],
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"language\":\"en\""));
        assert!(json.contains("\"alternatives\""));
    }

    #[test]
    fn test_translation_serialization() {
        let translation = Translation {
            translated_text: "Bonjour le monde".to_string(),
            source_language: "en".to_string(),
            target_language: "fr".to_string(),
        };
        let json = serde_json::to_string(&translation).unwrap();
        assert!(json.contains("\"translatedText\":\"Bonjour le monde\""));
        assert!(json.contains("\"sourceLanguage\":\"en\""));
        assert!(json.contains("\"targetLanguage\":\"fr\""));
    }

    // =========================================================================
    // LLM Types Tests
    // =========================================================================

    #[test]
    fn test_llm_availability_serialization() {
        let available = LlmAvailability {
            available: true,
            reason: None,
        };
        let json = serde_json::to_string(&available).unwrap();
        assert!(json.contains("\"available\":true"));
        assert!(!json.contains("reason"));

        let unavailable = LlmAvailability {
            available: false,
            reason: Some("Model not installed".to_string()),
        };
        let json = serde_json::to_string(&unavailable).unwrap();
        assert!(json.contains("\"available\":false"));
        assert!(json.contains("\"reason\":\"Model not installed\""));
    }

    #[test]
    fn test_llm_generate_options_serialization() {
        let opts = LlmGenerateOptions {
            prompt: "Hello".to_string(),
            system_prompt: Some("Be helpful".to_string()),
            temperature: Some(0.7),
            max_tokens: Some(512),
            top_p: None,
            top_k: None,
            seed: None,
        };
        let json = serde_json::to_string(&opts).unwrap();
        assert!(json.contains("\"prompt\":\"Hello\""));
        assert!(json.contains("\"systemPrompt\":\"Be helpful\""));
        assert!(json.contains("\"temperature\":0.7"));
        assert!(json.contains("\"maxTokens\":512"));
        assert!(!json.contains("topP"));
        assert!(!json.contains("topK"));
        assert!(!json.contains("seed"));
    }

    #[test]
    fn test_llm_generate_options_deserialization() {
        let json = r#"{"prompt":"Test","temperature":0.5,"maxTokens":100}"#;
        let opts: LlmGenerateOptions = serde_json::from_str(json).unwrap();
        assert_eq!(opts.prompt, "Test");
        assert_eq!(opts.temperature, Some(0.5));
        assert_eq!(opts.max_tokens, Some(100));
        assert!(opts.system_prompt.is_none());
    }

    #[test]
    fn test_llm_finish_reason_serialization() {
        assert_eq!(
            serde_json::to_string(&LlmFinishReason::Stop).unwrap(),
            "\"stop\""
        );
        assert_eq!(
            serde_json::to_string(&LlmFinishReason::Length).unwrap(),
            "\"length\""
        );
        assert_eq!(
            serde_json::to_string(&LlmFinishReason::Safety).unwrap(),
            "\"safety\""
        );
        assert_eq!(
            serde_json::to_string(&LlmFinishReason::Error).unwrap(),
            "\"error\""
        );
    }

    #[test]
    fn test_llm_generate_result_serialization() {
        let result = LlmGenerateResult {
            content: "Generated text".to_string(),
            model: "apple-foundation-model".to_string(),
            finish_reason: LlmFinishReason::Stop,
            usage: Some(LlmUsage {
                prompt_tokens: Some(10),
                completion_tokens: Some(20),
                total_tokens: Some(30),
            }),
        };
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"content\":\"Generated text\""));
        assert!(json.contains("\"model\":\"apple-foundation-model\""));
        assert!(json.contains("\"finishReason\":\"stop\""));
        assert!(json.contains("\"promptTokens\":10"));
    }

    #[test]
    fn test_llm_stream_event_delta_serialization() {
        let event = LlmStreamEvent::Delta {
            content: "Hello".to_string(),
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"type\":\"delta\""));
        assert!(json.contains("\"content\":\"Hello\""));
    }

    #[test]
    fn test_llm_stream_event_done_serialization() {
        let event = LlmStreamEvent::Done {
            content: "Full text".to_string(),
            finish_reason: LlmFinishReason::Stop,
            usage: None,
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"type\":\"done\""));
        assert!(json.contains("\"content\":\"Full text\""));
        assert!(json.contains("\"finishReason\":\"stop\""));
    }

    #[test]
    fn test_llm_stream_event_error_serialization() {
        let event = LlmStreamEvent::Error {
            message: "Timeout".to_string(),
        };
        let json = serde_json::to_string(&event).unwrap();
        assert!(json.contains("\"type\":\"error\""));
        assert!(json.contains("\"message\":\"Timeout\""));
    }

    #[test]
    fn test_llm_stream_event_deserialization() {
        let json = r#"{"type":"delta","content":"chunk"}"#;
        let event: LlmStreamEvent = serde_json::from_str(json).unwrap();
        match event {
            LlmStreamEvent::Delta { content } => assert_eq!(content, "chunk"),
            _ => panic!("Expected Delta event"),
        }
    }

    #[test]
    fn test_llm_session_options_default() {
        let opts = LlmSessionOptions::default();
        assert!(opts.system_prompt.is_none());
        assert!(opts.temperature.is_none());
        assert!(opts.max_tokens.is_none());
    }

    #[test]
    fn test_llm_rewrite_tone_serialization() {
        assert_eq!(
            serde_json::to_string(&LlmRewriteTone::Casual).unwrap(),
            "\"casual\""
        );
        assert_eq!(
            serde_json::to_string(&LlmRewriteTone::Formal).unwrap(),
            "\"formal\""
        );
        assert_eq!(
            serde_json::to_string(&LlmRewriteTone::Professional).unwrap(),
            "\"professional\""
        );
    }

    #[test]
    fn test_llm_model_info_serialization() {
        let info = LlmModelInfo {
            id: "apple-fm".to_string(),
            name: "Apple Foundation Model".to_string(),
            provider: "apple-foundationmodels".to_string(),
            context_window: 4096,
            on_device: true,
            capabilities: LlmModelCapabilities {
                streaming: true,
                system_prompts: true,
                temperature_control: true,
                max_tokens_control: true,
                seed_support: true,
                top_p_support: false,
                top_k_support: false,
                summarize: true,
                rewrite: true,
            },
        };
        let json = serde_json::to_string(&info).unwrap();
        assert!(json.contains("\"contextWindow\":4096"));
        assert!(json.contains("\"onDevice\":true"));
        assert!(json.contains("\"streaming\":true"));
    }

    #[test]
    fn test_llm_summarize_round_trip() {
        let opts = LlmSummarizeOptions {
            text: "Long text here".to_string(),
        };
        let json = serde_json::to_string(&opts).unwrap();
        let parsed: LlmSummarizeOptions = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.text, "Long text here");
    }

    #[test]
    fn test_llm_rewrite_round_trip() {
        let opts = LlmRewriteOptions {
            text: "hey wanna grab lunch".to_string(),
            tone: Some(LlmRewriteTone::Formal),
        };
        let json = serde_json::to_string(&opts).unwrap();
        let parsed: LlmRewriteOptions = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.text, "hey wanna grab lunch");
        assert_eq!(parsed.tone, Some(LlmRewriteTone::Formal));
    }

    #[test]
    fn test_capabilities_includes_language_model() {
        let caps = Capabilities::default();
        let json = serde_json::to_string(&caps).unwrap();
        assert!(json.contains("\"languageModel\""));
    }
}
