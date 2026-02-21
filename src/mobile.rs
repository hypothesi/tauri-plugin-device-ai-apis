use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_device_ai_apis);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<DeviceAiApis<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.hypothesi.device_ai_apis", "DeviceAiPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_device_ai_apis)?;
    Ok(DeviceAiApis(handle))
}

/// Access to the device-ai-apis APIs on mobile platforms.
pub struct DeviceAiApis<R: Runtime>(PluginHandle<R>);

fn mobile_invoke_error(error: tauri::plugin::mobile::PluginInvokeError) -> crate::Error {
    crate::Error::Platform(error.to_string())
}

impl<R: Runtime> DeviceAiApis<R> {
    // =========================================================================
    // Speech Recognition
    // =========================================================================

    /// Perform one-shot speech recognition.
    pub fn speech_recognize(
        &self,
        options: RecognitionOptions,
    ) -> crate::Result<RecognitionResult> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            options: RecognitionOptions,
        }
        self.0
            .run_mobile_plugin("speechRecognize", Args { options })
            .map_err(mobile_invoke_error)
    }

    /// Start streaming speech recognition.
    pub fn speech_recognize_start(
        &self,
        options: RecognitionOptions,
    ) -> crate::Result<SpeechSessionId> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            options: RecognitionOptions,
        }
        self.0
            .run_mobile_plugin("speechRecognizeStart", Args { options })
            .map_err(mobile_invoke_error)
    }

    /// Stop streaming speech recognition.
    pub fn speech_recognize_stop(
        &self,
        session_id: SpeechSessionId,
    ) -> crate::Result<RecognitionResult> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            session_id: SpeechSessionId,
        }
        self.0
            .run_mobile_plugin("speechRecognizeStop", Args { session_id })
            .map_err(mobile_invoke_error)
    }

    // =========================================================================
    // Speech Synthesis
    // =========================================================================

    /// Synthesize and speak text.
    pub fn speech_synthesize(&self, text: &str, options: SynthesisOptions) -> crate::Result<()> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args<'a> {
            text: &'a str,
            options: SynthesisOptions,
        }
        self.0
            .run_mobile_plugin("speechSynthesize", Args { text, options })
            .map_err(mobile_invoke_error)
    }

    /// Get available voices for speech synthesis.
    pub fn speech_get_voices(&self) -> crate::Result<Vec<Voice>> {
        self.0
            .run_mobile_plugin("speechGetVoices", ())
            .map_err(mobile_invoke_error)
    }

    // =========================================================================
    // Vision - Text Recognition
    // =========================================================================

    /// Recognize text in an image.
    pub fn vision_recognize_text(
        &self,
        image: ImageSource,
        options: OcrOptions,
    ) -> crate::Result<TextRecognitionResult> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            image: ImageSource,
            options: OcrOptions,
        }
        self.0
            .run_mobile_plugin("visionRecognizeText", Args { image, options })
            .map_err(mobile_invoke_error)
    }

    // =========================================================================
    // Vision - Barcode Detection
    // =========================================================================

    /// Detect barcodes in an image.
    pub fn vision_detect_barcodes(
        &self,
        image: ImageSource,
        options: BarcodeOptions,
    ) -> crate::Result<Vec<Barcode>> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            image: ImageSource,
            options: BarcodeOptions,
        }
        self.0
            .run_mobile_plugin("visionDetectBarcodes", Args { image, options })
            .map_err(mobile_invoke_error)
    }

    // =========================================================================
    // Vision - Face Detection
    // =========================================================================

    /// Detect faces in an image.
    pub fn vision_detect_faces(
        &self,
        image: ImageSource,
        options: FaceOptions,
    ) -> crate::Result<Vec<Face>> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            image: ImageSource,
            options: FaceOptions,
        }
        self.0
            .run_mobile_plugin("visionDetectFaces", Args { image, options })
            .map_err(mobile_invoke_error)
    }

    // =========================================================================
    // Vision - Image Classification
    // =========================================================================

    /// Classify an image.
    pub fn vision_classify_image(
        &self,
        image: ImageSource,
        options: ClassificationOptions,
    ) -> crate::Result<Vec<Classification>> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            image: ImageSource,
            options: ClassificationOptions,
        }
        self.0
            .run_mobile_plugin("visionClassifyImage", Args { image, options })
            .map_err(mobile_invoke_error)
    }

    // =========================================================================
    // Text Processing
    // =========================================================================

    /// Identify the language of text.
    pub fn text_identify_language(&self, text: &str) -> crate::Result<LanguageIdentification> {
        #[derive(serde::Serialize)]
        struct Args<'a> {
            text: &'a str,
        }
        self.0
            .run_mobile_plugin("textIdentifyLanguage", Args { text })
            .map_err(mobile_invoke_error)
    }

    /// Translate text between languages.
    pub fn text_translate(&self, text: &str, from: &str, to: &str) -> crate::Result<Translation> {
        #[derive(serde::Serialize)]
        struct Args<'a> {
            text: &'a str,
            from: &'a str,
            to: &'a str,
        }
        self.0
            .run_mobile_plugin("textTranslate", Args { text, from, to })
            .map_err(mobile_invoke_error)
    }

    // =========================================================================
    // Language Model
    // =========================================================================

    /// Check if on-device language model is available.
    pub fn llm_check_availability(&self) -> crate::Result<LlmAvailability> {
        self.0
            .run_mobile_plugin("llmCheckAvailability", ())
            .map_err(mobile_invoke_error)
    }

    /// Get language model information.
    pub fn llm_get_model_info(&self) -> crate::Result<LlmModelInfo> {
        self.0
            .run_mobile_plugin("llmGetModelInfo", ())
            .map_err(mobile_invoke_error)
    }

    /// Generate text using the on-device language model.
    pub fn llm_generate(&self, options: LlmGenerateOptions) -> crate::Result<LlmGenerateResult> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            options: LlmGenerateOptions,
        }
        self.0
            .run_mobile_plugin("llmGenerate", Args { options })
            .map_err(mobile_invoke_error)
    }

    /// Stream text generation (falls back to non-streaming on mobile).
    pub fn llm_generate_stream(
        &self,
        options: LlmGenerateOptions,
        channel: tauri::ipc::Channel<LlmStreamEvent>,
    ) -> crate::Result<()> {
        // Mobile doesn't support Channel via run_mobile_plugin, so we fall back to
        // non-streaming: generate the full result and send it as a single Done event.
        let result = self.llm_generate(options)?;
        let _ = channel.send(LlmStreamEvent::Done {
            content: result.content,
            finish_reason: result.finish_reason,
            usage: result.usage,
        });
        Ok(())
    }

    /// Create a multi-turn language model session.
    pub fn llm_create_session(&self, options: LlmSessionOptions) -> crate::Result<LlmSessionId> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            options: LlmSessionOptions,
        }
        self.0
            .run_mobile_plugin("llmCreateSession", Args { options })
            .map_err(mobile_invoke_error)
    }

    /// Send a message in a language model session.
    pub fn llm_session_send(
        &self,
        session_id: LlmSessionId,
        message: String,
    ) -> crate::Result<LlmGenerateResult> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            session_id: LlmSessionId,
            message: String,
        }
        self.0
            .run_mobile_plugin(
                "llmSessionSend",
                Args {
                    session_id,
                    message,
                },
            )
            .map_err(mobile_invoke_error)
    }

    /// Stream a response in a language model session (falls back to non-streaming on
    /// mobile).
    pub fn llm_session_send_stream(
        &self,
        session_id: LlmSessionId,
        message: String,
        channel: tauri::ipc::Channel<LlmStreamEvent>,
    ) -> crate::Result<()> {
        let result = self.llm_session_send(session_id, message)?;
        let _ = channel.send(LlmStreamEvent::Done {
            content: result.content,
            finish_reason: result.finish_reason,
            usage: result.usage,
        });
        Ok(())
    }

    /// Destroy a language model session.
    pub fn llm_destroy_session(&self, session_id: LlmSessionId) -> crate::Result<()> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            session_id: LlmSessionId,
        }
        self.0
            .run_mobile_plugin("llmDestroySession", Args { session_id })
            .map_err(mobile_invoke_error)
    }

    /// Summarize text using the on-device language model.
    pub fn llm_summarize(&self, options: LlmSummarizeOptions) -> crate::Result<LlmSummarizeResult> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            options: LlmSummarizeOptions,
        }
        self.0
            .run_mobile_plugin("llmSummarize", Args { options })
            .map_err(mobile_invoke_error)
    }

    /// Rewrite text using the on-device language model.
    pub fn llm_rewrite(&self, options: LlmRewriteOptions) -> crate::Result<LlmRewriteResult> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Args {
            options: LlmRewriteOptions,
        }
        self.0
            .run_mobile_plugin("llmRewrite", Args { options })
            .map_err(mobile_invoke_error)
    }
}
