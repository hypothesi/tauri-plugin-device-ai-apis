//! Tauri-agnostic access to native, on-device AI APIs.
//!
//! This crate exposes Rust-first access to platform-native speech, vision, text, and
//! language-model APIs without depending on Tauri. The Tauri plugin in the repository
//! consumes this crate as an adapter layer for JS/IPC use cases.

mod capabilities;
mod error;
mod models;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "windows")]
mod windows;

pub use capabilities::get_platform_capabilities;
pub use error::{Error, Result};
pub use models::*;

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
const LANGUAGE_MODEL_UNAVAILABLE_REASON: &str = "Language model not available on this platform";

/// Entry point for the device AI library.
#[derive(Debug, Default, Clone, Copy)]
pub struct DeviceAi;

impl DeviceAi {
    /// Create a new client for the current target platform.
    pub const fn new() -> Self {
        Self
    }

    /// Get the capabilities available on the current platform.
    pub fn capabilities(&self) -> Capabilities {
        #[cfg(target_os = "windows")]
        {
            windows::get_capabilities()
        }
        #[cfg(not(target_os = "windows"))]
        {
            capabilities::get_platform_capabilities()
        }
    }

    /// Access speech-related APIs.
    pub fn speech(&self) -> Speech<'_> {
        Speech(self)
    }

    /// Access vision-related APIs.
    pub fn vision(&self) -> Vision<'_> {
        Vision(self)
    }

    /// Access text-related APIs.
    pub fn text(&self) -> Text<'_> {
        Text(self)
    }

    /// Access language-model APIs.
    pub fn llm(&self) -> Llm<'_> {
        Llm(self)
    }
}

/// Speech-related APIs.
pub struct Speech<'a>(&'a DeviceAi);

impl Speech<'_> {
    /// Perform one-shot speech recognition.
    pub fn recognize(&self, options: RecognitionOptions) -> Result<RecognitionResult> {
        let _ = self.0;

        #[cfg(target_os = "windows")]
        {
            windows::speech_recognize(options)
        }
        #[cfg(target_os = "macos")]
        {
            macos::speech_recognize(options)
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            let _ = options;

            Err(feature_not_available("speechRecognition"))
        }
    }

    /// Start streaming speech recognition.
    pub fn start_recognition(&self, _options: RecognitionOptions) -> Result<SpeechSessionId> {
        let _ = self.0;

        Err(Error::SpeechRecognitionFailed {
            message: "Streaming speech recognition is not yet implemented. Use recognize() for one-shot recognition instead.".to_string(),
        })
    }

    /// Stop streaming speech recognition.
    pub fn stop_recognition(&self, _session_id: SpeechSessionId) -> Result<RecognitionResult> {
        let _ = self.0;

        Err(Error::SpeechRecognitionFailed {
            message: "Streaming speech recognition is not yet implemented. Use recognize() for one-shot recognition instead.".to_string(),
        })
    }

    /// Synthesize and play text.
    pub fn speak(&self, text: &str, options: SynthesisOptions) -> Result<()> {
        let _ = self.0;

        #[cfg(target_os = "windows")]
        {
            windows::speech_synthesize(text, options)
        }
        #[cfg(target_os = "macos")]
        {
            macos::speech_synthesize(text, options)
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            let _ = (text, options);

            Err(feature_not_available("speechSynthesis"))
        }
    }

    /// Get available voices for speech synthesis.
    pub fn voices(&self) -> Result<Vec<Voice>> {
        let _ = self.0;

        #[cfg(target_os = "windows")]
        {
            windows::speech_get_voices()
        }
        #[cfg(target_os = "macos")]
        {
            macos::speech_get_voices()
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            Err(feature_not_available("speechSynthesis"))
        }
    }
}

/// Vision-related APIs.
pub struct Vision<'a>(&'a DeviceAi);

impl Vision<'_> {
    /// Recognize text in an image.
    pub fn recognize_text(
        &self,
        image: ImageSource,
        options: OcrOptions,
    ) -> Result<TextRecognitionResult> {
        let _ = self.0;

        #[cfg(target_os = "windows")]
        {
            windows::vision_recognize_text(image, options)
        }
        #[cfg(target_os = "macos")]
        {
            macos::vision_recognize_text(image, options)
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos")))]
        {
            let _ = (image, options);

            Err(feature_not_available("textRecognition"))
        }
    }

    /// Detect barcodes in an image.
    pub fn detect_barcodes(
        &self,
        image: ImageSource,
        options: BarcodeOptions,
    ) -> Result<Vec<Barcode>> {
        let _ = self.0;

        #[cfg(target_os = "macos")]
        {
            macos::vision_detect_barcodes(image, options)
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = (image, options);

            Err(feature_not_available("barcodeDetection"))
        }
    }

    /// Detect faces in an image.
    pub fn detect_faces(&self, image: ImageSource, options: FaceOptions) -> Result<Vec<Face>> {
        let _ = self.0;

        #[cfg(target_os = "macos")]
        {
            macos::vision_detect_faces(image, options)
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = (image, options);

            Err(feature_not_available("faceDetection"))
        }
    }

    /// Classify an image.
    pub fn classify_image(
        &self,
        image: ImageSource,
        options: ClassificationOptions,
    ) -> Result<Vec<Classification>> {
        let _ = self.0;

        #[cfg(target_os = "macos")]
        {
            macos::vision_classify_image(image, options)
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = (image, options);

            Err(feature_not_available("imageClassification"))
        }
    }
}

/// Text-related APIs.
pub struct Text<'a>(&'a DeviceAi);

impl Text<'_> {
    /// Identify the language of text.
    pub fn identify_language(&self, text: &str) -> Result<LanguageIdentification> {
        let _ = self.0;

        #[cfg(target_os = "macos")]
        {
            macos::text_identify_language(text)
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = text;

            Err(feature_not_available("languageIdentification"))
        }
    }

    /// Translate text between languages.
    pub fn translate(&self, text: &str, from: &str, to: &str) -> Result<Translation> {
        let _ = self.0;
        let _ = (text, from, to);

        Err(feature_not_available("translation"))
    }
}

/// Language-model APIs.
pub struct Llm<'a>(&'a DeviceAi);

impl Llm<'_> {
    /// Check whether the on-device language model is available.
    pub fn check_availability(&self) -> Result<LlmAvailability> {
        let _ = self.0;

        #[cfg(target_os = "macos")]
        {
            macos::llm_check_availability()
        }
        #[cfg(target_os = "windows")]
        {
            windows::llm_check_availability()
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            Ok(LlmAvailability {
                available: false,
                reason: Some(LANGUAGE_MODEL_UNAVAILABLE_REASON.to_string()),
            })
        }
    }

    /// Get information about the on-device language model.
    pub fn model_info(&self) -> Result<LlmModelInfo> {
        let _ = self.0;

        #[cfg(target_os = "macos")]
        {
            macos::llm_get_model_info()
        }
        #[cfg(target_os = "windows")]
        {
            windows::llm_get_model_info()
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            Err(llm_not_available())
        }
    }

    /// Generate text using the on-device language model.
    pub fn generate(&self, options: LlmGenerateOptions) -> Result<LlmGenerateResult> {
        let _ = self.0;

        #[cfg(target_os = "macos")]
        {
            macos::llm_generate(options)
        }
        #[cfg(target_os = "windows")]
        {
            windows::llm_generate(options)
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = options;

            Err(llm_not_available())
        }
    }

    /// Stream generated text from the on-device language model.
    pub fn generate_stream<F>(&self, options: LlmGenerateOptions, on_event: F) -> Result<()>
    where
        F: FnMut(LlmStreamEvent) -> Result<()> + 'static,
    {
        let _ = self.0;

        #[cfg(target_os = "macos")]
        {
            macos::llm_generate_stream(options, on_event)
        }
        #[cfg(target_os = "windows")]
        {
            windows::llm_generate_stream(options, on_event)
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = (options, on_event);

            Err(llm_not_available())
        }
    }

    /// Create a multi-turn language-model session.
    pub fn create_session(&self, options: LlmSessionOptions) -> Result<LlmSessionId> {
        let _ = self.0;

        #[cfg(target_os = "macos")]
        {
            macos::llm_create_session(options)
        }
        #[cfg(target_os = "windows")]
        {
            windows::llm_create_session(options)
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = options;

            Err(llm_not_available())
        }
    }

    /// Send a message to a language-model session.
    pub fn session_send(
        &self,
        session_id: LlmSessionId,
        message: String,
    ) -> Result<LlmGenerateResult> {
        let _ = self.0;

        #[cfg(target_os = "macos")]
        {
            macos::llm_session_send(session_id, message)
        }
        #[cfg(target_os = "windows")]
        {
            windows::llm_session_send(session_id.as_str(), message.as_str())
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = (session_id, message);

            Err(llm_not_available())
        }
    }

    /// Stream a response from a language-model session.
    pub fn session_send_stream<F>(
        &self,
        session_id: LlmSessionId,
        message: String,
        on_event: F,
    ) -> Result<()>
    where
        F: FnMut(LlmStreamEvent) -> Result<()> + 'static,
    {
        let _ = self.0;

        #[cfg(target_os = "macos")]
        {
            macos::llm_session_send_stream(session_id, message, on_event)
        }
        #[cfg(target_os = "windows")]
        {
            windows::llm_session_send_stream(session_id.as_str(), message.as_str(), on_event)
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = (session_id, message, on_event);

            Err(llm_not_available())
        }
    }

    /// Destroy a language-model session.
    pub fn destroy_session(&self, session_id: LlmSessionId) -> Result<()> {
        let _ = self.0;

        #[cfg(target_os = "macos")]
        {
            macos::llm_destroy_session(session_id)
        }
        #[cfg(target_os = "windows")]
        {
            windows::llm_destroy_session(session_id.as_str())
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = session_id;

            Err(llm_not_available())
        }
    }

    /// Summarize text using the on-device language model.
    pub fn summarize(&self, options: LlmSummarizeOptions) -> Result<LlmSummarizeResult> {
        let _ = self.0;

        #[cfg(target_os = "macos")]
        {
            macos::llm_summarize(options)
        }
        #[cfg(target_os = "windows")]
        {
            windows::llm_summarize(options)
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = options;

            Err(llm_not_available())
        }
    }

    /// Rewrite text using the on-device language model.
    pub fn rewrite(&self, options: LlmRewriteOptions) -> Result<LlmRewriteResult> {
        let _ = self.0;

        #[cfg(target_os = "macos")]
        {
            macos::llm_rewrite(options)
        }
        #[cfg(target_os = "windows")]
        {
            windows::llm_rewrite(options)
        }
        #[cfg(not(any(target_os = "macos", target_os = "windows")))]
        {
            let _ = options;

            Err(llm_not_available())
        }
    }
}

fn feature_not_available(feature: &str) -> Error {
    Error::FeatureNotAvailable {
        feature: feature.to_string(),
    }
}

#[cfg(not(any(target_os = "macos", target_os = "windows")))]
fn llm_not_available() -> Error {
    Error::LlmNotAvailable {
        reason: LANGUAGE_MODEL_UNAVAILABLE_REASON.to_string(),
    }
}
