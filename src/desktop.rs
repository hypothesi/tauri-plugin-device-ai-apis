use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;
use crate::{DeviceAi, Error};

pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<DeviceAiApis<R>> {
    Ok(DeviceAiApis {
        inner: DeviceAi::new(),
        runtime: PhantomData,
    })
}

/// Access to the device-ai-apis APIs on desktop platforms.
pub struct DeviceAiApis<R: Runtime> {
    inner: DeviceAi,
    runtime: PhantomData<fn() -> R>,
}

impl<R: Runtime> DeviceAiApis<R> {
    /// Get platform capabilities.
    pub fn get_capabilities(&self) -> Capabilities {
        self.inner.capabilities()
    }

    /// Perform one-shot speech recognition.
    pub fn speech_recognize(
        &self,
        options: RecognitionOptions,
    ) -> crate::Result<RecognitionResult> {
        self.inner.speech().recognize(options)
    }

    /// Start streaming speech recognition.
    pub fn speech_recognize_start(
        &self,
        options: RecognitionOptions,
    ) -> crate::Result<SpeechSessionId> {
        self.inner.speech().start_recognition(options)
    }

    /// Stop streaming speech recognition.
    pub fn speech_recognize_stop(
        &self,
        session_id: SpeechSessionId,
    ) -> crate::Result<RecognitionResult> {
        self.inner.speech().stop_recognition(session_id)
    }

    /// Synthesize and speak text.
    pub fn speech_synthesize(&self, text: &str, options: SynthesisOptions) -> crate::Result<()> {
        self.inner.speech().speak(text, options)
    }

    /// Get available voices for speech synthesis.
    pub fn speech_get_voices(&self) -> crate::Result<Vec<Voice>> {
        self.inner.speech().voices()
    }

    /// Recognize text in an image.
    pub fn vision_recognize_text(
        &self,
        image: ImageSource,
        options: OcrOptions,
    ) -> crate::Result<TextRecognitionResult> {
        self.inner.vision().recognize_text(image, options)
    }

    /// Detect barcodes in an image.
    pub fn vision_detect_barcodes(
        &self,
        image: ImageSource,
        options: BarcodeOptions,
    ) -> crate::Result<Vec<Barcode>> {
        self.inner.vision().detect_barcodes(image, options)
    }

    /// Detect faces in an image.
    pub fn vision_detect_faces(
        &self,
        image: ImageSource,
        options: FaceOptions,
    ) -> crate::Result<Vec<Face>> {
        self.inner.vision().detect_faces(image, options)
    }

    /// Classify an image.
    pub fn vision_classify_image(
        &self,
        image: ImageSource,
        options: ClassificationOptions,
    ) -> crate::Result<Vec<Classification>> {
        self.inner.vision().classify_image(image, options)
    }

    /// Identify the language of text.
    pub fn text_identify_language(&self, text: &str) -> crate::Result<LanguageIdentification> {
        self.inner.text().identify_language(text)
    }

    /// Translate text between languages.
    pub fn text_translate(&self, text: &str, from: &str, to: &str) -> crate::Result<Translation> {
        self.inner.text().translate(text, from, to)
    }

    /// Check if on-device language model is available.
    pub fn llm_check_availability(&self) -> crate::Result<LlmAvailability> {
        self.inner.llm().check_availability()
    }

    /// Get language model information.
    pub fn llm_get_model_info(&self) -> crate::Result<LlmModelInfo> {
        self.inner.llm().model_info()
    }

    /// Generate text using the on-device language model.
    pub fn llm_generate(&self, options: LlmGenerateOptions) -> crate::Result<LlmGenerateResult> {
        self.inner.llm().generate(options)
    }

    /// Stream text generation from the on-device language model.
    pub fn llm_generate_stream(
        &self,
        options: LlmGenerateOptions,
        channel: tauri::ipc::Channel<LlmStreamEvent>,
    ) -> crate::Result<()> {
        self.inner.llm().generate_stream(options, move |event| {
            channel.send(event).map_err(stream_channel_error)
        })
    }

    /// Create a multi-turn language model session.
    pub fn llm_create_session(&self, options: LlmSessionOptions) -> crate::Result<LlmSessionId> {
        self.inner.llm().create_session(options)
    }

    /// Send a message in a language model session.
    pub fn llm_session_send(
        &self,
        session_id: LlmSessionId,
        message: String,
    ) -> crate::Result<LlmGenerateResult> {
        self.inner.llm().session_send(session_id, message)
    }

    /// Stream a response in a language model session.
    pub fn llm_session_send_stream(
        &self,
        session_id: LlmSessionId,
        message: String,
        channel: tauri::ipc::Channel<LlmStreamEvent>,
    ) -> crate::Result<()> {
        self.inner
            .llm()
            .session_send_stream(session_id, message, move |event| {
                channel.send(event).map_err(stream_channel_error)
            })
    }

    /// Destroy a language model session.
    pub fn llm_destroy_session(&self, session_id: LlmSessionId) -> crate::Result<()> {
        self.inner.llm().destroy_session(session_id)
    }

    /// Summarize text using the on-device language model.
    pub fn llm_summarize(&self, options: LlmSummarizeOptions) -> crate::Result<LlmSummarizeResult> {
        self.inner.llm().summarize(options)
    }

    /// Rewrite text using the on-device language model.
    pub fn llm_rewrite(&self, options: LlmRewriteOptions) -> crate::Result<LlmRewriteResult> {
        self.inner.llm().rewrite(options)
    }
}

fn stream_channel_error(error: impl std::fmt::Display) -> Error {
    Error::LlmGenerationFailed {
        message: format!("Failed to emit streaming event: {error}"),
    }
}
