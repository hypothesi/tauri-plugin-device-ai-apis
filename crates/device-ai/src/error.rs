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

    // Language model errors
    #[error("Language model not available: {reason}")]
    LlmNotAvailable { reason: String },

    #[error("Language model generation failed: {message}")]
    LlmGenerationFailed { message: String },

    #[error("Language model session not found: {session_id}")]
    LlmSessionNotFound { session_id: String },

    #[error("Language model content filtered: {message}")]
    LlmContentFiltered { message: String },

    // Platform errors
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
            Error::LlmNotAvailable { .. } => "LLM_NOT_AVAILABLE",
            Error::LlmGenerationFailed { .. } => "LLM_GENERATION_FAILED",
            Error::LlmSessionNotFound { .. } => "LLM_SESSION_NOT_FOUND",
            Error::LlmContentFiltered { .. } => "LLM_CONTENT_FILTERED",
            Error::Platform(_) => "PLATFORM_ERROR",
            Error::Io(_) => "IO_ERROR",
        }
    }
}

/// Serializable error response sent to the frontend.
#[derive(Serialize)]
struct ErrorResponse {
    code: &'static str,
    message: String,
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let response = ErrorResponse {
            code: self.code(),
            message: self.to_string(),
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
                expected: "png".to_string(),
                actual: "txt".to_string(),
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
            Error::Platform("test".to_string()).code(),
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
        ];

        // Check all codes are unique
        let mut unique_codes = codes.clone();
        unique_codes.sort();
        unique_codes.dedup();
        assert_eq!(
            codes.len(),
            unique_codes.len(),
            "Error codes should be unique"
        );
    }

    #[test]
    fn test_error_display_messages() {
        assert!(Error::NoSpeechDetected.to_string().contains("No speech"));
        assert!(Error::InvalidImageData
            .to_string()
            .contains("Invalid image"));
    }

    #[test]
    fn test_feature_not_available_error() {
        let error = Error::FeatureNotAvailable {
            feature: "speechRecognition".to_string(),
        };
        assert_eq!(error.code(), "FEATURE_NOT_AVAILABLE");
        assert!(error.to_string().contains("speechRecognition"));
    }

    #[test]
    fn test_permission_denied_error() {
        let error = Error::PermissionDenied {
            permission: "microphone".to_string(),
        };
        assert_eq!(error.code(), "PERMISSION_DENIED");
        assert!(error.to_string().contains("microphone"));
    }

    #[test]
    fn test_permission_required_error() {
        let error = Error::PermissionRequired {
            permission: "camera".to_string(),
        };
        assert_eq!(error.code(), "PERMISSION_REQUIRED");
        assert!(error.to_string().contains("camera"));
    }

    #[test]
    fn test_speech_recognition_failed_error() {
        let error = Error::SpeechRecognitionFailed {
            message: "Timeout".to_string(),
        };
        assert_eq!(error.code(), "SPEECH_RECOGNITION_FAILED");
        assert!(error.to_string().contains("Timeout"));
    }

    #[test]
    fn test_speech_synthesis_failed_error() {
        let error = Error::SpeechSynthesisFailed {
            message: "No audio device".to_string(),
        };
        assert_eq!(error.code(), "SPEECH_SYNTHESIS_FAILED");
        assert!(error.to_string().contains("No audio device"));
    }

    #[test]
    fn test_language_not_supported_error() {
        let error = Error::LanguageNotSupported {
            language: "xyz-ABC".to_string(),
        };
        assert_eq!(error.code(), "LANGUAGE_NOT_SUPPORTED");
        assert!(error.to_string().contains("xyz-ABC"));
    }

    #[test]
    fn test_invalid_session_id_error() {
        let error = Error::InvalidSessionId {
            session_id: "session-123".to_string(),
        };
        assert_eq!(error.code(), "INVALID_SESSION_ID");
        assert!(error.to_string().contains("session-123"));
    }

    #[test]
    fn test_image_processing_failed_error() {
        let error = Error::ImageProcessingFailed {
            message: "corrupt data".to_string(),
        };
        assert_eq!(error.code(), "IMAGE_PROCESSING_FAILED");
        assert!(error.to_string().contains("corrupt data"));
    }

    #[test]
    fn test_invalid_image_format_error() {
        let error = Error::InvalidImageFormat {
            expected: "JPEG".to_string(),
            actual: "PDF".to_string(),
        };
        assert_eq!(error.code(), "INVALID_IMAGE_FORMAT");
        assert!(error.to_string().contains("JPEG"));
        assert!(error.to_string().contains("PDF"));
    }

    #[test]
    fn test_text_processing_failed_error() {
        let error = Error::TextProcessingFailed {
            message: "Model not loaded".to_string(),
        };
        assert_eq!(error.code(), "TEXT_PROCESSING_FAILED");
        assert!(error.to_string().contains("Model not loaded"));
    }

    #[test]
    fn test_translation_failed_error() {
        let error = Error::TranslationFailed {
            message: "Network error".to_string(),
        };
        assert_eq!(error.code(), "TRANSLATION_FAILED");
        assert!(error.to_string().contains("Network error"));
    }

    #[test]
    fn test_platform_error() {
        let error = Error::Platform("WinRT error 0x80004005".to_string());
        assert_eq!(error.code(), "PLATFORM_ERROR");
        assert!(error.to_string().contains("WinRT error"));
    }

    #[test]
    fn test_error_json_structure() {
        let error = Error::FeatureNotAvailable {
            feature: "textRecognition".to_string(),
        };
        let json = serde_json::to_string(&error).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert!(parsed.get("code").is_some());
        assert!(parsed.get("message").is_some());
        assert_eq!(parsed["code"].as_str().unwrap(), "FEATURE_NOT_AVAILABLE");
    }

    #[test]
    fn test_io_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let error: Error = io_error.into();
        assert_eq!(error.code(), "IO_ERROR");
    }

    #[test]
    fn test_llm_not_available_error() {
        let error = Error::LlmNotAvailable {
            reason: "macOS 26 required".to_string(),
        };
        assert_eq!(error.code(), "LLM_NOT_AVAILABLE");
        assert!(error.to_string().contains("macOS 26 required"));
    }

    #[test]
    fn test_llm_generation_failed_error() {
        let error = Error::LlmGenerationFailed {
            message: "context overflow".to_string(),
        };
        assert_eq!(error.code(), "LLM_GENERATION_FAILED");
        assert!(error.to_string().contains("context overflow"));
    }

    #[test]
    fn test_llm_session_not_found_error() {
        let error = Error::LlmSessionNotFound {
            session_id: "abc-123".to_string(),
        };
        assert_eq!(error.code(), "LLM_SESSION_NOT_FOUND");
        assert!(error.to_string().contains("abc-123"));
    }

    #[test]
    fn test_llm_content_filtered_error() {
        let error = Error::LlmContentFiltered {
            message: "harmful content".to_string(),
        };
        assert_eq!(error.code(), "LLM_CONTENT_FILTERED");
        assert!(error.to_string().contains("harmful content"));
    }
}
