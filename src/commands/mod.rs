//! Tauri commands for the device AI plugin.
//!
//! This module contains all the IPC commands exposed to the frontend.

mod llm;
mod speech;
mod text;
mod vision;

pub use llm::*;
pub use speech::*;
pub use text::*;
pub use vision::*;

use tauri::{command, AppHandle, Runtime};

use crate::models::Capabilities;
use crate::{DeviceAiApisExt, Result};

/// Get the AI capabilities available on the current platform.
#[command]
pub async fn get_capabilities<R: Runtime>(app: AppHandle<R>) -> Result<Capabilities> {
    Ok(app.device_ai_apis().get_capabilities())
}
