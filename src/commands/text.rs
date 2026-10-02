//! Text processing Tauri commands.

use tauri::{command, AppHandle, Runtime};

use crate::models::{LanguageIdentification, Translation, TranslationAvailability};
use crate::{DeviceAiApisExt, Result};

/// Identify the language of text.
#[command]
pub async fn text_identify_language<R: Runtime>(
    app: AppHandle<R>,
    text: String,
) -> Result<LanguageIdentification> {
    app.device_ai_apis().text_identify_language(&text)
}

/// Check on-device translation availability between two languages.
#[command]
pub async fn text_check_translation_availability<R: Runtime>(
    app: AppHandle<R>,
    from: String,
    to: String,
) -> Result<TranslationAvailability> {
    app.device_ai_apis()
        .text_check_translation_availability(&from, &to)
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
