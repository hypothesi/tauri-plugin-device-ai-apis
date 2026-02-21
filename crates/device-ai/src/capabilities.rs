//! Capability detection for platform AI features.
//!
//! This module provides runtime detection of available AI capabilities
//! on the current platform.

use crate::models::{Capabilities, FeatureStatus};

/// Get the capabilities available on the current platform.
///
/// This function queries the underlying platform APIs to determine
/// which AI features are available.
#[cfg(target_os = "macos")]
pub fn get_platform_capabilities() -> Capabilities {
    Capabilities {
        // SFSpeechRecognizer is now implemented
        speech_recognition: FeatureStatus::available_on_device(true),
        // NSSpeechSynthesizer is fully implemented
        speech_synthesis: FeatureStatus::available_on_device(false),
        // VNRecognizeTextRequest is fully implemented
        text_recognition: FeatureStatus::available_on_device(false),
        // VNDetectBarcodesRequest is fully implemented
        barcode_detection: FeatureStatus::available_on_device(false),
        // VNDetectFaceRectanglesRequest is fully implemented
        face_detection: FeatureStatus::available_on_device(false),
        // VNClassifyImageRequest is fully implemented
        image_classification: FeatureStatus::available_on_device(false),
        // NLLanguageRecognizer is fully implemented
        language_identification: FeatureStatus::available_on_device(false),
        // Translation requires special entitlements
        translation: FeatureStatus::unavailable(),
        // FoundationModels requires macOS 26+ (detected at runtime)
        language_model: FeatureStatus::unavailable(),
    }
}

#[cfg(any(target_os = "ios", target_os = "android"))]
pub fn get_platform_capabilities() -> Capabilities {
    Capabilities {
        speech_recognition: FeatureStatus::available_on_device(true),
        speech_synthesis: FeatureStatus::available_on_device(false),
        text_recognition: FeatureStatus::available_on_device(false),
        barcode_detection: FeatureStatus::available_on_device(false),
        face_detection: FeatureStatus::available_on_device(false),
        image_classification: FeatureStatus::available_on_device(false),
        language_identification: FeatureStatus::available_on_device(false),
        translation: FeatureStatus::available_on_device(false),
        language_model: FeatureStatus::unavailable(),
    }
}

#[cfg(target_os = "windows")]
pub fn get_platform_capabilities() -> Capabilities {
    Capabilities {
        speech_recognition: FeatureStatus::available_on_device(true),
        speech_synthesis: FeatureStatus::available_on_device(false),
        text_recognition: FeatureStatus::available_on_device(false),
        barcode_detection: FeatureStatus::unavailable(),
        face_detection: FeatureStatus::unavailable(),
        image_classification: FeatureStatus::unavailable(),
        language_identification: FeatureStatus::unavailable(),
        translation: FeatureStatus::unavailable(),
        // Phi Silica requires Copilot+ PC with Windows App SDK 1.7+
        language_model: FeatureStatus::unavailable(),
    }
}

#[cfg(target_os = "linux")]
pub fn get_platform_capabilities() -> Capabilities {
    // Linux has limited native AI capabilities
    Capabilities {
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

#[cfg(not(any(
    target_os = "macos",
    target_os = "ios",
    target_os = "android",
    target_os = "windows",
    target_os = "linux"
)))]
pub fn get_platform_capabilities() -> Capabilities {
    Capabilities::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capabilities_serialization() {
        let caps = get_platform_capabilities();
        let json = serde_json::to_string(&caps).unwrap();
        assert!(json.contains("speechRecognition"));
        assert!(json.contains("available"));
    }

    #[test]
    fn test_feature_status_unavailable() {
        let status = FeatureStatus::unavailable();
        assert!(!status.available);
        assert!(!status.on_device);
        assert!(!status.requires_permission);
    }

    #[test]
    fn test_feature_status_available() {
        let status = FeatureStatus::available_on_device(true);
        assert!(status.available);
        assert!(status.on_device);
        assert!(status.requires_permission);
    }
}
