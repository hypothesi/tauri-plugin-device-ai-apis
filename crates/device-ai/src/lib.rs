//! Cross-platform access to native, on-device AI APIs.
//!
//! This crate provides Rust-first access to platform-native speech recognition, speech
//! synthesis, OCR, biometric detection, and local language model APIs. Works on macOS and Windows.
//!
//! ## Quick start
//!
//! ```no_run
//! use device_ai::{DeviceAi, ImageSource, OcrOptions};
//!
//! fn main() -> device_ai::Result<()> {
//!     let ai = DeviceAi::new();
//!
//!     // Check what's available on this platform
//!     let caps = ai.capabilities();
//!     println!("speech recognition: {}", caps.speech_recognition.available);
//!     println!("OCR: {}", caps.text_recognition.available);
//!
//!     // Run OCR on an image
//!     let result = ai.vision().recognize_text(
//!         ImageSource::from_path("receipt.png"),
//!         OcrOptions::new(),
//!     )?;
//!     println!("recognized: {}", result.text);
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Feature flags
//!
//! Enable or disable individual capabilities by disabling default features:
//!
//! ```toml
//! [dependencies]
//! device-ai = { version = "0.1", default-features = false, features = ["speech", "vision"] }
//! ```
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `speech` | Speech recognition and speech synthesis |
//! | `vision` | OCR, barcode detection, face detection, image classification |
//! | `text` | Language identification |
//! | `llm` | On-device language model (macOS only) |

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
///
/// # Quick start
///
/// ```no_run
/// use device_ai::DeviceAi;
///
/// fn main() -> device_ai::Result<()> {
///     let ai = DeviceAi::new();
///     let caps = ai.capabilities();
///
///     println!("speech recognition: {}", caps.speech_recognition.available);
///     println!("OCR: {}", caps.text_recognition.available);
///     Ok(())
/// }
/// ```
#[derive(Debug, Default, Clone, Copy)]
pub struct DeviceAi;

impl DeviceAi {
    /// Create a new client for the current target platform.
    pub const fn new() -> Self {
        Self
    }

    /// Get the capabilities available on the current platform.
    ///
    /// Returns a [`Capabilities`] struct indicating which AI features are available,
    /// whether they run on-device, and if they require user permission.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use device_ai::DeviceAi;
    ///
    /// fn main() -> device_ai::Result<()> {
    ///     let ai = DeviceAi::new();
    ///     let caps = ai.capabilities();
    ///
    ///     if caps.speech_recognition.available {
    ///         println!("speech is available on-device: {}", caps.speech_recognition.on_device);
    ///         println!("requires permission: {}", caps.speech_recognition.requires_permission);
    ///     }
    ///     Ok(())
    /// }
    /// ```
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
    ///
    /// Provides speech recognition and speech synthesis capabilities.
    pub fn speech(&self) -> Speech<'_> {
        Speech(self)
    }

    /// Access vision-related APIs.
    ///
    /// Provides OCR, barcode detection, face detection, and image classification.
    pub fn vision(&self) -> Vision<'_> {
        Vision(self)
    }

    /// Access text-related APIs.
    ///
    /// Provides language identification and translation.
    pub fn text(&self) -> Text<'_> {
        Text(self)
    }

    /// Access language-model APIs.
    ///
    /// Provides on-device LLM generation, summarization, and rewriting (macOS only).
    pub fn llm(&self) -> Llm<'_> {
        Llm(self)
    }
}

/// Speech-related APIs.
///
/// Provides access to speech recognition and speech synthesis capabilities.
///
/// # Example
///
/// ```no_run
/// use device_ai::{DeviceAi, RecognitionOptions, SynthesisOptions};
///
/// fn main() -> device_ai::Result<()> {
///     let ai = DeviceAi::new();
///
///     // Get available voices for synthesis
///     let voices = ai.speech().voices()?;
///     for voice in voices.iter().take(3) {
///         println!("{} ({})", voice.name, voice.language);
///     }
///
///     // Synthesize speech (plays audio on macOS)
///     ai.speech().speak("Hello, world!", SynthesisOptions::new())?;
///     Ok(())
/// }
/// ```
pub struct Speech<'a>(&'a DeviceAi);

impl Speech<'_> {
    /// Perform one-shot speech recognition.
    ///
    /// Converts audio to text. Requires microphone permission on macOS.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use device_ai::{DeviceAi, RecognitionOptions, AudioSource};
    ///
    /// fn main() -> device_ai::Result<()> {
    ///     let ai = DeviceAi::new();
    ///
    ///     // Recognize from a file
    ///     let result = ai.speech().recognize(
    ///         RecognitionOptions::new().with_audio_source(AudioSource::from_path("audio.wav")),
    ///     )?;
    ///     println!("recognized: {}", result.text);
    ///     Ok(())
    /// }
    /// ```
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
    ///
    /// Streaming recognition is not yet implemented. Returns an error.
    pub fn start_recognition(&self, _options: RecognitionOptions) -> Result<SpeechSessionId> {
        let _ = self.0;

        Err(Error::SpeechRecognitionFailed {
            message: "Streaming speech recognition is not yet implemented. Use recognize() for one-shot recognition instead.".to_string(),
        })
    }

    /// Stop streaming speech recognition.
    ///
    /// Streaming recognition is not yet implemented. Returns an error.
    pub fn stop_recognition(&self, _session_id: SpeechSessionId) -> Result<RecognitionResult> {
        let _ = self.0;

        Err(Error::SpeechRecognitionFailed {
            message: "Streaming speech recognition is not yet implemented. Use recognize() for one-shot recognition instead.".to_string(),
        })
    }

    /// Synthesize and play text as speech.
    ///
    /// Converts text to audio and plays it. On macOS, uses the system audio output.
    /// On Windows, synthesizes but does not play audio yet.
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
    ///
    /// Returns a list of voices installed on the system.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use device_ai::DeviceAi;
    ///
    /// fn main() -> device_ai::Result<()> {
    ///     let ai = DeviceAi::new();
    ///     let voices = ai.speech().voices()?;
    ///
    ///     // Find English voices
    ///     let en_voices: Vec<_> = voices.iter()
    ///         .filter(|v| v.language.starts_with("en"))
    ///         .collect();
    ///     println!("{} English voices available", en_voices.len());
    ///     Ok(())
    /// }
    /// ```
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
///
/// Provides access to OCR, barcode detection, face detection, and image classification.
///
/// # Example
///
/// ```no_run
/// use device_ai::{DeviceAi, ImageSource, OcrOptions};
///
/// fn main() -> device_ai::Result<()> {
///     let ai = DeviceAi::new();
///
///     let result = ai.vision().recognize_text(
///         ImageSource::from_path("receipt.png"),
///         OcrOptions::new(),
///     )?;
///     println!("recognized: {}", result.text);
///     Ok(())
/// }
/// ```
pub struct Vision<'a>(&'a DeviceAi);

impl Vision<'_> {
    /// Recognize text in an image (OCR).
    ///
    /// Extracts text from an image. Supports multiple languages.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use device_ai::{DeviceAi, ImageSource, OcrOptions};
    ///
    /// fn main() -> device_ai::Result<()> {
    ///     let ai = DeviceAi::new();
    ///
    ///     let result = ai.vision().recognize_text(
    ///         ImageSource::from_path("document.png"),
    ///         OcrOptions::new().with_language("en-US"),
    ///     )?;
    ///     println!("found {} lines", result.blocks.len());
    ///     Ok(())
    /// }
    /// ```
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
    ///
    /// Detects and decodes barcodes and QR codes. Currently only available on macOS.
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
    ///
    /// Detects faces with optional landmarks and attributes. Currently only available on macOS.
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
    ///
    /// Classifies an image into categories. Currently only available on macOS.
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
///
/// Provides access to language identification and translation.
///
/// # Example
///
/// ```no_run
/// use device_ai::DeviceAi;
///
/// fn main() -> device_ai::Result<()> {
///     let ai = DeviceAi::new();
///
///     let result = ai.text().identify_language("Hello world")?;
///     println!("detected: {} (confidence: {})", result.language, result.confidence);
///     Ok(())
/// }
/// ```
pub struct Text<'a>(&'a DeviceAi);

impl Text<'_> {
    /// Identify the language of text.
    ///
    /// Detects the most likely language for the given text. Currently only available on macOS.
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
    ///
    /// Translation is not yet implemented. Returns [`Error::FeatureNotAvailable`].
    pub fn translate(&self, text: &str, from: &str, to: &str) -> Result<Translation> {
        let _ = self.0;
        let _ = (text, from, to);

        Err(feature_not_available("translation"))
    }
}

/// Language-model APIs.
///
/// Provides access to on-device language model generation, summarization, and rewriting.
/// Currently only available on macOS with Apple Intelligence.
///
/// # Example
///
/// ```no_run
/// use device_ai::{DeviceAi, LlmGenerateOptions};
///
/// fn main() -> device_ai::Result<()> {
///     let ai = DeviceAi::new();
///
///     // Check availability
///     let availability = ai.llm().check_availability()?;
///     if !availability.available {
///         println!("LLM not available: {}", availability.reason.as_deref().unwrap_or("unknown"));
///         return Ok(());
///     }
///
///     // Generate text
///     let result = ai.llm().generate(
///         LlmGenerateOptions::new("What is Rust?").max_tokens(100),
///     )?;
///     println!("response: {}", result.content);
///     Ok(())
/// }
/// ```
pub struct Llm<'a>(&'a DeviceAi);

impl Llm<'_> {
    /// Check whether the on-device language model is available.
    ///
    /// Returns availability status and reason if unavailable. On macOS, requires
    /// Apple Intelligence to be enabled.
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
    ///
    /// Returns model details including context window size and capabilities.
    /// Currently only available on macOS.
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
    ///
    /// Performs single-shot text generation. For streaming output, use
    /// [`generate_stream`](Self::generate_stream) instead.
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
    ///
    /// Calls the provided callback with incremental text chunks. Use for real-time output.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use device_ai::{DeviceAi, LlmGenerateOptions, LlmStreamEvent};
    ///
    /// fn main() -> device_ai::Result<()> {
    ///     let ai = DeviceAi::new();
    ///
    ///     ai.llm().generate_stream(
    ///         LlmGenerateOptions::new("Count to 3"),
    ///         |event| {
    ///             match event {
    ///                 LlmStreamEvent::Delta { content } => print!("{}", content),
    ///                 LlmStreamEvent::Done { .. } => println!(),
    ///                 LlmStreamEvent::Error { message } => eprintln!("error: {}", message),
    ///             }
    ///             Ok(())
    ///         },
    ///     )?;
    ///     Ok(())
    /// }
    /// ```
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
    ///
    /// Sessions maintain conversation history for contextual responses.
    /// Currently only available on macOS.
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
    ///
    /// Send a message to an existing session. Use [`create_session`](Self::create_session)
    /// to create a new session first.
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
    ///
    /// Streams a response from an existing session. Use [`create_session`](Self::create_session)
    /// to create a new session first.
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
    ///
    /// Ends a session and releases its resources. Currently only available on macOS.
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
    ///
    /// Generates a summary of the provided text. Currently only available on macOS.
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
    ///
    /// Rewrites text with an optional tone. Currently only available on macOS.
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
