import { blobToBase64, getErrorMessage } from "./demo-utils.js";

function createPendingResult(sample) {
  return {
    name: sample.type,
    description: sample.description,
    status: "pending",
    details: "",
  };
}

async function fetchSampleBlob(fetchImpl, fileName) {
  const response = await fetchImpl(`/samples/${fileName}`);

  if (!response.ok) {
    throw new Error(`Failed to load ${fileName}`);
  }

  return response.blob();
}

async function runFaceDetectionTest(result, sample, base64, visionApi) {
  const faces = await visionApi.detectFaces({ base64 });
  const count = Array.isArray(faces) ? faces.length : (faces.faces || []).length;

  if (count >= sample.expected.minFaces) {
    result.status = "pass";
    result.details = `Detected ${count} faces (expected at least ${sample.expected.minFaces}).`;
    return;
  }

  result.status = "fail";
  result.details = `Detected ${count} faces (expected at least ${sample.expected.minFaces}).`;
}

async function runOcrTest(result, sample, base64, visionApi) {
  const ocr = await visionApi.recognizeText({ base64 });
  const textContent = ocr.text || "";
  const missing = sample.expected.contains.filter((word) => {
    return !textContent.toLowerCase().includes(word.toLowerCase());
  });

  if (missing.length === 0) {
    result.status = "pass";
    result.details = `Found all expected words: ${sample.expected.contains.join(", ")}.`;
    return;
  }

  result.status = "fail";
  result.details = `Missing words: ${missing.join(", ")}.`;
}

async function runBarcodeTest(result, sample, base64, visionApi) {
  const barcodes = await visionApi.detectBarcodes({ base64 });
  const list = Array.isArray(barcodes) ? barcodes : barcodes.barcodes || [];
  const match = list.find((barcode) => {
    return (
      barcode.format === sample.expected.format && barcode.rawValue === sample.expected.rawValue
    );
  });

  if (match) {
    result.status = "pass";
    result.details = `Found ${sample.expected.format} with value ${sample.expected.rawValue}.`;
    return;
  }

  result.status = "fail";
  result.details =
    `Expected ${sample.expected.format} (${sample.expected.rawValue}) but found ` +
    `${list.length} barcode(s).`;
}

async function runClassificationTest(result, sample, base64, visionApi) {
  const classifications = await visionApi.classifyImage({ base64 });
  const list = Array.isArray(classifications)
    ? classifications
    : classifications.classifications || [];
  const match = list.find((classification) => {
    return classification.identifier
      .toLowerCase()
      .includes(sample.expected.className.toLowerCase());
  });

  if (match) {
    result.status = "pass";
    result.details = `Classified as ${match.identifier} (${(match.confidence * 100).toFixed(1)}%).`;
    return;
  }

  result.status = "fail";
  result.details = `Expected ${sample.expected.className}, found ${list.map((item) => item.identifier).join(", ")}.`;
}

async function runSpeechRecognitionTest(result, sample, base64, speechApi) {
  const speechResult = await speechApi.recognize({
    language: "en-US",
    audioSource: { base64 },
  });
  const expectedText = sample.expected.text.toLowerCase();
  const actualText = speechResult.text.toLowerCase();
  const expectedWords = expectedText.split(" ").filter((word) => {
    return word.length > 3;
  });
  const matchedWords = expectedWords.filter((word) => {
    return actualText.includes(word);
  });
  const matchRatio = matchedWords.length / expectedWords.length;

  if (matchRatio > 0.5) {
    result.status = "pass";
    result.details =
      `Recognized "${speechResult.text}" ` + `(${(matchRatio * 100).toFixed(0)}% keyword match).`;
    return;
  }

  result.status = "fail";
  result.details = `Expected "${sample.expected.text}" but got "${speechResult.text}".`;
}

async function runSampleTest(sample, fetchImpl, visionApi, speechApi) {
  const result = createPendingResult(sample);

  try {
    const blob = await fetchSampleBlob(fetchImpl, sample.file);
    const base64 = await blobToBase64(blob);

    if (sample.type === "face-detection") {
      await runFaceDetectionTest(result, sample, base64, visionApi);
      return result;
    }

    if (sample.type === "ocr") {
      await runOcrTest(result, sample, base64, visionApi);
      return result;
    }

    if (sample.type === "barcode") {
      await runBarcodeTest(result, sample, base64, visionApi);
      return result;
    }

    if (sample.type === "image-classification") {
      await runClassificationTest(result, sample, base64, visionApi);
      return result;
    }

    if (sample.type === "speech-recognition") {
      await runSpeechRecognitionTest(result, sample, base64, speechApi);
      return result;
    }

    result.status = "skipped";
    result.details = `Unknown test type: ${sample.type}.`;
    return result;
  } catch (error) {
    result.status = "error";
    result.details = getErrorMessage(error);
    return result;
  }
}

/**
 * Run the sample-based demo test suite and report progress as each item completes.
 */
export async function runDemoTestSuite({ fetchImpl, visionApi, speechApi, onProgress = () => {} }) {
  const response = await fetchImpl("/samples/manifest.json");

  if (!response.ok) {
    throw new Error("Failed to load manifest.json");
  }

  const manifest = await response.json();
  const samples = manifest.samples || [];
  const results = [];

  onProgress({ current: 0, total: samples.length });

  for (const [index, sample] of samples.entries()) {
    onProgress({ current: index + 1, total: samples.length });
    results.push(await runSampleTest(sample, fetchImpl, visionApi, speechApi));
  }

  return results;
}
