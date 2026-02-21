//! Language model Tauri commands.

use tauri::{command, ipc::Channel, AppHandle, Runtime};

use crate::models::*;
use crate::{DeviceAiApisExt, Result};

/// Check if on-device language model is available.
#[command]
pub async fn llm_check_availability<R: Runtime>(app: AppHandle<R>) -> Result<LlmAvailability> {
    app.device_ai_apis().llm_check_availability()
}

/// Get language model information.
#[command]
pub async fn llm_get_model_info<R: Runtime>(app: AppHandle<R>) -> Result<LlmModelInfo> {
    app.device_ai_apis().llm_get_model_info()
}

/// Generate text using the on-device language model.
#[command]
pub async fn llm_generate<R: Runtime>(
    app: AppHandle<R>,
    options: LlmGenerateOptions,
) -> Result<LlmGenerateResult> {
    app.device_ai_apis().llm_generate(options)
}

/// Stream text generation from the on-device language model.
#[command]
pub async fn llm_generate_stream<R: Runtime>(
    app: AppHandle<R>,
    options: LlmGenerateOptions,
    on_event: Channel<LlmStreamEvent>,
) -> Result<()> {
    app.device_ai_apis().llm_generate_stream(options, on_event)
}

/// Create a multi-turn language model session.
#[command]
pub async fn llm_create_session<R: Runtime>(
    app: AppHandle<R>,
    options: Option<LlmSessionOptions>,
) -> Result<LlmSessionId> {
    let options = options.unwrap_or_default();
    app.device_ai_apis().llm_create_session(options)
}

/// Send a message in a language model session.
#[command]
pub async fn llm_session_send<R: Runtime>(
    app: AppHandle<R>,
    session_id: LlmSessionId,
    message: String,
) -> Result<LlmGenerateResult> {
    app.device_ai_apis().llm_session_send(session_id, message)
}

/// Stream a response in a language model session.
#[command]
pub async fn llm_session_send_stream<R: Runtime>(
    app: AppHandle<R>,
    session_id: LlmSessionId,
    message: String,
    on_event: Channel<LlmStreamEvent>,
) -> Result<()> {
    app.device_ai_apis()
        .llm_session_send_stream(session_id, message, on_event)
}

/// Destroy a language model session.
#[command]
pub async fn llm_destroy_session<R: Runtime>(
    app: AppHandle<R>,
    session_id: LlmSessionId,
) -> Result<()> {
    app.device_ai_apis().llm_destroy_session(session_id)
}

/// Summarize text using the on-device language model.
#[command]
pub async fn llm_summarize<R: Runtime>(
    app: AppHandle<R>,
    options: LlmSummarizeOptions,
) -> Result<LlmSummarizeResult> {
    app.device_ai_apis().llm_summarize(options)
}

/// Rewrite text using the on-device language model.
#[command]
pub async fn llm_rewrite<R: Runtime>(
    app: AppHandle<R>,
    options: LlmRewriteOptions,
) -> Result<LlmRewriteResult> {
    app.device_ai_apis().llm_rewrite(options)
}
