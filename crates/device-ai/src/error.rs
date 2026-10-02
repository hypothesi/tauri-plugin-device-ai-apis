use serde::{ser::Serializer, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

/// Error types for the device AI plugin.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    // Capability errors
    #[error("Feature not available on this platform: {feature}")]
    FeatureNotAvailable { feature: String },

    #[error("Feature requires permission: {permission}")]
    PermissionRequired { permission: String },

    #[error("Permission denied: {permission}")]
    PermissionDenied { permission: String },

    // Speech errors
    #[error("Speech recognition failed: {message}")]
    SpeechRecognitionFailed { message: String },

    #[error("Speech synthesis failed: {message}")]
    SpeechSynthesisFailed { message: String },

    #[error("Language not supported: {language}")]
    LanguageNotSupported { language: String },

    #[error("No speech detected")]
    NoSpeechDetected,

    #[error("Invalid session ID: {session_id}")]
    InvalidSessionId { session_id: String },

    // Vision errors
    #[error("Image processing failed: {message}")]
    ImageProcessingFailed { message: String },

    #[error("Invalid image format: expected {expected}, got {actual}")]
    InvalidImageFormat { expected: String, actual: String },

    #[error("Invalid image data")]
    InvalidImageData,

    // Text processing errors
    #[error("Text processing failed: {message}")]
    TextProcessingFailed { message: String },

    #[error("Translation failed: {message}")]
    TranslationFailed { message: String },

    #[error("Model not installed: {model_type}")]
    ModelNotInstalled {
        model_type: String,
        details: Option<String>,
        source_language: Option<String>,
        target_language: Option<String>,
    },

    // Language model errors
    #[error("Language model not available: {reason}")]
    LlmNotAvailable { reason: String },

    #[error("Language model generation failed: {message}")]
    LlmGenerationFailed { message: String },

    #[error("Language model session not found: {session_id}")]
    LlmSessionNotFound { session_id: String },

    #[error("Language model content filtered: {message}")]
    LlmContentFiltered { message: String },

    // General input / platform errors
    #[error("Invalid argument: {message}")]
    InvalidArgument { message: String },

    #[error("Platform error: {0}")]
    Platform(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl Error {
    /// Get an error code for this error type.
    pub fn code(&self) -> &'static str {
        match self {
            Error::FeatureNotAvailable { .. } => "FEATURE_NOT_AVAILABLE",
            Error::PermissionRequired { .. } => "PERMISSION_REQUIRED",
            Error::PermissionDenied { .. } => "PERMISSION_DENIED",
            Error::SpeechRecognitionFailed { .. } => "SPEECH_RECOGNITION_FAILED",
            Error::SpeechSynthesisFailed { .. } => "SPEECH_SYNTHESIS_FAILED",
            Error::LanguageNotSupported { .. } => "LANGUAGE_NOT_SUPPORTED",
            Error::NoSpeechDetected => "NO_SPEECH_DETECTED",
            Error::InvalidSessionId { .. } => "INVALID_SESSION_ID",
            Error::ImageProcessingFailed { .. } => "IMAGE_PROCESSING_FAILED",
            Error::InvalidImageFormat { .. } => "INVALID_IMAGE_FORMAT",
            Error::InvalidImageData => "INVALID_IMAGE_DATA",
            Error::TextProcessingFailed { .. } => "TEXT_PROCESSING_FAILED",
            Error::TranslationFailed { .. } => "TRANSLATION_FAILED",
            Error::ModelNotInstalled { .. } => "MODEL_NOT_INSTALLED",
            Error::LlmNotAvailable { .. } => "LLM_NOT_AVAILABLE",
            Error::LlmGenerationFailed { .. } => "LLM_GENERATION_FAILED",
            Error::LlmSessionNotFound { .. } => "LLM_SESSION_NOT_FOUND",
            Error::LlmContentFiltered { .. } => "LLM_CONTENT_FILTERED",
            Error::InvalidArgument { .. } => "INVALID_ARGUMENTS",
            Error::Platform(_) => "PLATFORM_ERROR",
            Error::Io(_) => "IO_ERROR",
        }
    }

    /// Get programmatic structured details for this error if available.
    pub fn details(&self) -> Option<ErrorDetails> {
        match self {
            Error::FeatureNotAvailable { feature } => Some(ErrorDetails {
                feature: Some(feature.clone()),
                ..Default::default()
            }),
            Error::PermissionRequired { permission } | Error::PermissionDenied { permission } => {
                Some(ErrorDetails {
                    permission: Some(permission.clone()),
                    ..Default::default()
                })
            }
            Error::LanguageNotSupported { language } => Some(ErrorDetails {
                language: Some(language.clone()),
                ..Default::default()
            }),
            Error::ModelNotInstalled {
                model_type,
                source_language,
                target_language,
                ..
            } => Some(ErrorDetails {
                model_type: Some(model_type.clone()),
                source_language: source_language.clone(),
                target_language: target_language.clone(),
                ..Default::default()
            }),
            Error::InvalidSessionId { session_id } | Error::LlmSessionNotFound { session_id } => {
                Some(ErrorDetails {
                    session_id: Some(session_id.clone()),
                    ..Default::default()
                })
            }
            Error::InvalidImageFormat { expected, actual } => Some(ErrorDetails {
                expected: Some(expected.clone()),
                actual: Some(actual.clone()),
                ..Default::default()
            }),
            Error::LlmNotAvailable { reason } => Some(ErrorDetails {
                reason: Some(reason.clone()),
                ..Default::default()
            }),
            _ => None,
        }
    }
}

/// Structured metadata providing programmatic context for an error.
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual: Option<String>,
}

/// Serializable error response sent to the frontend.
#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: &'static str,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<ErrorDetails>,
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let response = ErrorResponse {
            code: self.code(),
            message: self.to_string(),
            details: self.details(),
        };
        response.serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_serialization() {
        let error = Error::FeatureNotAvailable {
            feature: "speechRecognition".to_string(),
        };
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("FEATURE_NOT_AVAILABLE"));
        assert!(json.contains("speechRecognition"));
        assert!(json.contains("\"details\":{\"feature\":\"speechRecognition\"}"));
    }

    #[test]
    fn test_model_not_installed_serialization() {
        let error = Error::ModelNotInstalled {
            model_type: "translation".to_string(),
            details: None,
            source_language: Some("en".to_string()),
            target_language: Some("es".to_string()),
        };
        let json = serde_json::to_string(&error).unwrap();
        assert!(json.contains("MODEL_NOT_INSTALLED"));
        assert!(json.contains("modelType\":\"translation\""));
        assert!(json.contains("sourceLanguage\":\"en\""));
        assert!(json.contains("targetLanguage\":\"es\""));
    }

    #[test]
    fn test_error_codes() {
        assert_eq!(Error::NoSpeechDetected.code(), "NO_SPEECH_DETECTED");
        assert_eq!(
            Error::PermissionDenied {
                permission: "microphone".to_string()
            }
            .code(),
            "PERMISSION_DENIED"
        );
        assert_eq!(
            Error::ModelNotInstalled {
                model_type: "translation".to_string(),
                details: None,
                source_language: None,
                target_language: None,
            }
            .code(),
            "MODEL_NOT_INSTALLED"
        );
    }

    #[test]
    fn test_all_error_codes_unique() {
        let codes = vec![
            Error::FeatureNotAvailable {
                feature: "test".to_string(),
            }
            .code(),
            Error::PermissionRequired {
                permission: "test".to_string(),
            }
            .code(),
            Error::PermissionDenied {
                permission: "test".to_string(),
            }
            .code(),
            Error::SpeechRecognitionFailed {
                message: "test".to_string(),
            }
            .code(),
            Error::SpeechSynthesisFailed {
                message: "test".to_string(),
            }
            .code(),
            Error::LanguageNotSupported {
                language: "test".to_string(),
            }
            .code(),
            Error::NoSpeechDetected.code(),
            Error::InvalidSessionId {
                session_id: "test".to_string(),
            }
            .code(),
            Error::ImageProcessingFailed {
                message: "test".to_string(),
            }
            .code(),
            Error::InvalidImageFormat {
                expected: "test".to_string(),
                actual: "test".to_string(),
            }
            .code(),
            Error::InvalidImageData.code(),
            Error::TextProcessingFailed {
                message: "test".to_string(),
            }
            .code(),
            Error::TranslationFailed {
                message: "test".to_string(),
            }
            .code(),
            Error::ModelNotInstalled {
                model_type: "test".to_string(),
                details: None,
                source_language: None,
                target_language: None,
            }
            .code(),
            Error::LlmNotAvailable {
                reason: "test".to_string(),
            }
            .code(),
            Error::LlmGenerationFailed {
                message: "test".to_string(),
            }
            .code(),
            Error::LlmSessionNotFound {
                session_id: "test".to_string(),
            }
            .code(),
            Error::LlmContentFiltered {
                message: "test".to_string(),
            }
            .code(),
            Error::InvalidArgument {
                message: "test".to_string(),
            }
            .code(),
            Error::Platform("test".to_string()).code(),
            Error::Io(std::io::Error::other("test")).code(),
        ];

        let mut unique_codes = codes.clone();
        unique_codes.sort();
        unique_codes.dedup();
        assert_eq!(codes.len(), unique_codes.len());
    }

    #[test]
    fn test_error_display_messages() {
        assert_eq!(
            Error::FeatureNotAvailable {
                feature: "speechRecognition".to_string()
            }
            .to_string(),
            "Feature not available on this platform: speechRecognition"
        );
        assert_eq!(
            Error::PermissionRequired {
                permission: "microphone".to_string()
            }
            .to_string(),
            "Feature requires permission: microphone"
        );
        assert_eq!(
            Error::PermissionDenied {
                permission: "microphone".to_string()
            }
            .to_string(),
            "Permission denied: microphone"
        );
        assert_eq!(
            Error::SpeechRecognitionFailed {
                message: "timed out".to_string()
            }
            .to_string(),
            "Speech recognition failed: timed out"
        );
        assert_eq!(
            Error::SpeechSynthesisFailed {
                message: "voice not found".to_string()
            }
            .to_string(),
            "Speech synthesis failed: voice not found"
        );
        assert_eq!(
            Error::LanguageNotSupported {
                language: "xx-YY".to_string()
            }
            .to_string(),
            "Language not supported: xx-YY"
        );
        assert_eq!(Error::NoSpeechDetected.to_string(), "No speech detected");
        assert_eq!(
            Error::InvalidSessionId {
                session_id: "abc".to_string()
            }
            .to_string(),
            "Invalid session ID: abc"
        );
        assert_eq!(
            Error::ImageProcessingFailed {
                message: "corrupt".to_string()
            }
            .to_string(),
            "Image processing failed: corrupt"
        );
        assert_eq!(
            Error::InvalidImageFormat {
                expected: "RGBA".to_string(),
                actual: "RGB".to_string()
            }
            .to_string(),
            "Invalid image format: expected RGBA, got RGB"
        );
        assert_eq!(Error::InvalidImageData.to_string(), "Invalid image data");
        assert_eq!(
            Error::TextProcessingFailed {
                message: "empty".to_string()
            }
            .to_string(),
            "Text processing failed: empty"
        );
        assert_eq!(
            Error::TranslationFailed {
                message: "network".to_string()
            }
            .to_string(),
            "Translation failed: network"
        );
        assert_eq!(
            Error::ModelNotInstalled {
                model_type: "translation".to_string(),
                details: None,
                source_language: None,
                target_language: None,
            }
            .to_string(),
            "Model not installed: translation"
        );
        assert_eq!(
            Error::LlmNotAvailable {
                reason: "device not supported".to_string()
            }
            .to_string(),
            "Language model not available: device not supported"
        );
        assert_eq!(
            Error::LlmGenerationFailed {
                message: "timeout".to_string()
            }
            .to_string(),
            "Language model generation failed: timeout"
        );
        assert_eq!(
            Error::LlmSessionNotFound {
                session_id: "s1".to_string()
            }
            .to_string(),
            "Language model session not found: s1"
        );
        assert_eq!(
            Error::LlmContentFiltered {
                message: "safety policy".to_string()
            }
            .to_string(),
            "Language model content filtered: safety policy"
        );
        assert_eq!(
            Error::Platform("test".to_string()).to_string(),
            "Platform error: test"
        );
    }

    #[test]
    fn test_error_json_structure() {
        let error = Error::SpeechRecognitionFailed {
            message: "Recognition timed out".to_string(),
        };
        let value: serde_json::Value =
            serde_json::from_str(&serde_json::to_string(&error).unwrap()).unwrap();

        assert_eq!(value["code"], "SPEECH_RECOGNITION_FAILED");
        assert_eq!(
            value["message"],
            "Speech recognition failed: Recognition timed out"
        );
    }
}
