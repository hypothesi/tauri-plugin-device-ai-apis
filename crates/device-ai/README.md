# device-ai

`device-ai` is the Tauri-agnostic Rust library extracted from this repository. It owns the
reusable desktop-native implementation that the root Tauri plugin uses on macOS and
Windows.

iOS and Android still route through the root plugin's Swift/Kotlin mobile bridge, so
direct `device-ai` use is currently focused on desktop Rust consumers.

## What it provides

- Speech recognition and speech synthesis
- OCR, barcode detection, face detection, and image classification
- Language identification
- On-device LLM access where the target platform exposes it

## Current limitations

- Native streaming speech recognition is not implemented yet
- Translation currently returns `FEATURE_NOT_AVAILABLE`
- Windows text-to-speech synthesizes but does not play audio yet
- Windows LLM APIs are stubs until Rust bindings exist for Phi Silica
- Apple LLM support requires the FoundationModels SDK/runtime

## Quick start

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

    let result = ai.vision().recognize_text(
        ImageSource::from_path("receipt.png"),
        OcrOptions::new(),
    )?;

    println!("{}", result.text);
    Ok(())
}
```

## Direct verification

Use the bundled example CLI to exercise the library directly on desktop:

```bash
cargo run -p device-ai --example device-ai -- capabilities
cargo run -p device-ai --example device-ai -- speech-voices
cargo run -p device-ai --example device-ai -- vision-ocr ./path/to/image.png
```

Unsupported commands return explicit structured errors, which is the expected behavior for
unimplemented paths such as native speech streaming or translation.
