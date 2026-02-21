/**
 * Web Barcode Detection API implementation for browser fallback.
 * @module web-barcode
 */

import type { Barcode, BarcodeFormat, BarcodeOptions, ImageSource, BoundingBox } from "./types";

// Type definitions for Barcode Detection API
interface DetectedBarcode {
  rawValue: string;
  format: string;
  boundingBox: DOMRectReadOnly;
  cornerPoints: { x: number; y: number }[];
}

interface BarcodeDetector {
  detect(image: ImageBitmapSource): Promise<DetectedBarcode[]>;
}

interface BarcodeDetectorConstructor {
  new (options?: { formats?: string[] }): BarcodeDetector;
  getSupportedFormats(): Promise<string[]>;
}

declare global {
  interface Window {
    BarcodeDetector?: BarcodeDetectorConstructor;
  }
}

/**
 * Map our format types to Web Barcode Detection API format names.
 */
const FORMAT_MAP: Record<BarcodeFormat, string> = {
  qr_code: "qr_code",
  aztec: "aztec",
  codabar: "codabar",
  code_39: "code_39",
  code_93: "code_93",
  code_128: "code_128",
  data_matrix: "data_matrix",
  ean_8: "ean_8",
  ean_13: "ean_13",
  itf: "itf",
  pdf417: "pdf417",
  upc_a: "upc_a",
  upc_e: "upc_e",
};

/**
 * Map Web Barcode Detection API format names back to our format types.
 */
const REVERSE_FORMAT_MAP: Record<string, BarcodeFormat> = {
  qr_code: "qr_code",
  aztec: "aztec",
  codabar: "codabar",
  code_39: "code_39",
  code_93: "code_93",
  code_128: "code_128",
  data_matrix: "data_matrix",
  ean_8: "ean_8",
  ean_13: "ean_13",
  itf: "itf",
  pdf417: "pdf417",
  upc_a: "upc_a",
  upc_e: "upc_e",
};

/**
 * Get supported barcode formats from the Web Barcode Detection API.
 */
export async function webGetSupportedFormats(): Promise<BarcodeFormat[]> {
  if (typeof window === "undefined" || !window.BarcodeDetector) {
    return [];
  }

  try {
    const formats = await window.BarcodeDetector.getSupportedFormats();
    return formats
      .map((f) => REVERSE_FORMAT_MAP[f])
      .filter((f): f is BarcodeFormat => f !== undefined);
  } catch {
    return [];
  }
}

/**
 * Load an image from various sources into an ImageBitmap.
 */
async function loadImage(source: ImageSource): Promise<ImageBitmap> {
  if ("base64" in source) {
    // Handle base64
    const response = await fetch(`data:image/png;base64,${source.base64}`);
    const blob = await response.blob();
    return createImageBitmap(blob);
  }

  if ("bytes" in source) {
    // Handle byte array
    const uint8 = new Uint8Array(source.bytes);
    const blob = new Blob([uint8]);
    return createImageBitmap(blob);
  }

  if ("filePath" in source) {
    // File paths not supported in web context without backend
    throw new Error("File paths not supported in web context");
  }

  throw new Error("Invalid image source");
}

/**
 * Detect barcodes in an image using the Web Barcode Detection API.
 */
export async function webDetectBarcodes(
  source: ImageSource,
  options?: BarcodeOptions,
): Promise<Barcode[]> {
  if (typeof window === "undefined" || !window.BarcodeDetector) {
    throw new Error("Barcode Detection API not available");
  }

  // Map our formats to Web API formats
  const webFormats = options?.formats
    ?.map((f) => FORMAT_MAP[f])
    .filter((f): f is string => f !== undefined);

  const detector = new window.BarcodeDetector(
    webFormats && webFormats.length > 0 ? { formats: webFormats } : undefined,
  );

  const image = await loadImage(source);
  const detected = await detector.detect(image);

  // Get image dimensions for normalization
  const imageWidth = image.width;
  const imageHeight = image.height;

  return detected.map((barcode): Barcode => {
    const format = REVERSE_FORMAT_MAP[barcode.format] ?? "qr_code";
    const bbox = barcode.boundingBox;

    // Normalize bounding box to 0.0-1.0 range
    const boundingBox: BoundingBox = {
      x: bbox.x / imageWidth,
      y: bbox.y / imageHeight,
      width: bbox.width / imageWidth,
      height: bbox.height / imageHeight,
    };

    return {
      format,
      rawValue: barcode.rawValue,
      boundingBox,
    };
  });
}
