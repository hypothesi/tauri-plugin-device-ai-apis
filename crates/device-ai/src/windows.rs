//! Windows-specific implementations using WinRT APIs.
//!
//! This module provides Windows implementations for:
//! - Speech Recognition via Windows.Media.SpeechRecognition
//! - Speech Synthesis via Windows.Media.SpeechSynthesis
//! - OCR via Windows.Media.Ocr

use crate::models::*;
use crate::Error;

use windows::{
    core::HSTRING,
    Foundation::IAsyncOperation,
    Globalization::Language,
    Graphics::Imaging::{BitmapDecoder, SoftwareBitmap},
    Media::{
        Ocr::OcrEngine, SpeechRecognition::SpeechRecognizer, SpeechSynthesis::SpeechSynthesizer,
    },
    Storage::Streams::{DataWriter, InMemoryRandomAccessStream},
};

/// Block on a Windows async operation
fn block_on<T: windows::core::RuntimeType>(op: IAsyncOperation<T>) -> windows::core::Result<T> {
    // Windows async operations can be awaited synchronously in this context
    // by using the get() method which blocks until completion
    loop {
        match op.Status() {
            Ok(status) => match status {
                windows::Foundation::AsyncStatus::Completed => return op.GetResults(),
                windows::Foundation::AsyncStatus::Error => {
                    return Err(op
                        .ErrorCode()
                        .unwrap_or(windows::core::Error::from(windows::core::HRESULT(-1))))
                }
                windows::Foundation::AsyncStatus::Canceled => {
                    return Err(windows::core::Error::from(windows::core::HRESULT(-1)))
                }
                _ => {
                    std::thread::sleep(std::time::Duration::from_millis(10));
                }
            },
            Err(e) => return Err(e),
        }
    }
}

// =============================================================================
// Speech Recognition
// =============================================================================

/// Perform speech recognition using Windows.Media.SpeechRecognition
pub fn speech_recognize(options: RecognitionOptions) -> crate::Result<RecognitionResult> {
    let recognizer = if let Some(lang) = &options.language {
        let language = Language::CreateLanguage(&HSTRING::from(lang)).map_err(|e| {
            Error::SpeechRecognitionFailed {
                message: format!("Failed to create language: {}", e),
            }
        })?;
        SpeechRecognizer::CreateWithLanguage(&language)
    } else {
        SpeechRecognizer::new()
    }
    .map_err(|e| Error::SpeechRecognitionFailed {
        message: format!("Failed to create speech recognizer: {}", e),
    })?;

    // Configure timeouts
    if let Ok(timeouts) = recognizer.Timeouts() {
        let _ = timeouts.SetInitialSilenceTimeout(std::time::Duration::from_secs(5).into());
        let _ = timeouts.SetEndSilenceTimeout(std::time::Duration::from_secs(2).into());
    }

    // Compile constraints (use default dictation)
    let compile_op =
        recognizer
            .CompileConstraintsAsync()
            .map_err(|e| Error::SpeechRecognitionFailed {
                message: format!("Failed to compile constraints: {}", e),
            })?;

    let _compile_result = block_on(compile_op).map_err(|e| Error::SpeechRecognitionFailed {
        message: format!("Failed to compile constraints: {}", e),
    })?;

    // Start recognition
    let recognize_op = recognizer
        .RecognizeAsync()
        .map_err(|e| Error::SpeechRecognitionFailed {
            message: format!("Failed to start recognition: {}", e),
        })?;

    let result = block_on(recognize_op).map_err(|e| Error::SpeechRecognitionFailed {
        message: format!("Recognition failed: {}", e),
    })?;

    let status = result
        .Status()
        .map_err(|e| Error::SpeechRecognitionFailed {
            message: format!("Failed to get result status: {}", e),
        })?;

    if status != windows::Media::SpeechRecognition::SpeechRecognitionResultStatus::Success {
        return Err(Error::SpeechRecognitionFailed {
            message: format!("Recognition status: {:?}", status),
        });
    }

    let text = result.Text().map_err(|e| Error::SpeechRecognitionFailed {
        message: format!("Failed to get text: {}", e),
    })?;

    let confidence = result.RawConfidence().unwrap_or(0.9);

    // Get alternatives if available
    let mut alternatives = Vec::new();
    if let Ok(alternates) = result.GetAlternates(5) {
        for i in 0..alternates.Size().unwrap_or(0) {
            if let Ok(alt) = alternates.GetAt(i) {
                if let (Ok(alt_text), Ok(alt_conf)) = (alt.Text(), alt.RawConfidence()) {
                    alternatives.push(RecognitionAlternative {
                        text: alt_text.to_string(),
                        confidence: alt_conf as f32,
                    });
                }
            }
        }
    }

    Ok(RecognitionResult {
        text: text.to_string(),
        confidence: confidence as f32,
        is_final: true,
        alternatives,
    })
}

// =============================================================================
// Speech Synthesis
// =============================================================================

/// Synthesize speech using Windows.Media.SpeechSynthesis
pub fn speech_synthesize(text: &str, options: SynthesisOptions) -> crate::Result<()> {
    let synthesizer = SpeechSynthesizer::new().map_err(|e| Error::SpeechSynthesisFailed {
        message: format!("Failed to create synthesizer: {}", e),
    })?;

    // Set voice if specified
    if let Some(voice_id) = &options.voice {
        if let Ok(all_voices) = SpeechSynthesizer::AllVoices() {
            for i in 0..all_voices.Size().unwrap_or(0) {
                if let Ok(voice) = all_voices.GetAt(i) {
                    if let Ok(id) = voice.Id() {
                        if id.to_string() == *voice_id {
                            let _ = synthesizer.SetVoice(&voice);
                            break;
                        }
                    }
                }
            }
        }
    }

    // Configure options
    if let Ok(synth_options) = synthesizer.Options() {
        if let Some(rate) = options.rate {
            // Windows rate is 0.5 - 6.0, default 1.0
            let _ = synth_options.SetSpeakingRate(rate as f64);
        }
        if let Some(pitch) = options.pitch {
            // Windows pitch is 0.5 - 2.0, default 1.0
            let _ = synth_options.SetAudioPitch(pitch as f64);
        }
        if let Some(volume) = options.volume {
            // Windows volume is 0.0 - 1.0
            let _ = synth_options.SetAudioVolume(volume as f64);
        }
    }

    // Synthesize to stream
    let synth_op = synthesizer
        .SynthesizeTextToStreamAsync(&HSTRING::from(text))
        .map_err(|e| Error::SpeechSynthesisFailed {
            message: format!("Failed to start synthesis: {}", e),
        })?;

    let _stream = block_on(synth_op).map_err(|e| Error::SpeechSynthesisFailed {
        message: format!("Synthesis failed: {}", e),
    })?;

    // Note: In a real implementation, you would play this stream through
    // Windows.Media.Playback.MediaPlayer or similar. For now, we just
    // return success after synthesis completes.
    // The actual playback would require additional async handling.

    Ok(())
}

/// Get available voices for speech synthesis
pub fn speech_get_voices() -> crate::Result<Vec<Voice>> {
    let all_voices = SpeechSynthesizer::AllVoices().map_err(|e| Error::SpeechSynthesisFailed {
        message: format!("Failed to get voices: {}", e),
    })?;

    let mut voices = Vec::new();
    let default_voice = SpeechSynthesizer::DefaultVoice().ok();

    for i in 0..all_voices.Size().unwrap_or(0) {
        if let Ok(voice) = all_voices.GetAt(i) {
            let id = voice.Id().map(|s| s.to_string()).unwrap_or_default();
            let name = voice
                .DisplayName()
                .map(|s| s.to_string())
                .unwrap_or_default();
            let language = voice.Language().map(|s| s.to_string()).unwrap_or_default();

            let is_default = default_voice
                .as_ref()
                .map(|dv| dv.Id().map(|s| s.to_string()).unwrap_or_default() == id)
                .unwrap_or(false);

            let gender = voice.Gender().ok().map(|g| match g {
                windows::Media::SpeechSynthesis::VoiceGender::Male => VoiceGender::Male,
                windows::Media::SpeechSynthesis::VoiceGender::Female => VoiceGender::Female,
                _ => VoiceGender::Neutral,
            });

            voices.push(Voice {
                id,
                name,
                language,
                is_default,
                quality: Some(VoiceQuality::Default),
                gender,
            });
        }
    }

    Ok(voices)
}

// =============================================================================
// OCR
// =============================================================================

/// Recognize text in an image using Windows.Media.Ocr
pub fn vision_recognize_text(
    image: ImageSource,
    options: OcrOptions,
) -> crate::Result<TextRecognitionResult> {
    // Create OCR engine with specified language or default
    let engine = if let Some(lang) = options.languages.first() {
        let language = Language::CreateLanguage(&HSTRING::from(lang.as_str())).map_err(|e| {
            Error::TextProcessingFailed {
                message: format!("Failed to create language: {}", e),
            }
        })?;

        if OcrEngine::IsLanguageSupported(&language).unwrap_or(false) {
            OcrEngine::TryCreateFromLanguage(&language).map_err(|e| {
                Error::TextProcessingFailed {
                    message: format!("Failed to create OCR engine: {}", e),
                }
            })?
        } else {
            OcrEngine::TryCreateFromUserProfileLanguages().map_err(|e| {
                Error::TextProcessingFailed {
                    message: format!("Failed to create OCR engine: {}", e),
                }
            })?
        }
    } else {
        OcrEngine::TryCreateFromUserProfileLanguages().map_err(|e| Error::TextProcessingFailed {
            message: format!("Failed to create OCR engine: {}", e),
        })?
    };

    // Load image into SoftwareBitmap
    let bitmap = load_software_bitmap(image)?;
    let image_width = bitmap.PixelWidth().unwrap_or(1) as f32;
    let image_height = bitmap.PixelHeight().unwrap_or(1) as f32;

    // Perform OCR
    let recognize_op = engine
        .RecognizeAsync(&bitmap)
        .map_err(|e| Error::TextProcessingFailed {
            message: format!("Failed to start OCR: {}", e),
        })?;

    let result = block_on(recognize_op).map_err(|e| Error::TextProcessingFailed {
        message: format!("OCR failed: {}", e),
    })?;

    let full_text = result.Text().map(|s| s.to_string()).unwrap_or_default();

    let mut blocks = Vec::new();
    if let Ok(lines) = result.Lines() {
        for i in 0..lines.Size().unwrap_or(0) {
            if let Ok(line) = lines.GetAt(i) {
                let line_text = line.Text().map(|s| s.to_string()).unwrap_or_default();

                let mut words_in_line = Vec::new();
                if let Ok(words) = line.Words() {
                    for j in 0..words.Size().unwrap_or(0) {
                        if let Ok(word) = words.GetAt(j) {
                            let word_text = word.Text().map(|s| s.to_string()).unwrap_or_default();
                            if let Ok(rect) = word.BoundingRect() {
                                words_in_line.push(TextLine {
                                    text: word_text,
                                    bounding_box: BoundingBox {
                                        x: rect.X as f32 / image_width,
                                        y: rect.Y as f32 / image_height,
                                        width: rect.Width as f32 / image_width,
                                        height: rect.Height as f32 / image_height,
                                    },
                                });
                            }
                        }
                    }
                }

                // Get bounding box from first and last word
                let bbox = if !words_in_line.is_empty() {
                    let first = &words_in_line[0].bounding_box;
                    let last = &words_in_line[words_in_line.len() - 1].bounding_box;
                    BoundingBox {
                        x: first.x,
                        y: first.y,
                        width: (last.x + last.width) - first.x,
                        height: first.height.max(last.height),
                    }
                } else {
                    BoundingBox {
                        x: 0.0,
                        y: 0.0,
                        width: 0.0,
                        height: 0.0,
                    }
                };

                blocks.push(TextBlock {
                    text: line_text,
                    bounding_box: bbox,
                    lines: words_in_line,
                    confidence: None,
                });
            }
        }
    }

    Ok(TextRecognitionResult {
        text: full_text,
        blocks,
    })
}

/// Load an image source into a SoftwareBitmap for Windows APIs
fn load_software_bitmap(image: ImageSource) -> crate::Result<SoftwareBitmap> {
    use base64::Engine;

    let bytes = match image {
        ImageSource::Base64(b64) => base64::engine::general_purpose::STANDARD
            .decode(&b64)
            .map_err(|e| Error::ImageProcessingFailed {
                message: format!("Invalid base64 image data: {}", e),
            })?,
        ImageSource::FilePath(path) => {
            std::fs::read(&path).map_err(|e| Error::ImageProcessingFailed {
                message: format!("Failed to read image file: {}", e),
            })?
        }
        ImageSource::Bytes(bytes) => bytes,
    };

    // Create an InMemoryRandomAccessStream and write the bytes
    let stream = InMemoryRandomAccessStream::new().map_err(|e| Error::ImageProcessingFailed {
        message: format!("Failed to create stream: {}", e),
    })?;

    let writer =
        DataWriter::CreateDataWriter(&stream).map_err(|e| Error::ImageProcessingFailed {
            message: format!("Failed to create writer: {}", e),
        })?;

    writer
        .WriteBytes(&bytes)
        .map_err(|e| Error::ImageProcessingFailed {
            message: format!("Failed to write bytes: {}", e),
        })?;

    let store_op = writer
        .StoreAsync()
        .map_err(|e| Error::ImageProcessingFailed {
            message: format!("Failed to store: {}", e),
        })?;

    block_on(store_op).map_err(|e| Error::ImageProcessingFailed {
        message: format!("Failed to store: {}", e),
    })?;

    // Seek to beginning
    stream.Seek(0).map_err(|e| Error::ImageProcessingFailed {
        message: format!("Failed to seek: {}", e),
    })?;

    // Create BitmapDecoder
    let decoder_op =
        BitmapDecoder::CreateAsync(&stream).map_err(|e| Error::ImageProcessingFailed {
            message: format!("Failed to create decoder: {}", e),
        })?;

    let decoder = block_on(decoder_op).map_err(|e| Error::ImageProcessingFailed {
        message: format!("Failed to decode image: {}", e),
    })?;

    // Get SoftwareBitmap
    let bitmap_op = decoder
        .GetSoftwareBitmapAsync()
        .map_err(|e| Error::ImageProcessingFailed {
            message: format!("Failed to get bitmap: {}", e),
        })?;

    let bitmap = block_on(bitmap_op).map_err(|e| Error::ImageProcessingFailed {
        message: format!("Failed to get bitmap: {}", e),
    })?;

    Ok(bitmap)
}

// =============================================================================
// Capabilities
// =============================================================================

/// Check Windows-specific capabilities
pub fn get_capabilities() -> Capabilities {
    let speech_recognition_available = SpeechRecognizer::new().is_ok();
    let speech_synthesis_available = SpeechSynthesizer::new().is_ok();
    let ocr_available = OcrEngine::TryCreateFromUserProfileLanguages().is_ok();

    // Get supported OCR languages
    let ocr_languages: Vec<String> = OcrEngine::AvailableRecognizerLanguages()
        .map(|langs| {
            let mut result = Vec::new();
            for i in 0..langs.Size().unwrap_or(0) {
                if let Ok(lang) = langs.GetAt(i) {
                    if let Ok(tag) = lang.LanguageTag() {
                        result.push(tag.to_string());
                    }
                }
            }
            result
        })
        .unwrap_or_default();

    Capabilities {
        speech_recognition: FeatureStatus {
            available: speech_recognition_available,
            on_device: true,
            requires_permission: false,
            supported_languages: None,
        },
        speech_synthesis: FeatureStatus {
            available: speech_synthesis_available,
            on_device: true,
            requires_permission: false,
            supported_languages: None,
        },
        text_recognition: FeatureStatus {
            available: ocr_available,
            on_device: true,
            requires_permission: false,
            supported_languages: Some(ocr_languages),
        },
        barcode_detection: FeatureStatus {
            available: false, // Windows doesn't have built-in barcode detection
            on_device: false,
            requires_permission: false,
            supported_languages: None,
        },
        face_detection: FeatureStatus {
            available: false, // Would require Windows.Media.FaceAnalysis
            on_device: false,
            requires_permission: false,
            supported_languages: None,
        },
        image_classification: FeatureStatus {
            available: false, // Would require Windows ML
            on_device: false,
            requires_permission: false,
            supported_languages: None,
        },
        language_identification: FeatureStatus {
            available: false,
            on_device: false,
            requires_permission: false,
            supported_languages: None,
        },
        translation: FeatureStatus {
            available: false,
            on_device: false,
            requires_permission: false,
            supported_languages: None,
        },
        language_model: FeatureStatus::unavailable(),
    }
}

// =============================================================================
// LLM - Windows AI (Phi Silica) Stubs
// =============================================================================
//
// The Windows Phi Silica APIs (Microsoft.Windows.AI.Text) are part of the Windows
// App SDK, which uses the `Microsoft.*` WinRT namespace. The `windows` crate v0.58
// only covers `Windows.*` namespaces. Full implementation requires either:
// 1. The `windows` crate adding Microsoft.Windows.AI.Text support
// 2. Using `windows-bindgen` with Windows App SDK .winmd metadata files
// 3. A C++/WinRT bridge similar to the macOS Swift FFI approach

const WINDOWS_LLM_REASON: &str =
    "Windows Phi Silica APIs (Microsoft.Windows.AI.Text) require Windows App SDK bindings not yet available from Rust";

fn llm_not_available<T>() -> crate::Result<T> {
    Err(crate::Error::LlmNotAvailable {
        reason: WINDOWS_LLM_REASON.to_string(),
    })
}

pub fn llm_check_availability() -> crate::Result<LlmAvailability> {
    Ok(LlmAvailability {
        available: false,
        reason: Some(WINDOWS_LLM_REASON.to_string()),
    })
}

pub fn llm_get_model_info() -> crate::Result<LlmModelInfo> {
    llm_not_available()
}

pub fn llm_generate(_options: LlmGenerateOptions) -> crate::Result<LlmGenerateResult> {
    llm_not_available()
}

pub fn llm_generate_stream<F>(_options: LlmGenerateOptions, _on_event: F) -> crate::Result<()>
where
    F: FnMut(LlmStreamEvent) -> crate::Result<()> + 'static,
{
    llm_not_available()
}

pub fn llm_create_session(_options: LlmSessionOptions) -> crate::Result<LlmSessionId> {
    llm_not_available()
}

pub fn llm_session_send(_session_id: &str, _message: &str) -> crate::Result<LlmGenerateResult> {
    llm_not_available()
}

pub fn llm_session_send_stream<F>(
    _session_id: &str,
    _message: &str,
    _on_event: F,
) -> crate::Result<()>
where
    F: FnMut(LlmStreamEvent) -> crate::Result<()> + 'static,
{
    llm_not_available()
}

pub fn llm_destroy_session(_session_id: &str) -> crate::Result<()> {
    llm_not_available()
}

pub fn llm_summarize(_options: LlmSummarizeOptions) -> crate::Result<LlmSummarizeResult> {
    llm_not_available()
}

pub fn llm_rewrite(_options: LlmRewriteOptions) -> crate::Result<LlmRewriteResult> {
    llm_not_available()
}
