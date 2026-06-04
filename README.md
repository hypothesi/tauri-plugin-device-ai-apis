# Tauri Plugin: Device AI APIs

A Tauri v2 plugin for device-native AI capabilities, plus a reusable `device-ai`
Rust crate for direct desktop use. The repository covers speech recognition,
text-to-speech, OCR, barcode detection, face detection, image classification,
language identification, and conditional on-device LLM access.

## Features

| Feature                 | iOS | Android | macOS | Windows  
| ----------------------- | --- | ------- | ----- | ------- | 
| Language Model (LLM)    | ✅† | ❌      | ✅†   | ❌‡      |
| Speech Recognition      | ✅  | ✅      | ✅    | ✅       |
| Text-to-Speech          | ✅  | ✅      | ✅    | ✅\*     |
| Text Recognition (OCR)  | ✅  | ✅      | ✅    | ✅       |
| Barcode/QR Detection    | ✅  | ✅      | ✅    | ❌       |
| Face Detection          | ✅  | ✅      | ✅    | ❌       |
| Image Classification    | ✅  | ✅      | ✅    | ❌       |
| Language Identification | ✅  | ✅      | ✅    | ❌       |

Legend: ✅ Implemented | ❌ Not Available (yet)

\* Windows TTS completes synthesis, but the current Rust backend does not yet play the
generated stream.

† Requires macOS 26+ (Tahoe) or iOS 26+ with Apple FoundationModels. The feature
compiles conditionally — if the SDK is not present, LLM commands return a "not available"
error gracefully.

‡ Windows Phi Silica APIs (`Microsoft.Windows.AI.Text`) are not yet accessible from Rust
via the `windows` crate. Stubs return a clear "not available" error. Full support is
planned once WinRT bindings are available.

Linux is not currently supported due to the lack of a unified set of native AI
APIs across distributions, but contributions to expand platform coverage are
welcome.

## Installation

### Rust

Add the plugin to your Tauri project's `Cargo.toml`:

```toml
[dependencies]
tauri-plugin-device-ai-apis = { git = "https://github.com/hypothesi/tauri-plugin-device-ai-apis" }
```

Register the plugin in your `lib.rs`:

```rust
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_device_ai_apis::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### Rust library

Use the extracted `device-ai` crate when you want direct Rust access without Tauri on
desktop. The root crate remains the Tauri plugin adapter, and iOS/Android still route
through the plugin-hosted mobile bridge:

```toml
[dependencies]
device-ai = { git = "https://github.com/hypothesi/tauri-plugin-device-ai-apis" }
```

```rust
use device_ai::{DeviceAi, ImageSource, OcrOptions};

fn main() -> device_ai::Result<()> {
    let ai = DeviceAi::new();
    let capabilities = ai.capabilities();

    println!("speech recognition: {}", capabilities.speech_recognition.available);

    let ocr = ai.vision().recognize_text(
        ImageSource::from_path("receipt.png"),
        OcrOptions::new(),
    )?;

    println!("{}", ocr.text);
    Ok(())
}
```

### JavaScript/TypeScript

Install the API package:

```bash
npm install @hypothesi/tauri-plugin-device-ai-apis
```

## Usage

### Language model (LLM)

```typescript
import { llm } from "@hypothesi/tauri-plugin-device-ai-apis";

// Check availability
const status = await llm.checkAvailability();
if (!status.available) {
  console.log("LLM not available:", status.reason);
}

// Single-shot generation
const result = await llm.generate({
  prompt: "Explain quantum computing in one paragraph.",
  temperature: 0.7,
  maxTokens: 256,
});
console.log(result.content);

// Streaming generation
let streamed = "";
await llm.generateStream(
  { prompt: "Write a short poem about the sea." },
  (event) => {
    if (event.type === "delta") streamed += event.content;
    if (event.type === "done") {
      console.log(streamed || event.content);
      console.log("Done:", event.finishReason);
    }
  },
);

// Multi-turn session
const sessionId = await llm.createSession({
  systemPrompt: "You are a helpful assistant.",
});
const reply = await llm.sessionSend(sessionId, "What is 2+2?");
console.log(reply.content);
await llm.destroySession(sessionId);

// Text intelligence
const summary = await llm.summarize({
  text: "Long article text here...",
});
console.log(summary.summary);

const rewritten = await llm.rewrite({
  text: "hey wanna grab lunch tmrw?",
  tone: "formal",
});
console.log(rewritten.rewrittenText);
```

### Capability Detection

Check which features are available on the current device:

```typescript
import { getCapabilities } from "@hypothesi/tauri-plugin-device-ai-apis";

const capabilities = await getCapabilities();
if (capabilities.speechRecognition.available) {
  console.log("Speech recognition is available!");
}
```

### Speech Recognition

Convert speech to text:

```typescript
import { speech } from "@hypothesi/tauri-plugin-device-ai-apis";

// One-shot recognition
const result = await speech.recognize({ language: "en-US" });
console.log("Recognized:", result.text);
console.log("Confidence:", result.confidence);
```

### Text-to-Speech

Synthesize speech from text:

```typescript
import { speech } from "@hypothesi/tauri-plugin-device-ai-apis";

// Speak text
await speech.synthesize("Hello, world!", {
  rate: 1.0,
  pitch: 1.0,
  volume: 1.0,
});

// List available voices
const voices = await speech.getVoices();
console.log("Available voices:", voices);

// Use a specific voice
await speech.synthesize("Hello!", { voice: voices[0].id });
```

### Text Recognition (OCR)

Extract text from images:

```typescript
import { vision } from "@hypothesi/tauri-plugin-device-ai-apis";

// From base64 image data
const result = await vision.recognizeText({ base64: imageData });
console.log("Extracted text:", result.text);

// Access individual text blocks
for (const block of result.blocks) {
  console.log("Block:", block.text, "at", block.boundingBox);
}
```

### Barcode Detection

Detect and decode barcodes and QR codes:

```typescript
import { vision } from "@hypothesi/tauri-plugin-device-ai-apis";

const barcodes = await vision.detectBarcodes({ base64: imageData });
for (const barcode of barcodes) {
  console.log(`${barcode.format}: ${barcode.rawValue}`);
}
```

### Face Detection

Detect faces with optional landmarks:

```typescript
import { vision } from "@hypothesi/tauri-plugin-device-ai-apis";

const faces = await vision.detectFaces(
  { base64: imageData },
  { detectLandmarks: true, classifyAttributes: true },
);

for (const face of faces) {
  console.log("Face at:", face.boundingBox);
  if (face.landmarks) {
    console.log("Left eye:", face.landmarks.leftEye);
  }
  if (face.attributes) {
    console.log("Smiling:", face.attributes.smilingProbability);
  }
}
```

### Image Classification

Classify images with labels:

```typescript
import { vision } from "@hypothesi/tauri-plugin-device-ai-apis";

const classifications = await vision.classifyImage(
  { base64: imageData },
  { maxResults: 5, minConfidence: 0.5 },
);

for (const classification of classifications) {
  console.log(`${classification.identifier}: ${classification.confidence}`);
}
```

### Language Identification

Identify the language of text:

```typescript
import { text } from "@hypothesi/tauri-plugin-device-ai-apis";

const result = await text.identifyLanguage("Bonjour, comment allez-vous?");
console.log("Language:", result.language); // 'fr'
console.log("Confidence:", result.confidence);
```

## Permissions

Add permissions in your app's capability file (e.g.
`src-tauri/capabilities/default.json`).

Grant all plugin permissions:

```json
{
  "permissions": ["core:default", "device-ai-apis:all"]
}
```

Or use granular permission sets:

```json
{
  "permissions": [
    "core:default",
    "device-ai-apis:allow-get-capabilities",
    "device-ai-apis:speech-recognition",
    "device-ai-apis:speech-synthesis",
    "device-ai-apis:vision-all",
    "device-ai-apis:text-all"
  ]
}
```

Individual permissions are also available:

| Permission                              | Description                        |
| --------------------------------------- | ---------------------------------- |
| `allow-get-capabilities`                | Query available AI features        |
| `allow-speech-recognize`                | One-shot speech recognition        |
| `allow-speech-recognize-start`          | Start streaming recognition        |
| `allow-speech-recognize-stop`           | Stop streaming recognition         |
| `allow-speech-synthesize`              | Text-to-speech synthesis           |
| `allow-speech-get-voices`              | List available TTS voices          |
| `allow-vision-recognize-text`          | OCR text recognition               |
| `allow-vision-detect-barcodes`         | Barcode and QR code detection      |
| `allow-vision-detect-faces`            | Face detection                     |
| `allow-vision-classify-image`          | Image classification               |
| `allow-text-identify-language`         | Language identification            |
| `allow-text-translate`                 | Text translation                   |
| `allow-llm-check-availability`        | Check on-device LLM availability   |
| `allow-llm-get-model-info`            | Get language model metadata        |
| `allow-llm-generate`                  | Single-shot text generation        |
| `allow-llm-generate-stream`           | Streaming text generation          |
| `allow-llm-create-session`            | Create multi-turn session          |
| `allow-llm-session-send`              | Send message in a session          |
| `allow-llm-session-send-stream`       | Stream response in a session       |
| `allow-llm-destroy-session`           | Destroy a session                  |
| `allow-llm-summarize`                 | Summarize text                     |
| `allow-llm-rewrite`                   | Rewrite text with a given tone     |

Permission sets: `speech-recognition`, `speech-synthesis`, `vision-all`, `text-all`,
`llm-all`, and `all` (everything).

## Platform Requirements

### iOS

- iOS 13.0+
- Add to `Info.plist`:
  ```xml
  <key>NSSpeechRecognitionUsageDescription</key>
  <string>Speech recognition is used for voice commands</string>
  <key>NSMicrophoneUsageDescription</key>
  <string>Microphone access is needed for speech recognition</string>
  ```

### Android

- Android API 21+
- Permissions are declared in the plugin's `AndroidManifest.xml`
- Runtime permission request for `RECORD_AUDIO` is handled automatically

### macOS/Windows

- macOS and Windows desktop support is available (feature coverage varies by API).

## Architecture

The repository now has two Rust entry points:

- `tauri-plugin-device-ai-apis`: Tauri plugin wiring, permissions, commands, and mobile host
  bridging
- `crates/device-ai`: Tauri-agnostic Rust library that owns the reusable desktop-native
  implementation used by the plugin on macOS and Windows

On desktop, plugin commands delegate into `device-ai`. On iOS/Android, commands still
flow through the Swift/Kotlin mobile bridge in this repository.

The native backends use:

- **iOS/macOS**: Speech framework, AVFoundation, Vision framework
- **Android**: SpeechRecognizer, TextToSpeech, ML Kit
- **Windows**: Windows.Media.SpeechRecognition, Windows.Media.SpeechSynthesis, Windows.Media.Ocr

Most native features use local platform APIs, but there are important exceptions and
current gaps:

- Android speech recognition and browser speech fallbacks may rely on platform/browser
  services rather than strictly on-device inference
- Native streaming speech recognition (`speech_recognize_start` / `speech_recognize_stop`)
  is not implemented yet
- Translation commands exist in the API surface but currently return "not available"
- Windows text-to-speech synthesizes successfully but does not yet play audio

## Manual verification

### Direct library verification

Run the desktop example CLI in `crates/device-ai/examples/device-ai.rs`:

```bash
cargo run -p device-ai --example device-ai -- capabilities
cargo run -p device-ai --example device-ai -- speech-voices
cargo run -p device-ai --example device-ai -- speech-speak "Hello from device-ai"
cargo run -p device-ai --example device-ai -- speech-recognize ./path/to/audio.wav
cargo run -p device-ai --example device-ai -- speech-stream
cargo run -p device-ai --example device-ai -- vision-ocr ./path/to/image.png
cargo run -p device-ai --example device-ai -- vision-barcodes ./path/to/image.png
cargo run -p device-ai --example device-ai -- vision-faces ./path/to/image.png
cargo run -p device-ai --example device-ai -- vision-classify ./path/to/image.png
cargo run -p device-ai --example device-ai -- text-language "Bonjour tout le monde"
cargo run -p device-ai --example device-ai -- text-translate en es "hello world"
cargo run -p device-ai --example device-ai -- llm-availability
cargo run -p device-ai --example device-ai -- llm-model-info
cargo run -p device-ai --example device-ai -- llm-generate "Explain OCR in one paragraph."
cargo run -p device-ai --example device-ai -- llm-stream "Write a haiku about Rust."
cargo run -p device-ai --example device-ai -- llm-session "What is 2 + 2?"
cargo run -p device-ai --example device-ai -- llm-session-stream "Describe this platform."
cargo run -p device-ai --example device-ai -- llm-summarize "Long text to summarize..."
cargo run -p device-ai --example device-ai -- llm-rewrite formal "hey wanna meet tomorrow?"
```

Use local audio/image assets that make sense for your host platform. This example is aimed
at the direct desktop library (`device-ai` on macOS/Windows). Unsupported features should
return explicit structured errors; for example, `speech-stream` is currently expected to
report that native streaming recognition is not implemented.

### Plugin + sample app verification

```bash
npm run build
cd examples/tauri-app
npm run tauri dev
```

Use the sample app to exercise capabilities, speech, vision, language identification, and
LLM flows end-to-end through the plugin surface.

## Error Handling

All API calls can throw errors with structured error codes:

```typescript
import { DeviceAiError } from "@hypothesi/tauri-plugin-device-ai-apis";

try {
  await speech.recognize();
} catch (error) {
  if ((error as DeviceAiError).code === "FEATURE_NOT_AVAILABLE") {
    console.log("Speech recognition not available on this device");
  } else if ((error as DeviceAiError).code === "PERMISSION_DENIED") {
    console.log("Permission was denied");
  }
}
```

## License

MIT
