//! Text processing Tauri commands.

use tauri::{command, AppHandle, Runtime};

use crate::models::{LanguageIdentification, Translation};
use crate::{DeviceAiApisExt, Result};

/// Identify the language of text.
#[command]
pub async fn text_identify_language<R: Runtime>(
    app: AppHandle<R>,
    text: String,
) -> Result<LanguageIdentification> {
    app.device_ai_apis().text_identify_language(&text)
}

/// Translate text between languages.
#[command]
pub async fn text_translate<R: Runtime>(
    app: AppHandle<R>,
    text: String,
    from: String,
    to: String,
) -> Result<Translation> {
    app.device_ai_apis().text_translate(&text, &from, &to)
}
