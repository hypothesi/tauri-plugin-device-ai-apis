//! Speech-related Tauri commands.

use tauri::{command, AppHandle, Runtime};

use crate::models::{
    RecognitionOptions, RecognitionResult, SpeechSessionId, SynthesisOptions, Voice,
};
use crate::{DeviceAiApisExt, Result};

/// Perform one-shot speech recognition.
#[command]
pub async fn speech_recognize<R: Runtime>(
    app: AppHandle<R>,
    options: Option<RecognitionOptions>,
) -> Result<RecognitionResult> {
    let options = options.unwrap_or_default();
    app.device_ai_apis().speech_recognize(options)
}

/// Start streaming speech recognition.
#[command]
pub async fn speech_recognize_start<R: Runtime>(
    app: AppHandle<R>,
    options: Option<RecognitionOptions>,
) -> Result<SpeechSessionId> {
    let options = options.unwrap_or_default();
    app.device_ai_apis().speech_recognize_start(options)
}

/// Stop streaming speech recognition.
#[command]
pub async fn speech_recognize_stop<R: Runtime>(
    app: AppHandle<R>,
    session_id: SpeechSessionId,
) -> Result<RecognitionResult> {
    app.device_ai_apis().speech_recognize_stop(session_id)
}

/// Synthesize and speak text.
#[command]
pub async fn speech_synthesize<R: Runtime>(
    app: AppHandle<R>,
    text: String,
    options: Option<SynthesisOptions>,
) -> Result<()> {
    let options = options.unwrap_or_default();
    app.device_ai_apis().speech_synthesize(&text, options)
}

/// Get available voices for speech synthesis.
#[command]
pub async fn speech_get_voices<R: Runtime>(app: AppHandle<R>) -> Result<Vec<Voice>> {
    app.device_ai_apis().speech_get_voices()
}
