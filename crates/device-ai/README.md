# device-ai

`device-ai` is a Rust crate for calling the AI APIs built into macOS and Windows. Use it
in a desktop Rust application when you need OCR, speech, image analysis, language
identification, or local language-model APIs.

The operating system and device decide what is available. Check capabilities before
showing a feature or calling its API.

## Requirements

- Rust 1.77.2 or later.
- A macOS or Windows target for direct API calls.
- The relevant operating-system permission when an API requires one, such as microphone
  access for speech recognition.

On Linux and other unsupported targets, API calls return `Error::FeatureNotAvailable`.

## Install

```toml
[dependencies]
device-ai = "0.1.1"
```

## Start with a capability check

Create one `DeviceAi` value, inspect the capability you need, then make the call. This
keeps platform-specific decisions in one place.

```rust
use device_ai::{DeviceAi, ImageSource, OcrOptions};

fn main() -> device_ai::Result<()> {
    let ai = DeviceAi::new();

    if !ai.capabilities().text_recognition.available {
        eprintln!("OCR is unavailable on this device.");
        return Ok(());
    }

    let result = ai.vision().recognize_text(
        ImageSource::from_path("receipt.png"),
        OcrOptions::new().with_language("en-US"),
    )?;

    println!("{}", result.text);
    Ok(())
}
```

`ImageSource` accepts file paths, image bytes, and base64-encoded data. OCR results
include the complete text plus blocks, lines, confidence values when the platform
provides them, and normalized bounding boxes.

## APIs and platform support

| API | macOS | Windows | Notes |
| --- | --- | --- | --- |
| Speech recognition | Yes | Yes | One-shot recognition. macOS may request microphone permission. |
| Speech synthesis and voices | Yes | Yes | Windows completes synthesis but does not play the generated audio. |
| OCR | Yes | Yes | Access through `ai.vision().recognize_text()`. |
| Barcode and QR detection | Yes | No | Access through `ai.vision().detect_barcodes()`. |
| Face detection | Yes | No | Supports optional landmarks and attributes. |
| Image classification | Yes | No | Returns labels and confidence values. |
| Language identification | Yes | No | Access through `ai.text().identify_language()`. |
| Translation | No | No | `ai.text().translate()` returns `Error::FeatureNotAvailable`. |
| Local language model | Conditional | No | See the language-model section. |

The capability response also reports whether a feature runs on device and whether it
requires permission. Treat it as the source of truth for the current machine.

## Speech

Pass an `AudioSource` through `RecognitionOptions` for a file or the microphone. Use
`voices()` before selecting a synthesis voice.

```rust
use device_ai::{AudioSource, DeviceAi, RecognitionOptions, SynthesisOptions};

fn recognize_file() -> device_ai::Result<()> {
    let ai = DeviceAi::new();
    let options = RecognitionOptions::new()
        .with_audio_source(AudioSource::from_path("meeting.wav"));
    let result = ai.speech().recognize(options)?;

    println!("{}", result.text);
    ai.speech().speak("Transcription complete.", SynthesisOptions::new())?;
    Ok(())
}
```

Streaming speech recognition is not implemented. `start_recognition()` and
`stop_recognition()` return an error; use `recognize()` for one-shot recognition.

## Vision

The vision client handles OCR, barcode detection, face detection, and image
classification. Barcode detection, face detection, and image classification are
available on macOS.

```rust
use device_ai::{BarcodeOptions, DeviceAi, ImageSource};

fn scan_code() -> device_ai::Result<()> {
    let ai = DeviceAi::new();
    let codes = ai.vision().detect_barcodes(
        ImageSource::from_path("shipping-label.png"),
        BarcodeOptions::new(),
    )?;

    for code in codes {
        println!("{:?}: {}", code.format, code.raw_value);
    }

    Ok(())
}
```

## Text and language models

Language identification is available on macOS. Translation is declared in the API but
does not have an implementation yet.

Language-model methods cover availability checks, generation, streaming, sessions,
summaries, and rewrites. The macOS implementation requires the FoundationModels SDK;
call `check_availability()` before using another language-model method. Windows reports
the language model as unavailable.

```rust
use device_ai::{DeviceAi, LlmGenerateOptions};

fn generate_summary() -> device_ai::Result<()> {
    let ai = DeviceAi::new();
    let availability = ai.llm().check_availability()?;

    if !availability.available {
        eprintln!("Local language model unavailable: {}",
            availability.reason.as_deref().unwrap_or("unknown reason"));
        return Ok(());
    }

    let result = ai.llm().generate(
        LlmGenerateOptions::new("Summarize this paragraph in one sentence.")
            .max_tokens(80),
    )?;

    println!("{}", result.content);
    Ok(())
}
```

## Feature flags

`speech`, `vision`, `text`, and `llm` are enabled by default. In version 0.1.1,
these flags do not change which API groups compile or which platform dependencies Cargo
selects. Leave the default feature set enabled.

## Verify locally

The crate includes a small command-line example for exercising the APIs on a real
device. Run it from the crate directory:

```bash
cargo run --example device-ai -- capabilities
cargo run --example device-ai -- vision-ocr ./receipt.png
cargo run --example device-ai -- speech-voices
```

Run the crate checks before changing or publishing code:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets --all-features
```

## Usage

- [Tauri plugin: Device AI APIs](https://github.com/hypothesi/tauri-plugin-device-ai-apis)
  exposes these capabilities to a Tauri application.
