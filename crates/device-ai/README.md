# device-ai

Cross-platform access to native, on-device AI APIs for Rust.

`device-ai` provides a high-level, idiomatic Rust interface to platform-native AI capabilities on macOS and Windows. By leveraging the APIs already built into the operating system (like Apple's Vision and Speech frameworks or Windows Media and ML APIs), you can add powerful AI features to your applications without the overhead of heavy models or external cloud dependencies.

## Key Features

* **Vision:** OCR (text recognition), barcode detection, face detection, and image classification.
* **Speech:** Speech recognition (speech-to-text) and speech synthesis (text-to-speech).
* **Text:** Language identification and translation.
* **LLM:** On-device language model generation, summarization, and rewriting.

## Platform Support

| Feature | macOS | Windows | Linux |
|---------|-------|---------|-------|
| Vision  | ✅    | ✅ (OCR) | -     |
| Speech  | ✅    | ✅      | -     |
| Text    | ✅    | -       | -     |
| LLM     | ✅    | (Stubs) | -     |

*Note: Some features are still in development or have platform-specific limitations.*

## Quick Start

Add `device-ai` to your `Cargo.toml`:

```toml
[dependencies]
device-ai = { git = "https://github.com/hypothesi/tauri-plugin-device-ai-apis" }
```

```rust
use device_ai::{DeviceAi, ImageSource, OcrOptions};

fn main() -> device_ai::Result<()> {
   let ai = DeviceAi::new();

   // Check what's available on this platform
   let caps = ai.capabilities();
   println!("speech recognition: {}", caps.speech_recognition.available);
   println!("OCR: {}", caps.text_recognition.available);

   // Run OCR on an image
   let result = ai.vision().recognize_text(
      ImageSource::from_path("receipt.png"),
      OcrOptions::new(),
   )?;

   println!("recognized: {}", result.text);
   Ok(())
}
```

## Feature Flags

Enable or disable individual capabilities to minimize dependencies and binary size:

| Feature | Description | Default |
|---------|-------------|---------|
| `speech` | Speech recognition and speech synthesis | Yes |
| `vision` | OCR, barcode detection, face detection, image classification | Yes |
| `text` | Language identification | Yes |
| `llm` | On-device language model | Yes |

## Current Limitations

* **Streaming Speech:** Native streaming speech recognition is not yet implemented.
* **Translation:** Currently returns `FEATURE_NOT_AVAILABLE`.
* **Windows Synthesis:** Text-to-speech synthesizes but does not yet play audio directly.
* **Windows LLM:** APIs are currently stubs awaiting Phi Silica bindings.
* **Apple Intelligence:** LLM support requires macOS 15.1+ and the FoundationModels SDK.

## Local Development & Verification

You can test the library's capabilities on your machine using the bundled example CLI:

```bash
# General
cargo run -p device-ai --example device-ai -- capabilities

# Speech
cargo run -p device-ai --example device-ai -- speech-voices
cargo run -p device-ai --example device-ai -- speech-speak "Hello from device-ai"

# Vision
cargo run -p device-ai --example device-ai -- vision-ocr ./path/to/image.png
cargo run -p device-ai --example device-ai -- vision-faces ./path/to/image.png

# LLM
cargo run -p device-ai --example device-ai -- llm-availability
cargo run -p device-ai --example device-ai -- llm-generate "Explain Rust in one sentence."
```

---

*This crate is maintained as part of the [tauri-plugin-device-ai-apis](https://github.com/hypothesi/tauri-plugin-device-ai-apis) project. It serves as the core Rust implementation for the Tauri plugin but can be used as a standalone library in any Rust project.*
