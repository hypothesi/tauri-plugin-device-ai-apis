//! Vision-related Tauri commands.

use tauri::{command, AppHandle, Runtime};

use crate::models::{
    Barcode, BarcodeOptions, Classification, ClassificationOptions, Face, FaceOptions, ImageSource,
    OcrOptions, TextRecognitionResult,
};
use crate::{DeviceAiApisExt, Result};

/// Recognize text in an image (OCR).
#[command]
pub async fn vision_recognize_text<R: Runtime>(
    app: AppHandle<R>,
    image: ImageSource,
    options: Option<OcrOptions>,
) -> Result<TextRecognitionResult> {
    let options = options.unwrap_or_default();
    app.device_ai_apis().vision_recognize_text(image, options)
}

/// Detect barcodes in an image.
#[command]
pub async fn vision_detect_barcodes<R: Runtime>(
    app: AppHandle<R>,
    image: ImageSource,
    options: Option<BarcodeOptions>,
) -> Result<Vec<Barcode>> {
    let options = options.unwrap_or_default();
    app.device_ai_apis().vision_detect_barcodes(image, options)
}

/// Detect faces in an image.
#[command]
pub async fn vision_detect_faces<R: Runtime>(
    app: AppHandle<R>,
    image: ImageSource,
    options: Option<FaceOptions>,
) -> Result<Vec<Face>> {
    let options = options.unwrap_or_default();
    app.device_ai_apis().vision_detect_faces(image, options)
}

/// Classify an image.
#[command]
pub async fn vision_classify_image<R: Runtime>(
    app: AppHandle<R>,
    image: ImageSource,
    options: Option<ClassificationOptions>,
) -> Result<Vec<Classification>> {
    let options = options.unwrap_or_default();
    app.device_ai_apis().vision_classify_image(image, options)
}
