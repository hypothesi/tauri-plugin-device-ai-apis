//! # tauri-plugin-device-ai-apis
//!
//! A Tauri plugin that provides cross-platform access to device-native AI capabilities
//! including speech recognition, text-to-speech, vision (OCR, barcode, face detection),
//! natural language processing, and on-device language models.
//!
//! ## Features
//!
//! - **Speech Recognition**: Convert speech to text using platform-native APIs
//! - **Speech Synthesis**: Text-to-speech with voice selection
//! - **Vision**: OCR, barcode/QR detection, face detection, image classification
//! - **Text Processing**: Language identification, translation (where available)
//! - **Language Model**: On-device text generation, streaming, multi-turn sessions,
//!   summarization, and rewriting (macOS 26+, iOS 26+)
//!
//! ## Platform Support
//!
//! | Feature | macOS | iOS | Android | Windows | Web |
//! |---------|-------|-----|---------|---------|-----|
//! | Speech Recognition | ✓ | ✓ | ✓ | ✓ | ✓ |
//! | Speech Synthesis | ✓ | ✓ | ✓ | ✓ | ✓ |
//! | Text Recognition | ✓ | ✓ | ✓ | ✓ | ✗ |
//! | Barcode Detection | ✓ | ✓ | ✓ | ✗ | △ |
//! | Face Detection | ✓ | ✓ | ✓ | ✗ | ✗ |
//! | Language Model | ✓† | ✓† | ✗ | ✗ | ✗ |
//!
//! Legend: ✓ = Available, ✗ = Not available, △ = Partial support,
//! † = Requires macOS 26+ / iOS 26+

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use device_ai::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::DeviceAiApis;
#[cfg(mobile)]
use mobile::DeviceAiApis;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the device-ai-apis APIs.
pub trait DeviceAiApisExt<R: Runtime> {
    fn device_ai_apis(&self) -> &DeviceAiApis<R>;
}

impl<R: Runtime, T: Manager<R>> DeviceAiApisExt<R> for T {
    fn device_ai_apis(&self) -> &DeviceAiApis<R> {
        self.state::<DeviceAiApis<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("device-ai-apis")
        .invoke_handler(tauri::generate_handler![
            // Capabilities
            commands::get_capabilities,
            // Speech
            commands::speech_recognize,
            commands::speech_recognize_start,
            commands::speech_recognize_stop,
            commands::speech_synthesize,
            commands::speech_get_voices,
            // Vision
            commands::vision_recognize_text,
            commands::vision_detect_barcodes,
            commands::vision_detect_faces,
            commands::vision_classify_image,
            // Text
            commands::text_identify_language,
            commands::text_translate,
            // LLM
            commands::llm_check_availability,
            commands::llm_get_model_info,
            commands::llm_generate,
            commands::llm_generate_stream,
            commands::llm_create_session,
            commands::llm_session_send,
            commands::llm_session_send_stream,
            commands::llm_destroy_session,
            commands::llm_summarize,
            commands::llm_rewrite,
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let device_ai_apis = mobile::init(app, api)?;
            #[cfg(desktop)]
            let device_ai_apis = desktop::init(app, api)?;
            app.manage(device_ai_apis);
            Ok(())
        })
        .build()
}
