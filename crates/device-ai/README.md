# device-ai

`device-ai` is a Rust crate for calling the AI APIs built into macOS and Windows. Use it
in a desktop Rust application when you need OCR, speech, image analysis, language
identification, or local language-model APIs.

The operating system and device decide what is available. Check capabilities before
showing a feature or calling its API.

## Requirements

- **Vision:** OCR (text recognition), barcode detection, face detection, and image classification.
- **Speech:** Speech recognition (speech-to-text) and speech synthesis (text-to-speech).
- **Text:** Language identification and translation.
- **LLM:** On-device language model generation, summarization, and rewriting.

On Linux and other unsupported targets, API calls return `Error::FeatureNotAvailable`.

| Feature | macOS | Windows  | Linux |
| ------- | ----- | -------- | ----- |
| Vision  | ✅    | ✅ (OCR) | -     |
| Speech  | ✅    | ✅       | -     |
| Text    | ✅    | -        | -     |
| LLM     | ✅    | (Stubs)  | -     |

_Note: Some features are still in development or have platform-specific limitations._

## Requirements

- **macOS:** Xcode Command Line Tools are required (`xcode-select --install`).

## Quick Start

Add `device-ai` to your `Cargo.toml`:

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

| Feature  | Description                                                  | Default |
| -------- | ------------------------------------------------------------ | ------- |
| `speech` | Speech recognition and speech synthesis                      | Yes     |
| `vision` | OCR, barcode detection, face detection, image classification | Yes     |
| `text`   | Language identification                                      | Yes     |
| `llm`    | On-device language model                                     | Yes     |

The capability response also reports whether a feature runs on device and whether it
requires permission. Treat it as the source of truth for the current machine.

- **Streaming Speech:** Native streaming speech recognition is not yet implemented.
- **Translation:** Currently returns `FEATURE_NOT_AVAILABLE`.
- **Windows Synthesis:** Text-to-speech synthesizes but does not yet play audio directly.
- **Windows LLM:** APIs are currently stubs awaiting Phi Silica bindings.
- **Apple Intelligence:** LLM support requires macOS 15.1+ and the FoundationModels SDK.

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

_This crate is maintained as part of the [tauri-plugin-device-ai-apis](https://github.com/hypothesi/tauri-plugin-device-ai-apis) project. It serves as the core Rust implementation for the Tauri plugin but can be used as a standalone library in any Rust project._
