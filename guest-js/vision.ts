/**
 * Vision API for image processing.
 * @module vision
 */

import { invoke } from "@tauri-apps/api/core";
import type {
  Barcode,
  BarcodeOptions,
  Classification,
  ClassificationOptions,
  Face,
  FaceOptions,
  ImageSource,
  OcrOptions,
  TextRecognitionResult,
} from "./types";
import { isTauri, hasBarcodeDetection } from "./platform";
import { webDetectBarcodes } from "./web-barcode";

/**
 * Recognize text in an image (OCR).
 *
 * @param image The image to process.
 * @param options OCR options.
 * @returns The text recognition result.
 *
 * @example
 * ```typescript
 * import { vision } from '@hypothesi/tauri-plugin-device-ai-apis';
 *
 * // From base64
 * const result = await vision.recognizeText({ base64: imageData });
 * console.log('Found text:', result.text);
 *
 * // From file path
 * const result2 = await vision.recognizeText({ filePath: '/path/to/image.png' });
 * ```
 */
export async function recognizeText(
  image: ImageSource,
  options?: OcrOptions,
): Promise<TextRecognitionResult> {
  if (!isTauri()) {
    throw new Error("Text recognition (OCR) requires native platform APIs and is not available in the browser.");
  }
  return invoke<TextRecognitionResult>("plugin:device-ai-apis|vision_recognize_text", {
    image,
    options: options ?? {},
  });
}

/**
 * Detect barcodes in an image.
 *
 * Uses native APIs when running in Tauri, falls back to Web Barcode Detection API
 * when running in a browser (Chrome 83+, Edge 83+, Opera 69+).
 *
 * @param image The image to process.
 * @param options Barcode detection options.
 * @returns The detected barcodes.
 *
 * @example
 * ```typescript
 * import { vision } from '@hypothesi/tauri-plugin-device-ai-apis';
 *
 * const barcodes = await vision.detectBarcodes({ base64: imageData });
 * for (const barcode of barcodes) {
 *   console.log(`Found ${barcode.format}: ${barcode.rawValue}`);
 * }
 * ```
 */
export async function detectBarcodes(
  image: ImageSource,
  options?: BarcodeOptions,
): Promise<Barcode[]> {
  if (isTauri()) {
    return invoke<Barcode[]>("plugin:device-ai-apis|vision_detect_barcodes", {
      image,
      options: options ?? {},
    });
  }
  if (hasBarcodeDetection()) {
    return webDetectBarcodes(image, options);
  }
  throw new Error("Barcode detection not available on this platform");
}

/**
 * Detect faces in an image.
 *
 * @param image The image to process.
 * @param options Face detection options.
 * @returns The detected faces.
 *
 * @example
 * ```typescript
 * import { vision } from '@hypothesi/tauri-plugin-device-ai-apis';
 *
 * const faces = await vision.detectFaces(
 *   { base64: imageData },
 *   { detectLandmarks: true, classifyAttributes: true }
 * );
 * console.log(`Found ${faces.length} faces`);
 * ```
 */
export async function detectFaces(image: ImageSource, options?: FaceOptions): Promise<Face[]> {
  if (!isTauri()) {
    throw new Error("Face detection requires native platform APIs and is not available in the browser.");
  }
  return invoke<Face[]>("plugin:device-ai-apis|vision_detect_faces", {
    image,
    options: options ?? {},
  });
}

/**
 * Classify/label an image.
 *
 * @param image The image to process.
 * @param options Classification options.
 * @returns The classifications.
 *
 * @example
 * ```typescript
 * import { vision } from '@hypothesi/tauri-plugin-device-ai-apis';
 *
 * const labels = await vision.classifyImage(
 *   { base64: imageData },
 *   { maxResults: 5, minConfidence: 0.7 }
 * );
 * for (const label of labels) {
 *   console.log(`${label.identifier}: ${label.confidence}`);
 * }
 * ```
 */
export async function classifyImage(
  image: ImageSource,
  options?: ClassificationOptions,
): Promise<Classification[]> {
  if (!isTauri()) {
    throw new Error("Image classification requires native platform APIs and is not available in the browser.");
  }
  return invoke<Classification[]>("plugin:device-ai-apis|vision_classify_image", {
    image,
    options: options ?? {},
  });
}
